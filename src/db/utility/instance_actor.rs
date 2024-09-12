use openssl::{pkey::Private, rsa::Rsa};
use url::Url;

use crate::{
    activitystream_objects::actors::{Actor, PublicKey},
    db::conn::Conn,
};

use super::new_actor::instance_actor_links;

pub struct InstanceActor {
    pub private_key_pem: String,
    pub public_key_pem: String,
}

impl InstanceActor {
    pub fn pub_key_id(domain: &str) -> String {
        format!("https://{domain}/actor#main-key")
    }
    pub async fn init_instance_actor(conn: &dyn Conn) {
        if conn.get_instance_actor().await.is_none() {
            let rsa = Rsa::generate(2048).unwrap();
            let private_key_pem = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
            let public_key_pem = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
            conn.create_instance_actor(private_key_pem, public_key_pem)
                .await;
        }
    }
    pub fn get_rsa(&self) -> Rsa<Private> {
        openssl::rsa::Rsa::private_key_from_pem(self.private_key_pem.as_bytes()).unwrap()
    }
    pub fn to_actor(&self, domain: &str) -> Actor {
        let links = instance_actor_links(domain);
        Actor {
            type_field: crate::activitystream_objects::actors::ActorType::Application,
            id: links.id.clone(),
            preferred_username: domain.to_string(),
            summary: None,
            name: None,
            url: Some(
                Url::parse(&format!("https://{domain}/about/more?instance_actor=true")).unwrap(),
            ),
            public_key: PublicKey {
                id: links.pub_key_id,
                owner: links.id,
                public_key_pem: self.public_key_pem.clone(),
            },
            inbox: links.inbox,
            outbox: links.outbox,
            followers: links.followers,
            following: links.following,
            domain: Some(domain.to_string()),
            liked: Some(links.liked),
        }
    }
}