use uuid::Uuid;

use crate::{db::pg_sesh::Sesh, routes::api::types::proxy_user::ApiProxyUser};

const fn create_statement() -> &'static str {
    r#"
    INSERT INTO proxies
    (proxy_id, uid, proxy_name, proxy_created, proxy_bio)
    VALUES
    ($1, $2, $3, $4, $5)
    RETURNING *;
    "#
}
const fn read_statement() -> &'static str {
    r#"
    SELECT * FROM proxies WHERE proxy_id = $1;
    "#
}
const fn update_statement() -> &'static str {
    r#"
    UPDATE proxies SET
    proxy_name = $1,
    proxy_bio = $2
    WHERE proxy_id = $3
    RETURNING *;
    "#
}
const fn delete_statement() -> &'static str {
    r#"
    DELETE FROM proxies WHERE proxy_id = $1;
    "#
}
const fn user_proxies() -> &'static str {
    r#"
    SELECT * FROM proxies WHERE uid = $1;
    "#
}

#[allow(dead_code, unused_variables)]
impl Sesh<'_> {
    pub async fn create_proxy(&self, new_proxy: ApiProxyUser) -> ApiProxyUser {
        let result = self
            .query(
                create_statement(),
                &[&new_proxy.id, &new_proxy.parent_id, &new_proxy.name, &new_proxy.created, &new_proxy.bio],
            )
            .await
            .expect("failed to create proxy")
            .pop()
            .expect("creating proxy returned nothing");
        ApiProxyUser::from_row(&result)
    }
    pub async fn delete_proxy(&self, proxy_id: Uuid) {
        delete_statement();
        todo!()
    }
    pub async fn get_proxy(&self, proxy_id: Uuid) -> Option<ApiProxyUser> {
        let stmt = read_statement();
        let result = self
            .query(stmt, &[&proxy_id])
            .await
            .expect("failed to fetch proxy")
            .pop();
        result.map(|x| ApiProxyUser::from_row(&x))
    }
    pub async fn update_proxy(&self, proxy: ApiProxyUser) -> Option<ApiProxyUser> {
        update_statement();
        todo!()
    }
    pub async fn get_user_proxies(&self, uid: Uuid) -> Vec<ApiProxyUser> {
        user_proxies();
        todo!()
    }
}