use bayou_protocol::{
    cryptography::{key::{Algorithms, Key}, openssl::OpenSSLPrivate},
    types::activitystream_objects::actors::Actor,
};

pub struct InstanceActor {
    pub private_key_pem: String,
    pub public_key_pem: String,
    pub algorithm: Algorithms,
}

impl InstanceActor {
    pub fn pub_key_id(domain: &str) -> String {
        format!("https://{domain}/actor/ap#main-key")
    }
    // pub async fn init_instance_actor(conn: &dyn Conn) {
    //     if conn.get_instance_actor().await.is_none() {
    //         let key = OpenSSLPrivate::generate();
    //         conn.create_instance_actor(key.private_key_pem(), key.public_key_pem())
    //             .await;
    //     }
    // }
    pub fn get_private_key(&self) -> OpenSSLPrivate {
        OpenSSLPrivate::from_pem(self.private_key_pem.as_bytes()).expect("invalid instance actor key")
    }
    pub fn get_key_id(domain: &str) -> String {
        format!("https://{}/{}", domain, domain)
    }
    pub fn to_actor(&self, _domain: &str) -> Actor {
        // let links = instance_actor_links(domain);
        todo!()
        // Actor {
        //     type_field: crate::activitystream_objects::actors::ActorType::Application,
        //     id: links.id.clone(),
        //     preferred_username: domain.to_string(),
        //     summary: None,
        //     name: None,
        //     url: Some(
        //         Url::parse(&format!("https://{domain}/about/more?instance_actor=true")).unwrap(),
        //     ),
        //     public_key: PublicKey {
        //         id: links.pub_key_id,
        //         owner: links.id,
        //         public_key_pem: self.public_key_pem.clone(),
        //     },
        //     inbox: links.inbox,
        //     outbox: links.outbox,
        //     followers: links.followers,
        //     following: links.following,
        //     domain: Some(domain.to_string()),
        //     liked: Some(links.liked),
        // }
    }
}
