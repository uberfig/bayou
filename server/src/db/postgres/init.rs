use std::ops::DerefMut;

use super::pg_conn::PgConn;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn init(conn: &PgConn, primary_domain: &str) -> Result<(), String> {
    let mut conn = conn
        .db
        .get()
        .await
        .expect("could not get conn for migrations");
    let client = conn.deref_mut().deref_mut();
    let report = embedded::migrations::runner().run_async(client).await;
    match report {
        Ok(x) => {
            println!("migrations sucessful");
            if x.applied_migrations().is_empty() {
                println!("no migrations applied")
            } else {
                println!("applied migrations: ");
                for migration in x.applied_migrations() {
                    match migration.applied_on() {
                        Some(x) => println!(" - {} applied {}", migration.name(), x),
                        None => println!(" - {} applied N/A", migration.name()),
                    }
                }
            }
        }
        Err(x) => {
            return Err(x.to_string());
        }
    }

    let stmt = r#"
        INSERT INTO instances
        (
            domain, is_primary, is_authoratative, allowlisted
        )
        VALUES
        ($1, $2, $3, $4)
        ON CONFLICT (domain) DO UPDATE
        SET is_primary = true, is_authoratative = true, allowlisted = true;
        "#;
    let stmt = conn.prepare(stmt).await.unwrap();

    let result = conn
        .query(&stmt, &[&primary_domain, &true, &true, &true])
        .await;
    if let Err(err) = result {
        return Err(err.to_string());
    }

    Ok(())
}
