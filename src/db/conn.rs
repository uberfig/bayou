use bayou_protocol::{
    cryptography::openssl::OpenSSLPublic,
    protocol::{errors::FetchErr, versia_protocol::requests::Signer},
    types::{
        activitystream_objects::{actors::Actor, new_post::NewPost, postable::ApPostable},
        versia_types::{
            entities::{instance_metadata::InstanceMetadata, user::User},
            postable::VersiaPostable,
        },
    },
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    types::{Follower, FollowerEndpoint},
    utility::{instance_actor::InstanceActor, new_actor::NewLocal, protocols::Protocol},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InsertErr {
    AlreadyExists,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DbErr {
    FetchErr(FetchErr),
    InsertErr(InsertErr),
    InvalidType,
}

impl std::fmt::Display for DbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

/// the origin of a post containing its instance domain
pub enum EntityOrigin<'a> {
    Local(&'a str),
    Federated(&'a str),
}

pub enum ProtoUser {
    Versia(User),
    ActivityPub(Actor),
}

#[allow(unused_variables)]
#[enum_dispatch(UncachedConn)]
#[enum_dispatch(CachedConn)]
#[enum_dispatch(DbConn)]
pub trait Conn: Sync {
    // async fn get_actor_post_count(&self, uname: &str, origin: &EntityOrigin) -> Option<u64>;
    async fn get_uuid_url(&self, url: &Url) -> &str {
        todo!()
    }
    async fn get_user_posts_ap(
        &self,
        uname: &str,
        origin: &EntityOrigin<'_>,
        page_size: u64,
        ofset: u64,
    ) -> Option<Vec<ApPostable>> {
        todo!()
    }
    async fn get_ap_post(&self, post_id: &str, origin: &EntityOrigin<'_>) -> Option<ApPostable> {
        todo!()
    }
    /// inserts a federated post into the db and returns the uuid if successful
    async fn create_ap_post(
        &self,
        post: ApPostable,
        origin: &EntityOrigin<'_>,
    ) -> Result<String, ()> {
        todo!()
    }
    async fn new_local_post(
        &self,
        new_post: NewPost,
        origin: &EntityOrigin<'_>,
    ) -> Result<String, DbErr> {
        todo!()
    }
    /// run any prep for the database, for example running migrations
    async fn init(&self) -> Result<(), String> {
        todo!()
    }
    /// gets the instance actor. creates one if its not present
    async fn get_instance_actor(&self) -> InstanceActor {
        todo!()
    }

    /// returns the uid if sucessful
    async fn create_user(&self, domain: &str, content: &NewLocal) -> Result<String, ()> {
        todo!()
    }
    /// gets actor, backfills if not in db. returns none if not in the db and defederated or unable to fetch
    async fn backfill_actor(&self, username: &str, origin: &EntityOrigin<'_>) -> Option<Actor> {
        todo!()
    }
    async fn get_actor(&self, username: &str, origin: &EntityOrigin<'_>) -> Option<Actor> {
        todo!()
    }

    /// signed_by will always be user for activitypub users
    /// this will backfill the user if they aren't in the db yet
    async fn get_public_key(&self, signed_by: &Signer) -> Option<OpenSSLPublic> {
        todo!()
    }

    //-------------------------versia---------------------

    // TODO make versia routes just use unames
    async fn get_user_post_count(&self, uname: &str, origin: &EntityOrigin<'_>) -> Option<u64> {
        todo!()
    }
    /// ofset is one based
    async fn get_user_posts_versia(
        &self,
        uuid: &str,
        origin: &EntityOrigin<'_>,
        page_size: u64,
        ofset: u64,
    ) -> Option<Vec<VersiaPostable>> {
        todo!()
    }
    async fn get_key(&self, signed_by: &Signer) -> Option<OpenSSLPublic> {
        todo!()
    }
    /// gets the metadata of an instance, backfills if not present
    async fn get_versia_instance_metadata(
        &self,
        instance_domain: &str,
    ) -> Option<InstanceMetadata> {
        todo!()
    }
    /// get the protocol of the given instance. will backfill if the instance isn't in the db
    async fn get_protocol(&self, instance: &str) -> Protocol {
        todo!()
    }
    async fn get_versia_user(&self, uuid: &str, origin: &EntityOrigin<'_>) -> Option<User> {
        todo!()
    }
    async fn get_versia_post(
        &self,
        post_id: &str,
        origin: &EntityOrigin<'_>,
    ) -> Option<VersiaPostable> {
        todo!()
    }
    /// create a post and return the post
    async fn create_versia_post(
        &self,
        post: VersiaPostable,
        origin: &EntityOrigin<'_>,
    ) -> Result<VersiaPostable, ()> {
        todo!()
    }
    async fn delete_post(&self, post_id: &str, origin: &EntityOrigin<'_>) -> Result<(), ()> {
        todo!()
    }
    async fn delete_user(&self, uid: &Url, origin: &EntityOrigin<'_>) -> Result<(), ()> {
        todo!()
    }

    // //------------------------------posts---------------------------------

    // async fn create_new_post(
    //     &self,
    //     post: &PostType,
    //     instance_domain: &str,
    //     uid: i64,
    //     is_local: bool,
    //     in_reply_to: Option<i64>,
    // ) -> i64;

    // async fn get_post(&self, object_id: i64) -> Option<PostType>;

    // //------------------------------likes-----------------------------------

    // // async fn create_like(&self, uid: i64, obj_id: i64) -> Result<(), ()>;
    // // async fn remove_like(&self, uid: i64, obj_id: i64) -> Result<(), ()>;
    // // async fn get_post_likes(&self, obj_id: i64) -> Result<Vec<Like>, ()>;
    // // async fn get_user_likes(&self, uid: i64) -> Result<Vec<Like>, ()>;

    // //-------------------------private keys----------------------------

    // /// get the private key of a local user, none if we don't have authority over them
    async fn get_private_key_pem(&self, uuid: &str) -> Option<String> {
        todo!()
    }

    // //--------------------followers---------------------------------

    async fn create_follow_request(&self, from: &str, to: &str, pending: bool) -> Result<(), ()> {
        todo!()
    }

    /// approves an existing follow request and creates the record in
    /// the followers
    async fn approve_follow_request(&self, from: &str, to: &str) -> Result<(), ()> {
        todo!()
    }

    /// in the event that we cannot view from the source instance, just show
    /// local followers
    async fn get_followers(&self, user: &str) -> Result<Vec<Follower>, DbErr> {
        todo!()
    }

    /// only intended for local users, will deduplicate users with the same shared inbox,
    /// will also skip users we are authoratative over
    async fn get_follower_inboxes(&self, user: &str) -> Result<Vec<FollowerEndpoint>, DbErr> {
        todo!()
    }
}
