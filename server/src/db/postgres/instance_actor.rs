use bayou_protocol::protocol::ap_protocol::signature::Algorithms;

use crate::db::utility::instance_actor::InstanceActor;

use super::pg_conn::PgConn;

#[allow(dead_code)]
pub async fn get_instance_actor(conn: &PgConn) -> Option<InstanceActor> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
    SELECT * FROM ap_instance_actor;
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await
        .expect("failed to get instance actor")
        .pop();
    result.map(|result| InstanceActor {
        private_key_pem: result.get("private_key_pem"),
        public_key_pem: result.get("public_key_pem"),
        algorithm: Algorithms::Hs2019,
    })
}

// pub async fn create_instance_actor(conn: &PgConn, private_key_pem: &str, public_key_pem: &str) {
//     let client = conn.db.get().await.expect("failed to get client");
//     let stmt = r#"
//     INSERT INTO ap_instance_actor
//     (private_key_pem, public_key_pem)
//     VALUES
//     ($1, $2);
//     "#;
//     let stmt = client.prepare(stmt).await.unwrap();

//     let result = client
//         .query(&stmt, &[&private_key_pem, &public_key_pem])
//         .await;
//     result.unwrap();
// }
