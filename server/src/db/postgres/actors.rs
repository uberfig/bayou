use bayou_protocol::{
    cryptography::{key::Key, openssl::OpenSSLPublic},
    types::activitystream_objects::{
        actors::{Actor, ActorType},
        public_key::ApPublicKey,
    },
};
use tokio_postgres::Row;
use url::Url;

use crate::db::conn::EntityOrigin;

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
    let followers = followers.map(|followers| Url::parse(followers).expect("invalid followers stored in db"));
    let following: Option<&str> = result.get("following");
    let following = following.map(|following| Url::parse(following).expect("invalid following stored in db"));

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

// pub async fn get_local_user_actor(
//     conn: &PgConn,
//     preferred_username: &str,
//     instance_domain: &str,
// ) -> Option<(Actor, i64)> {
//     let client = conn.db.get().await.expect("failed to get client");
//     let stmt = r#"
//         SELECT * FROM internal_users NATURAL JOIN unified_users WHERE preferred_username = $1;
//         "#;
//     let stmt = client.prepare(stmt).await.unwrap();

//     let result = client
//         .query(&stmt, &[&preferred_username])
//         .await
//         .expect("failed to get local user")
//         .pop();

//     let result = match result {
//         Some(x) => x,
//         None => return None,
//     };
//     let id: i64 = result.get("uid");

//     Some((local_user_from_row(result, instance_domain), id))
// }

// pub async fn get_actor(conn: &PgConn, uid: i64, instance_domain: &str) -> Option<Actor> {
//     println!("{}", uid);
//     let mut client = conn.db.get().await.expect("failed to get client");
//     let transaction = client
//         .transaction()
//         .await
//         .expect("failed to begin transaction");

//     //LEFT OUTER JOIN internal_users ON unified_users.local_id = internal_users.local_id
//     //LEFT OUTER JOIN federated_ap_users ON unified_users.fedi_id = federated_ap_users.fedi_id

//     let stmt = r#"
//         SELECT * FROM unified_users
//         WHERE uid = $1;
//         "#;
//     let stmt = transaction.prepare(stmt).await.unwrap();

//     let result = transaction
//         .query(&stmt, &[&uid])
//         .await
//         .expect("failed to get actor")
//         .pop();

//     let result = match result {
//         Some(x) => x,
//         None => return None,
//     };

//     let is_local: bool = result.get("is_local");

//     match is_local {
//         true => {
//             let local_id: i64 = result.get("local_id");
//             let stmt = r#"
//             SELECT * FROM internal_users
//             WHERE local_id = $1;
//             "#;
//             let stmt = transaction.prepare(stmt).await.unwrap();

//             let result = transaction
//                 .query(&stmt, &[&local_id])
//                 .await
//                 .expect("failed to get actor")
//                 .pop()
//                 .unwrap();
//             transaction.commit().await.expect("failed to commit");

//             Some(local_user_from_row(result, instance_domain))
//         }
//         false => {
//             let fedi_id: i64 = result.get("fedi_id");
//             let stmt = r#"
//             SELECT * FROM federated_ap_users
//             WHERE fedi_id = $1;
//             "#;
//             let stmt = transaction.prepare(stmt).await.unwrap();

//             let result = transaction
//                 .query(&stmt, &[&fedi_id])
//                 .await
//                 .expect("failed to get actor")
//                 .pop()
//                 .unwrap();

//             transaction.commit().await.expect("failed to commit");

//             Some(actor_from_row(result))
//         }
//     }
// }

// pub async fn create_federated_actor(conn: &PgConn, actor: &Actor) -> i64 {
//     let mut client = conn.db.get().await.expect("failed to get client");
//     let transaction = client
//         .transaction()
//         .await
//         .expect("failed to begin transaction");

//     let stmt = r#"
//         SELECT * FROM unified_users NATURAL JOIN federated_ap_users WHERE id = $1;
//         "#;
//     let stmt = transaction.prepare(stmt).await.unwrap();

//     let result = transaction
//         .query(&stmt, &[&actor.id.as_str()])
//         .await
//         .expect("failed to get actor")
//         .pop();

//     //user already exists
//     if let Some(x) = result {
//         return x.get("uid");
//     }

//     let stmt = r#"
//         INSERT INTO federated_ap_users
//         (
//             id, type_field, preferred_username, domain,
//             name, summary, url,
//             public_key_pem, public_key_id,
//             inbox, outbox, followers, following
//         )
//         VALUES
//         (
//             $1, $2, $3, $4,
//             $5, $6, $7,
//             $8, $9,
//             $10, $11, $12, $13
//         )
//         RETURNING fedi_id;
//         "#;
//     let stmt = transaction.prepare(stmt).await.unwrap();

//     let domain = actor.id.domain().unwrap();
//     let url = actor.url.as_ref().map(|url| url.as_str());
//     let fedi_id: i64 = transaction
//         .query(
//             &stmt,
//             &[
//                 &actor.id.as_str(),
//                 &serde_json::to_string(&actor.type_field).unwrap(),
//                 &actor.preferred_username,
//                 &domain,
//                 &actor.name,
//                 &actor.summary,
//                 &url,
//                 &actor.public_key.public_key_pem.to_pem().unwrap(),
//                 &actor.public_key.id.as_str(),
//                 &actor.inbox.as_str(),
//                 &actor.outbox.as_str(),
//                 &actor.followers.as_str(),
//                 &actor.following.as_str(),
//             ],
//         )
//         .await
//         .expect("failed to insert user")
//         .pop()
//         .expect("did not return fedi_id")
//         .get("fedi_id");

//     let stmt = r#"
//         INSERT INTO unified_users
//         (
//             is_local, fedi_id
//         )
//         VALUES
//         (
//             $1, $2
//         )
//         RETURNING uid;
//         "#;
//     let stmt = transaction.prepare(stmt).await.unwrap();

//     let uid: i64 = transaction
//         .query(&stmt, &[&false, &fedi_id])
//         .await
//         .expect("failed to insert user")
//         .pop()
//         .expect("did not return uid")
//         .get("uid");

//     //update to have the new uid
//     let stmt = r#"
//         UPDATE federated_ap_users
//         SET uid = $1
//         WHERE fedi_id = $2;
//     "#;
//     let stmt = transaction.prepare(stmt).await.unwrap();

//     let _ = transaction
//         .query(&stmt, &[&uid, &fedi_id])
//         .await
//         .expect("failed to update fedi user");

//     transaction.commit().await.expect("failed to commit");

//     uid
// }

// pub async fn get_federated_db_id(conn: &PgConn, actor_id: &str) -> Option<i64> {
//     let client = conn.db.get().await.expect("failed to get client");
//     let stmt = r#"
//     SELECT * FROM unified_users NATURAL JOIN federated_ap_users WHERE id = $1;
//     "#;
//     let stmt = client.prepare(stmt).await.unwrap();

//     client
//         .query(&stmt, &[&actor_id])
//         .await
//         .expect("failed to get federated user uid")
//         .pop()
//         .map(|x| x.get("uid"))
// }
// pub async fn get_local_user_db_id(conn: &PgConn, preferred_username: &str) -> Option<i64> {
//     let client = conn.db.get().await.expect("failed to get client");
//     let stmt = r#"
//     SELECT * FROM unified_users NATURAL JOIN internal_users WHERE preferred_username = $1;
//     "#;
//     let stmt = client.prepare(stmt).await.unwrap();

//     let result = client
//         .query(&stmt, &[&preferred_username])
//         .await
//         .expect("failed to get local user")
//         .pop();
//     result.map(|x| x.get("uid"))
// }
