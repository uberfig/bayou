use bayou_protocol::cryptography::{
    key::{Algorithms, Key, PrivateKey},
    openssl::OpenSSLPrivate,
};

use crate::db::utility::instance_actor::InstanceActor;

use super::pg_conn::PgConn;

#[allow(dead_code)]
pub async fn get_instance_actor(conn: &PgConn, algorithm: Algorithms) -> InstanceActor {
    let mut client = conn.db.get().await.expect("failed to get client");
    let transaction = client
        .transaction()
        .await
        .expect("failed to begin transaction");
    let stmt = r#"
    SELECT * FROM ap_instance_actor where algorithm = $1;
    "#;
    let stmt = transaction
        .prepare(stmt)
        .await
        .expect("failed to prepare query");

    let result = transaction
        .query(&stmt, &[&algorithm.to_string()])
        .await
        .expect("failed to get instance actor")
        .pop();

    if let Some(result) = result {
        return InstanceActor {
            private_key_pem: result.get("private_key_pem"),
            public_key_pem: result.get("public_key_pem"),
            algorithm,
        };
    }
    let private_key = OpenSSLPrivate::generate(algorithm);
    let instance_actor = InstanceActor {
        private_key_pem: private_key.to_pem().unwrap(),
        public_key_pem: private_key.public_key_pem().unwrap(),
        algorithm,
    };
    create_instance_actor(&transaction, &instance_actor).await;

    transaction
        .commit()
        .await
        .expect("failed to commit transaction");

    return instance_actor;
}

async fn create_instance_actor(
    transaction: &deadpool_postgres::Transaction<'_>,
    instance_actor: &InstanceActor,
) {
    let stmt = r#"
    INSERT INTO ap_instance_actor
    (private_key_pem, public_key_pem, algorithm)
    VALUES
    ($1, $2, $3);
    "#;
    let stmt = transaction
        .prepare(stmt)
        .await
        .expect("failed to prepare query");
    let _ = transaction
        .query(
            &stmt,
            &[
                &instance_actor.private_key_pem,
                &instance_actor.public_key_pem,
                &instance_actor.algorithm.to_string(),
            ],
        )
        .await
        .expect("failed to insert instance actor");
}
