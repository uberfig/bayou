use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseType {
    /// Indicates that the actor accepts the object. The target property
    /// can be used in certain circumstances to indicate the context into
    /// which the object has been accepted.
    ///
    /// example of an accept for a [`Follow`]
    ///
    /// ```json
    /// {
    ///   "@context": "https://www.w3.org/ns/activitystreams",
    ///   "summary": "sally accepts john's follow request",
    ///   "type": "Accept",
    ///   "actor": {
    ///     "type": "Person",
    ///     "name": "Sally"
    ///   },
    ///   "object": {
    ///     "type": "Follow",
    ///     "actor": "http://john.example.org",
    ///     "object": {
    ///       "id": "https://example.com",
    ///       "type": "Person",
    ///       "name": "Sally"
    ///     }
    ///   }
    /// }
    ///
    /// ```
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept
    Accept,
    /// Indicates that the actor is rejecting the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject
    Reject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowResponse {
    #[serde(rename = "type")]
    pub type_field: ResponseType,
    pub id: Url,
    pub actor: Url,
    pub object: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FollowType {
    Follow,
    /// currently activitypub is built around using undos to unfollow,
    /// however we will also accept a custom unfollow activity as this
    /// is much simpler. we will still support undos but we will send
    /// an unfollow as well when a user unfollows so hopefully one day
    /// the fedi can move to get rid of the undo activity and the
    /// needless complexity that it introduces
    Unfollow,
}
/// Indicates that the actor is "following" the object. Following
/// is defined in the sense typically used within Social systems in
/// which the actor is interested in any activity performed by or on
/// the object. The target and origin typically have no defined meaning.
///
/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow
///
/// The side effect of receiving this in an inbox is that the server
/// SHOULD generate either an [`ResponseType::Accept`] or
/// [`ResponseType::Reject`] activity with the Follow as the object
/// and deliver it to the actor of the Follow.
///
/// The Accept or Reject MAY be generated automatically, or MAY be the result of
/// user input (possibly after some delay in which the user reviews).
/// Servers MAY choose to not explicitly send a Reject in response to
/// a Follow, this would typically be represented as pending
///
/// https://www.w3.org/TR/activitypub/#follow-activity-inbox
///
/// example from activitystreams:
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally followed John",
///   "type": "Follow",
///   "actor": {
///     "id": "https://example.com",
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "id": "https://example.com",
///     "type": "Person",
///     "name": "John"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    #[serde(rename = "type")]
    pub type_field: FollowType,
    pub id: Url,
    pub actor: Url,
    pub object: Url,
}
