use actix_web::rt::spawn;
use bayou_protocol::{
    cryptography::{
        key::{Algorithms, Key},
        openssl::OpenSSLPublic,
    },
    protocol::{
        ap_protocol::fetch::authorized_fetch, webfinger::{RelTypes, RelWrap}, webfinger_resolve::webfinger_resolve
    },
    types::activitystream_objects::{
        actors::{Actor, ActorType},
        context::ContextWrap,
        public_key::ApPublicKey,
    },
};
use tokio_postgres::Row;
use url::Url;
use uuid::Uuid;

use crate::db::{
    conn::{Conn, DbErr, EntityOrigin},
    utility::instance_actor::InstanceActor,
};

use super::pg_conn::PgConn;

fn actor_from_row(result: Row) -> Actor {
    let id: &str = result.get("resource_link");
    let id = Url::parse(&id).expect("invalid resource link stored in db");
    let type_field: String = result.get("type_field");
    let type_field: ActorType = serde_json::from_str(&type_field).expect("unkown actor type in db");
    let preferred_username: String = result.get("username");
    let name: Option<String> = result.get("display_name");
    let summary: Option<String> = result.get("summary");
    let url: Option<&str> = result.get("url");
    let url = url.map(|url| Url::parse(&url).unwrap());

    let inbox: &str = result.get("inbox");
    let inbox = Url::parse(inbox).expect("invalid inbox stored in db");
    let outbox: &str = result.get("outbox");
    let outbox = Url::parse(outbox).expect("invalid outbox stored in db");
    let followers: Option<&str> = result.get("followers");
    let followers =
        followers.map(|followers| Url::parse(followers).expect("invalid followers stored in db"));
    let following: Option<&str> = result.get("following");
    let following =
        following.map(|following| Url::parse(following).expect("invalid following stored in db"));

    let public_key_id: &str = result.get("public_key_id");
    let pem: &str = result.get("public_key_pem");
    let public_key = ApPublicKey {
        id: Url::parse(&public_key_id).unwrap(),
        owner: id.clone(),
        public_key_pem: OpenSSLPublic::from_pem(pem.as_bytes()).unwrap(),
    };

    // todo!()
    Actor {
        type_field,
        id,
        preferred_username,
        summary,
        name,
        url,
        public_key,
        inbox,
        outbox,
        followers,
        following,
        versia_url: None,
    }
}

pub async fn get_actor(conn: &PgConn, username: &str, origin: &EntityOrigin<'_>) -> Option<Actor> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        SELECT * FROM users NATURAL JOIN instances WHERE username = $1 AND domain = $2;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&username, &origin.inner()])
        .await
        .expect("failed to get local user")
        .pop();

    let row = match result {
        Some(x) => x,
        None => return None,
    };
    // uncomment to enforce is_authoratative
    // let is_authoratative: bool = row.get("is_authoratative");
    // if is_authoratative != origin.is_local() {
    //     return None;
    // }
    Some(actor_from_row(row))
}

async fn insert_actor(conn: &deadpool_postgres::Transaction<'_>, actor: &Actor) -> Result<Uuid, DbErr> {
    todo!()
}

pub async fn get_actor_backfilling(
    conn: &PgConn,
    username: &str,
    origin: &EntityOrigin<'_>,
    algorithm: Algorithms,
    // the domain if this instance, used for auth fetch
    instance_domain: &str,
) -> Option<Actor> {
    let mut client = conn.db.get().await.expect("failed to get client");
    let transaction = client
        .transaction()
        .await
        .expect("failed to begin transaction");

    let stmt = r#"
        SELECT * FROM users WHERE username = $1 AND domain = $2;
        "#;
    let stmt = transaction.prepare(stmt).await.unwrap();

    let result = transaction
        .query(&stmt, &[&username, &origin.inner()])
        .await
        .expect("failed to get actor")
        .pop();
    if let Some(row) = result {
        return Some(actor_from_row(row))
    }

    if conn.is_authoratative(origin.inner()).await {
        // we are authoratative over this domain and we don't
        // have this actor, it does not exist.
        return None;
    }

    if conn.backfill_domain(origin.inner()).await.is_none() {
        // we could not backfill the domain, the domain and by proxy, the user, does not exist
        return None;
    }

    let Ok(user_id) = webfinger_resolve(
        username,
        origin.inner(),
        RelWrap::Defined(RelTypes::RelSelf),
    )
    .await
    else {
        return None;
    };
    let instance_actor = conn.get_instance_actor(algorithm).await;
    let fetched: Result<ContextWrap<Actor>, _> = authorized_fetch(
        user_id,
        &InstanceActor::get_key_id(instance_domain),
        &mut instance_actor.get_private_key(),
        algorithm,
    )
    .await;
    let Ok(fetched) = fetched else { return None };
    let inserted = insert_actor(&transaction, &fetched.item).await.expect("failed to insert fetched actor");

    transaction.commit().await.expect("failed to commit inserting new actor from backfilling");

    let cloned = conn.clone();
    spawn(async move {
        cloned.backfill_actor(&inserted).await
    });

    // we don't just return fetched to ensure what this fn produces is identical to what get actor produces
    conn.get_actor(username, origin).await
}

