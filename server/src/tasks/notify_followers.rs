use actix_web::web::Data;
use bayou_protocol::{
    cryptography::key::Algorithms,
    protocol::ap_protocol::fetch::ap_post,
};

use crate::db::{
    conn::{Conn, EntityOrigin},
    dbconn::DbConn,
    utility::instance_actor::InstanceActor,
};

pub async fn notify_followers(
    conn: Data<DbConn>,
    post_id: &str,
    origin: EntityOrigin<'_>,
    algorithm: Algorithms,
) {
    let Some(ap_rep) = conn
        .get_ap_post(post_id, &origin)
        .await
        .map(|post| post.wrap_context())
    else {
        return;
    };
    let uuid = conn.get_uuid_url(ap_rep.item.actor()).await;
    let actor = conn.get_instance_actor(algorithm).await;
    let keyid = InstanceActor::get_key_id(
        ap_rep
            .item
            .actor()
            .domain()
            .expect("somehow a local user does not have a domain"),
    );
    let mut key = actor.get_private_key();

    let Ok(followers) = conn.get_follower_inboxes(uuid).await else {
        return;
    };
    let ap_rep = serde_json::to_string(&ap_rep).expect("failed to serialize content from the db");

    let ap_digest = algorithm.hash(ap_rep.as_bytes());

    for follower in followers {
        match follower.protocol {
            bayou_protocol::protocol::protocols::Protocols::Activitypub => {
                let _ = ap_post(
                    follower.inbox,
                    &ap_rep,
                    &ap_digest,
                    &keyid,
                    &mut key,
                    actor.algorithm,
                )
                .await;
            }
            bayou_protocol::protocol::protocols::Protocols::Versia => {
                todo!()
            }
        }
    }
}
