use js_int::Int;
#[cfg(feature = "unstable-msc3917")]
use ruma_common::{events::AnyStrippedStateEvent, serde::Raw};
use ruma_common::{
    serde::CanBeEmpty, MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedTransactionId, OwnedUserId,
};
#[cfg(feature = "unstable-msc3917")]
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

use super::{
    relation::{BundledMessageLikeRelations, BundledStateRelations},
    room::redaction::RoomRedactionEventContent,
    MessageLikeEventContent, OriginalSyncMessageLikeEvent, PossiblyRedactedStateEventContent,
};

/// Extra information about a message event that is not incorporated into the event's hash.
#[derive(Clone, Debug, Deserialize)]
#[serde(bound = "OriginalSyncMessageLikeEvent<C>: DeserializeOwned")]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct MessageLikeUnsigned<C: MessageLikeEventContent> {
    /// The time in milliseconds that has elapsed since the event was sent.
    ///
    /// This field is generated by the local homeserver, and may be incorrect if the local time on
    /// at least one of the two servers is out of sync, which can cause the age to either be
    /// negative or greater than it actually is.
    pub age: Option<Int>,

    /// The client-supplied transaction ID, if the client being given the event is the same one
    /// which sent it.
    pub transaction_id: Option<OwnedTransactionId>,

    /// [Bundled aggregations] of related child events.
    ///
    /// [Bundled aggregations]: https://spec.matrix.org/latest/client-server-api/#aggregations-of-child-events
    #[serde(rename = "m.relations", default)]
    pub relations: BundledMessageLikeRelations<OriginalSyncMessageLikeEvent<C>>,
}

impl<C: MessageLikeEventContent> MessageLikeUnsigned<C> {
    /// Create a new `Unsigned` with fields set to `None`.
    pub fn new() -> Self {
        Self { age: None, transaction_id: None, relations: BundledMessageLikeRelations::default() }
    }
}

impl<C: MessageLikeEventContent> Default for MessageLikeUnsigned<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: MessageLikeEventContent> CanBeEmpty for MessageLikeUnsigned<C> {
    /// Whether this unsigned data is empty (all fields are `None`).
    ///
    /// This method is used to determine whether to skip serializing the `unsigned` field in room
    /// events. Do not use it to determine whether an incoming `unsigned` field was present - it
    /// could still have been present but contained none of the known fields.
    fn is_empty(&self) -> bool {
        self.age.is_none() && self.transaction_id.is_none() && self.relations.is_empty()
    }
}

/// Extra information about a state event that is not incorporated into the event's hash.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct StateUnsigned<C: PossiblyRedactedStateEventContent> {
    /// The time in milliseconds that has elapsed since the event was sent.
    ///
    /// This field is generated by the local homeserver, and may be incorrect if the local time on
    /// at least one of the two servers is out of sync, which can cause the age to either be
    /// negative or greater than it actually is.
    pub age: Option<Int>,

    /// The client-supplied transaction ID, if the client being given the event is the same one
    /// which sent it.
    pub transaction_id: Option<OwnedTransactionId>,

    /// Optional previous content of the event.
    pub prev_content: Option<C>,

    /// [Bundled aggregations] of related child events.
    ///
    /// [Bundled aggregations]: https://spec.matrix.org/latest/client-server-api/#aggregations-of-child-events
    #[serde(rename = "m.relations", default)]
    pub relations: BundledStateRelations,
}

impl<C: PossiblyRedactedStateEventContent> StateUnsigned<C> {
    /// Create a new `Unsigned` with fields set to `None`.
    pub fn new() -> Self {
        Self { age: None, transaction_id: None, prev_content: None, relations: Default::default() }
    }
}

impl<C: PossiblyRedactedStateEventContent> CanBeEmpty for StateUnsigned<C> {
    /// Whether this unsigned data is empty (all fields are `None`).
    ///
    /// This method is used to determine whether to skip serializing the `unsigned` field in room
    /// events. Do not use it to determine whether an incoming `unsigned` field was present - it
    /// could still have been present but contained none of the known fields.
    fn is_empty(&self) -> bool {
        self.age.is_none()
            && self.transaction_id.is_none()
            && self.prev_content.is_none()
            && self.relations.is_empty()
    }
}

impl<C: PossiblyRedactedStateEventContent> Default for StateUnsigned<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Extra information about a redacted event that is not incorporated into the event's hash.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct RedactedUnsigned {
    /// The event that redacted this event, if any.
    pub redacted_because: UnsignedRoomRedactionEvent,
}

impl RedactedUnsigned {
    /// Create a new `RedactedUnsigned` with the given redaction event.
    pub fn new(redacted_because: UnsignedRoomRedactionEvent) -> Self {
        Self { redacted_because }
    }
}

/// A redaction event as found in `unsigned.redacted_because`.
///
/// While servers usually send this with the `redacts` field (unless nested), the ID of the event
/// being redacted is known from context wherever this type is used, so it's not reflected as a
/// field here.
///
/// It is intentionally not possible to create an instance of this type other than through `Clone`
/// or `Deserialize`.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UnsignedRoomRedactionEvent {
    /// Data specific to the event type.
    pub content: RoomRedactionEventContent,

    /// The globally unique event identifier for the user who sent the event.
    pub event_id: OwnedEventId,

    /// The fully-qualified ID of the user who sent this event.
    pub sender: OwnedUserId,

    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,

    /// Additional key-value pairs not signed by the homeserver.
    #[serde(default)]
    pub unsigned: MessageLikeUnsigned<RoomRedactionEventContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[cfg(feature = "unstable-msc3917")]
pub struct UnsignedRoomMemberEvent {
    /// An array holding a chain of stripped state events proving the user's possible membership
    /// in the room specified in the join rule, starting with the cause-of-membership event, and
    /// following parent events back to the specified room's m.room.create event.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "org.matrix.msc3917.v1.membership_events"
    )]
    pub membership_events: Option<Vec<Raw<AnyStrippedStateEvent>>>,
}
