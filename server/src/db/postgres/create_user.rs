use uuid::Uuid;

use crate::db::{
    conn::{DbErr, InsertErr},
    utility::new_actor::{generate_ap_links, NewLocal},
};

use super::pg_conn::PgConn;

pub async fn create_user(conn: &PgConn, domain: &str, new_user: &NewLocal) -> Result<Uuid, DbErr> {
    let mut client = conn.db.get().await.expect("failed to get client");
    let transaction = client
        .transaction()
        .await
        .expect("failed to begin transaction");

    let stmt = r#"
        SELECT * FROM users WHERE preferred_username = $1 AND domain = $2;
        "#;
    let stmt = transaction.prepare(stmt).await.unwrap();

    let result = transaction
        .query(&stmt, &[&new_user.username, &domain])
        .await
        .expect("failed to get actor")
        .pop();
    if result.is_some() {
        return Err(DbErr::InsertErr(InsertErr::AlreadyExists));
    }

    let links = generate_ap_links(domain, &new_user.username);

    let uuid = uuid::Uuid::now_v7();

    let stmt = r#"
        INSERT INTO users
    (
        uid, resource_link, versia_id, url,
        domain, username, public_key_pem, public_key_id,
        inbox, outbox, followers, following,
        password, email, private_key_pem, permission_level
    )
    VALUES
    (
        $1, $2, $3, $4,
        $5, $6, $7, $8,
        $9, $10, $11, $12,
        $13, $14, $15, $16
    );"#;
    let stmt = transaction.prepare(stmt).await.unwrap();
    let uuid_str = &mut Uuid::encode_buffer();
    let uuid_str = &*uuid.as_hyphenated().encode_lower(uuid_str);

    let result = transaction
        .query(
            &stmt,
            &[
                &uuid_str,
                &links.id.to_string(),
                &uuid_str,
                &links.url.to_string(),
                &domain,
                &new_user.username,
                &new_user.public_key_pem,
                &links.pub_key_id.to_string(),
                &links.inbox.to_string(),
                &links.outbox.to_string(),
                &links.followers.to_string(),
                &links.following.to_string(),
                &new_user.password,
                &new_user.email,
                &new_user.private_key_pem,
                &new_user.permission_level.to_i16(),
            ],
        )
        .await;

    if result.is_err() {
        return Err(DbErr::InsertErr(InsertErr::Failure));
    }
    transaction
        .commit()
        .await
        .expect("failed to commit new user");

    Ok(uuid)
}
