use crate::db::{
    conn::{DbErr, InsertErr},
    utility::new_actor::NewLocal,
};

use super::pg_conn::PgConn;

pub async fn create_local_user(conn: &PgConn, user: &NewLocal) -> Result<i64, DbErr> {
    let mut client = conn.db.get().await.expect("failed to get client");
    let transaction = client
        .transaction()
        .await
        .expect("failed to begin transaction");

    let stmt = r#"
        SELECT * FROM unified_users NATURAL JOIN internal_users WHERE preferred_username = $1;
        "#;
    let stmt = transaction.prepare(stmt).await.unwrap();

    let result = transaction
        .query(&stmt, &[&user.username])
        .await
        .expect("failed to get actor")
        .pop();

    //user already exists
    if result.is_some() {
        return Err(DbErr::InsertErr(InsertErr::AlreadyExists));
    }

    let stmt = r#"
    INSERT INTO internal_users 
    (
        password, preferred_username, email, private_key_pem,
        public_key_pem, permission_level, custom_domain
    )
    VALUES
    (
        $1, $2, $3, $4,
        $5, $6, $7
    )
    RETURNING local_id;
    "#;
    let stmt = transaction.prepare(stmt).await.unwrap();

    let permission: i16 = user.permission_level.into();

    let local_id: i64 = transaction
        .query(
            &stmt,
            &[
                &user.password,
                &user.username,
                &user.email,
                &user.private_key_pem,
                &user.public_key_pem,
                &permission,
                &user.custom_domain,
            ],
        )
        .await
        .expect("failed to insert user")
        .pop()
        .expect("did not return uid")
        .get("local_id");

    let stmt = r#"
    INSERT INTO unified_users 
    (
        is_local, local_id
    )
    VALUES
    (
        $1, $2
    )
    RETURNING uid;
    "#;
    let stmt = transaction.prepare(stmt).await.unwrap();

    let result = transaction
        .query(&stmt, &[&true, &local_id])
        .await
        .expect("failed to insert user")
        .pop()
        .expect("did not return uid")
        .get("uid");

    //update to have the new uid
    let stmt = r#"
        UPDATE internal_users
        SET uid = $1
        WHERE local_id = $2;
        "#;
    let stmt = transaction.prepare(stmt).await.unwrap();

    let _ = transaction
        .query(&stmt, &[&result, &local_id])
        .await
        .expect("failed to update user");

    transaction.commit().await.expect("failed to commit");

    Ok(result)
}
