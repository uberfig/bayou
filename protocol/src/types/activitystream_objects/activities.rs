//---------------Activities--------------
use serde::{Deserialize, Serialize};
use url::Url;

use super::{actors::Actor, link::RangeLinkItem, object::Object};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(rename = "type")]
    pub type_field: ActivityType,

    pub id: Url,
    // #[serde(rename = "type")]
    // pub type_field: IntransitiveType,
    // #[serde(flatten)]
    // pub extends_object: Object,
    pub actor: RangeLinkItem<Actor>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>, //TODO

    pub object: RangeLinkItem<Object>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActivityType {
    Activity,
    /// A specialization of [`Accept`] indicating that the acceptance is tentative.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept
    TentativeAccept,
    /// Indicates that the actor has added the object to the target.
    /// If the target property is not explicitly specified, the target
    /// would need to be determined implicitly by context. The origin
    /// can be used to identify the context from which the object originated.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add
    Add,
    /// Indicates that the actor is ignoring the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore
    Ignore,
    /// Indicates that the actor has joined the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join
    Join,
    /// Indicates that the actor has left the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave
    Leave,
    /// Indicates that the actor likes, recommends or endorses the object.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like
    Like,
    /// Indicates that the actor is offering the object.
    /// If specified, the target indicates the entity to which the object is being offered.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer
    Offer,
    /// A specialization of [`Offer`] in which the actor is extending an invitation for the object to the target.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite
    Invite,
    /// A specialization of [`Reject`] in which the rejection is considered tentative.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject
    TentativeReject,
    /// Indicates that the actor is removing the object.
    /// If specified, the origin indicates the context from which the object is being removed.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove
    Remove,
    /// Indicates that the actor is undoing the object. In most cases,
    /// the object will be an [`Activity`] describing some previously performed action (for instance,
    /// a person may have previously "liked" an article but, for whatever reason,
    /// might choose to undo that like at some later point in time).
    ///
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-undo
    ///
    /// The Undo activity is used to undo a previous activity. See the
    /// Activity Vocabulary documentation on Inverse Activities and "Undo".
    /// For example, Undo may be used to undo a previous [`ActivityType::Like`],
    /// [`ActivityType::Follow`], or [`ActivityType::Block`]. The undo activity
    /// and the activity being undone MUST both have the same actor. Side effects
    /// should be undone, to the extent possible. For example, if undoing a Like,
    /// any counter that had been incremented previously should be decremented appropriately.
    ///
    /// There are some exceptions where there is an existing and explicit
    /// "inverse activity" which should be used instead.
    /// [`ActivityType::Create`] based activities should instead use
    /// [`ActivityType::Delete`], and [`ActivityType::Add`] activities
    /// should use [`ActivityType::Remove`].
    ///
    /// https://www.w3.org/TR/activitypub/#undo-activity-outbox
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#inverse
    Undo,
    /// Indicates that the actor has updated the object.
    /// Note, however, that this vocabulary does not define a mechanism for
    /// describing the actual set of modifications made to object.
    ///
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update
    Update,
    /// Indicates that the actor has viewed the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view
    View,
    /// Indicates that the actor has listened to the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen
    Listen,
    /// Indicates that the actor has read the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read
    Read,
    /// Indicates that the actor has moved object from origin to target.
    /// If the origin or target are not specified, either can be determined by context.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move
    Move,
    /// Indicates that the actor is calling the target's attention the object.
    /// The origin typically has no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce
    Announce,
    /// Indicates that the actor is blocking the object.
    /// Blocking is a stronger form of [`Ignore`].
    /// The typical use is to support social systems that allow one user
    /// to block activities or content of other users.
    /// The target and origin typically have no defined meaning.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block
    Block,
    /// Indicates that the actor is "flagging" the object.
    /// Flagging is defined in the sense common to many social platforms
    /// as reporting content as being inappropriate for any number of reasons.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag
    Flag,
    /// Indicates that the actor dislikes the object.
    ///
    /// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-dislike
    Dislike,
}
