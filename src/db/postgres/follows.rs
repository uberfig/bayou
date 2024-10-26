use std::time::{SystemTime, UNIX_EPOCH};

use tokio_postgres::Row;

use crate::db::types::Follower;

use super::pg_conn::PgConn;

///requires following joined on unified_users
fn to_follower(row: Row) -> Follower {
    Follower {
        uuid: row.get("follower"),
        is_local: row.get("is_local"),
        protocol: serde_json::from_str(row.get("protocol"))
            .expect("failed to deserialize protocol"),
    }
}

pub async fn create_follow_request(
    conn: &PgConn,
    from_id: i64,
    to_id: i64,
    pending: bool,
) -> Result<(), ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        INSERT INTO following 
        (
            follower, target_user,
            pending, published
        )
        VALUES
        (
            $1, $2, 
            $3, $4
        );
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let result = client
        .query(&stmt, &[&from_id, &to_id, &pending, &created])
        .await;
    // .expect("failed to create follow");
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub async fn approve_follow_request(conn: &PgConn, from_id: i64, to_id: i64) -> Result<(), ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        UPDATE following 
        SET pending = false
        WHERE
        follower = $1
        target_user = $2;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let _result = client
        .query(&stmt, &[&from_id, &to_id])
        .await
        .expect("failed to approve follow");

    Ok(())
}

pub async fn get_followers(conn: &PgConn, user: i64) -> Result<Vec<Follower>, ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
            SELECT * FROM following INNER JOIN unified_users ON follower = uid
            WHERE
            target_user = $1;
            "#;
    let stmt = client.prepare(stmt).await.unwrap();

    // let (target_fedi, target_local) = user.parts();

    let result = client.query(&stmt, &[&user]).await;
    // .expect("failed to get followers");
    // dbg!(&result);
    let result = result.unwrap();

    let x = result.into_iter().map(to_follower).collect();
    dbg!(&x);

    Ok(x)
}

pub async fn get_follower_count(conn: &PgConn, user: i64) -> Result<i64, ()> {
    let client = conn.db.get().await.expect("failed to get client");
    let stmt = r#"
        SELECT COUNT(*) FROM following 
        WHERE
        target_user = $1;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result: i64 = client
        .query(&stmt, &[&user])
        .await
        .expect("failed to get follow count")
        .pop()
        .expect("did not return row for follow count")
        .get("count");

    Ok(result)
}

pub async fn get_follow(conn: &PgConn, from_id: i64, to_id: i64) -> Option<Follower> {
    let client = conn.db.get().await.expect("failed to get client");

    let stmt = r#"
        SELECT * FROM following NATURAL JOIN unified_users
        WHERE
        follower = $1 AND
        target_user = $2;
        "#;
    let stmt = client.prepare(stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&from_id, &to_id])
        .await
        .expect("failed to get follow count")
        .pop();

    result.map(to_follower)
}
