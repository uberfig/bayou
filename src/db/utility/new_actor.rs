use openssl::rsa::Rsa;
use url::Url;

use super::permission::PermissionLevel;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub struct UserLinks {
    pub id: Url,
    pub inbox: Url,
    pub outbox: Url,
    pub followers: Url,
    pub following: Url,
    pub liked: Url,
    pub url: Url,
    pub pub_key_id: Url,
}

pub fn generate_links(domain: &str, uname: &str) -> UserLinks {
    UserLinks {
        id: Url::parse(&format!("https://{domain}/users/{uname}")).unwrap(),
        inbox: Url::parse(&format!("https://{domain}/users/{uname}/inbox")).unwrap(),
        outbox: Url::parse(&format!("https://{domain}/users/{uname}/outbox")).unwrap(),
        followers: Url::parse(&format!("https://{domain}/users/{uname}/followers")).unwrap(),
        following: Url::parse(&format!("https://{domain}/users/{uname}/following")).unwrap(),
        liked: Url::parse(&format!("https://{domain}/users/{uname}/liked")).unwrap(),
        url: Url::parse(&format!("https://{domain}/@{uname}")).unwrap(),
        pub_key_id: Url::parse(&format!("https://{domain}/users/{uname}#main-key")).unwrap(),
    }
}

pub fn instance_actor_links(domain: &str) -> UserLinks {
    UserLinks {
        id: Url::parse(&format!("https://{domain}/actor")).unwrap(),
        inbox: Url::parse(&format!("https://{domain}/actor/inbox")).unwrap(),
        outbox: Url::parse(&format!("https://{domain}/actor/outbox")).unwrap(),
        followers: Url::parse(&format!("https://{domain}/actor/followers")).unwrap(),
        following: Url::parse(&format!("https://{domain}/actor/following")).unwrap(),
        liked: Url::parse(&format!("https://{domain}/actor/liked")).unwrap(),
        url: Url::parse(&format!("https://{domain}:")).unwrap(),
        pub_key_id: Url::parse(&format!("https://{domain}/actor#main-key")).unwrap(),
    }
}

/// since this is intended to be a dumb implimentation, the
/// "password" being passed in should be the hashed argon2
/// output containing the hash and the salt. the database
/// should not be responsible for performing this task
pub struct NewLocal {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub permission_level: PermissionLevel,
    pub private_key_pem: String,
    pub public_key_pem: String,
    pub custom_domain: Option<String>,
}

impl NewLocal {
    pub fn new(
        username: String,
        password: String,
        email: Option<String>,
        custom_domain: Option<String>,
        permission_level: Option<PermissionLevel>,
    ) -> Self {
        let permission_level = match permission_level {
            Some(x) => x,
            None => PermissionLevel::UntrustedUser,
        };
        let rsa = Rsa::generate(2048).unwrap();
        let private_key_pem = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
        let public_key_pem = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        NewLocal {
            username,
            password: password_hash,
            email,
            permission_level,
            private_key_pem,
            public_key_pem,
            custom_domain,
        }
    }
}