#[cfg(feature = "crypto")]
use super::link::RangeLinkItem;
use super::{
    create::Create,
    delete::Delete,
    follow_and_response::{Follow, FollowResponse},
    postable::ApPostable,
};
#[cfg(feature = "crypto")]
use crate::cryptography::key::PrivateKey;
#[cfg(feature = "crypto")]
use crate::protocol::ap_protocol::fetch::authorized_fetch;
#[cfg(feature = "protocol")]
use crate::protocol::ap_protocol::signature::Algorithms;
#[cfg(feature = "protocol")]
use crate::protocol::errors::FetchErr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Inboxable {
    /// while activitypub requires special create activities,
    /// we will also support just recieving the post itself
    /// as the create is redundant. recieving a post that already
    /// exists on this instance will trigger an update
    ///
    /// allowing this breaks spec but we feel it makes things
    /// much simpler and hopefully a future version of the
    /// spec can allow for this. when creating new posts
    /// we will just do creates as normally but we may send
    /// the post on its own as well down the road
    Postable(ApPostable),
    Create(Create),
    Delete(Delete),
    Follow(Follow),
    FollowResponse(FollowResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VerifiedInboxable {
    /// Creates and updates are simplified into just a postable
    ///
    /// we need to check if the thing already exists in the db
    /// anyway, might as well use that to determine the db logic
    Postable(ApPostable),
    Delete(Delete),
    Follow(Follow),
    FollowResponse(FollowResponse),
}

#[cfg(feature = "protocol")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InboxableVerifyErr {
    InnerFetchErr(FetchErr),
    ForgedAttribution,
}

#[cfg(feature = "protocol")]
impl Inboxable {
    pub async fn verify<K: PrivateKey>(
        self,
        origin_domain: &str,
        instance_key_id: &str,
        instance_private_key: &mut K,
        algorithm: Algorithms,
    ) -> Result<VerifiedInboxable, InboxableVerifyErr> {
        match self {
            Inboxable::Postable(postable) => match postable.verify(origin_domain) {
                Ok(x) => Ok(VerifiedInboxable::Postable(x)),
                Err(x) => return Err(x),
            },
            Inboxable::Create(create) => Ok({
                let postable = match create.object {
                    RangeLinkItem::Item(x) => x,
                    RangeLinkItem::Link(post_id) => {
                        let postable: Result<ApPostable, FetchErr> = authorized_fetch(
                            post_id.get_id().to_owned(),
                            instance_key_id,
                            instance_private_key,
                            algorithm,
                        )
                        .await;
                        match postable {
                            Ok(postable) => postable,
                            Err(x) => return Err(InboxableVerifyErr::InnerFetchErr(x)),
                        }
                    }
                };
                match postable.verify(origin_domain) {
                    Ok(x) => VerifiedInboxable::Postable(x),
                    Err(x) => return Err(x),
                }
            }),
            Inboxable::Delete(delete) => {
                if delete.actor.domain().ne(&Some(origin_domain))
                    || delete.object.domain().ne(&Some(origin_domain))
                {
                    return Err(InboxableVerifyErr::ForgedAttribution);
                }
                Ok(VerifiedInboxable::Delete(delete))
            }
            Inboxable::Follow(follow) => {
                if follow.actor.domain().ne(&Some(origin_domain))
                    || follow.id.domain().ne(&Some(origin_domain))
                {
                    return Err(InboxableVerifyErr::ForgedAttribution);
                }
                Ok(VerifiedInboxable::Follow(follow))
            }
            Inboxable::FollowResponse(follow_response) => {
                if follow_response.actor.domain().ne(&Some(origin_domain))
                    || follow_response.id.domain().ne(&Some(origin_domain))
                {
                    return Err(InboxableVerifyErr::ForgedAttribution);
                }
                Ok(VerifiedInboxable::FollowResponse(follow_response))
            }
        }
    }
}
