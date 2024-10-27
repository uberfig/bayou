use super::pg_conn::PgConn;

pub async fn get_local_manually_approves_followers(conn: &PgConn, uid: i64) -> bool {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
    SELECT * FROM unified_users NATURAL JOIN internal_users WHERE uid = $1;
    "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&uid])
        .await
        .expect("failed to get local user")
        .pop()
        .unwrap();
    result.get("manual_followers")
}
