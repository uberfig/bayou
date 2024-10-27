// use actix_web::rt::spawn;
// use serial_test::serial;
// use url::Url;

// use crate::{
//     activitystream_objects::{actors::Actor, context::ContextWrap},
//     app::start_application,
//     config::get_config,
//     db::utility::{instance_actor::InstanceActor, new_actor::NewLocal},
//     protocol::ap_protocol::fetch::authorized_fetch,
// };

// #[actix_web::test]
// #[serial]
// #[ignore]
// async fn test_actor_endpoint() -> Result<(), String> {
//     let config = get_config().unwrap();
//     let conn = config.create_conn();
//     conn.init().await.unwrap();

//     let handle = spawn(start_application(config.clone()));

//     let preferred_username = "test_actor_endpoint".to_string();

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
//         return Err(
//             "failed to retrieve actor with get_actor, may have failed to insert".to_string(),
//         );
//     };

//     // dbg!(&actor);
//     println!("actor id: {}", actor.id);

//     let instance_actor = conn.get_instance_actor().await;

//     let local_url = &Url::parse(&format!(
//         "http://{}:{}/users/{}",
//         config.bind_address, config.port, actor.preferred_username
//     ))
//     .unwrap();
//     println!("local url: {}", &local_url);
//     let result: Result<ContextWrap<Actor>, crate::protocol::errors::FetchErr> = authorized_fetch(
//         local_url,
//         &InstanceActor::pub_key_id(&config.instance_domain),
//         &mut instance_actor.get_private_key(),
//     )
//     .await;
//     if result.is_err() {
//         conn.delete_actor(uid, None).await.unwrap();
//         return Err(format!(
//             "failed to retrieve the actor from the local address {}",
//             result.unwrap_err()
//         ));
//     }

//     // let result = authorized_fetch(
//     //     &actor.id,
//     //     &InstanceActor::pub_key_id(&config.instance_domain),
//     //     &instance_actor.get_rsa(),
//     // )
//     // .await;
//     // if result.is_err() {
//     //     conn.delete_actor(uid, None).await.unwrap();
//     //     return Err(format!("failed to retrieve the actor from the id, don't worry if you see this and your dev env is not exposed to the internet {}", result.unwrap_err()));
//     // }

//     let deleted = conn.delete_actor(uid, None).await;
//     if deleted.is_err() {
//         return Err("failed to delete the test user".to_string());
//     }

//     handle.abort();
//     Ok(())
// }
