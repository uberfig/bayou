// use actix_web::rt::spawn;
// use serial_test::serial;
// use url::Url;

// use crate::{app::start_application, config::get_config};

// use super::utility::new_actor::NewLocal;

// #[actix_web::test]
// #[serial]
// #[ignore]
// async fn create_and_retrieve_user() -> Result<(), String> {
//     let config = get_config().unwrap();
//     let conn = config.create_conn();
//     conn.init().await.unwrap();

//     let preferred_username = "create_and_retrieve_user".to_string();

//     let uid = conn
//         .create_local_user(&NewLocal::new(
//             preferred_username.clone(),
//             "filler".to_string(),
//             None,
//             None,
//             None,
//         ))
//         .await
//         .unwrap();
//     let actor = conn.get_actor(uid, &config.instance_domain).await;
//     let Some(actor) = actor else {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err(
//             "failed to retrieve actor with get_actor, may have failed to insert".to_string(),
//         );
//     };
//     let second = conn
//         .get_local_user_actor(&preferred_username, &config.instance_domain)
//         .await;
//     let Some((second_actor, second_uid)) = second else {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err("failed to retrieve actor with get_local_user_actor".to_string());
//     };

//     if second_actor.id.ne(&actor.id) {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err("id's of the retrieved actors don't match".to_string());
//     }

//     if uid != second_uid {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err(
//             "get_local_user_actor uid doesn't match uid returned from inserting".to_string(),
//         );
//     }

//     conn.delete_actor(uid, None).await.unwrap();

//     Ok(())
// }

// #[actix_web::test]
// #[serial]
// #[ignore]
// async fn backfill_fedi_user() -> Result<(), String> {
//     let config = get_config().unwrap();
//     let conn = config.create_conn();
//     conn.init().await.unwrap();
//     let handle = spawn(start_application(config.clone()));

//     let uid = match conn
//         .load_new_federated_actor(
//             &Url::parse("https://mastodon.social/@ivy_test").unwrap(),
//             &config.instance_domain,
//         )
//         .await
//     {
//         Ok(x) => x,
//         Err(x) => return Err(format!("failed to load the federated actor: {}", x)),
//     };

//     let Some(actor) = conn.get_actor(uid, &config.instance_domain).await else {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err("failed to get backfilled fedi actor".to_string());
//     };

//     if actor.preferred_username.ne("ivy_test") {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err(format!(
//             "preferred uname doesn't match ivy_test value: {}",
//             actor.preferred_username
//         ));
//     }

//     conn.delete_actor(uid, None).await.unwrap();
//     handle.abort();
//     Ok(())
// }
