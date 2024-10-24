use actix_web::web::Data;

use crate::db::{conn::EntityOrigin, dbconn::DbConn};

pub async fn notify_followers(
    conn: Data<DbConn>,
    post_id: &str,
    origin: EntityOrigin<'_>,
) {
    let ap_rep = conn.get_ap_post(post_id, &origin).await;
    

}