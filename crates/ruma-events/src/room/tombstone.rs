//! Types for the [`m.room.tombstone`] event.
//!
//! [`m.room.tombstone`]: https://spec.matrix.org/latest/client-server-api/#mroomtombstone

use ruma_common::OwnedRoomId;
#[cfg(feature = "unstable-msc3917")]
use ruma_common::{encryption::CrossSigningKeySignatures, OwnedEventId};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::{
    EmptyStateKey, EventContent, PossiblyRedactedStateEventContent, StateEventType,
    StaticEventContent,
};

/// The content of an `m.room.tombstone` event.
///
/// A state event signifying that a room has been upgraded to a different room version, and that
/// clients should go there.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(
    type = "m.room.tombstone",
    kind = State,
    state_key_type = EmptyStateKey,
    custom_possibly_redacted,
)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct RoomTombstoneEventContent {
    /// A server-defined message.
    ///
    /// If the `compat-optional` feature is enabled, this field being absent in JSON will result
    /// in an empty string instead of an error when deserializing.
    #[cfg_attr(feature = "compat-optional", serde(default))]
    pub body: String,

    /// The new room the client should be visiting.
    pub replacement_room: OwnedRoomId,

    /// The sender's public Room Signing Key, signed by their Master Signing Key, in the same
    /// CrossSigningKey format used by the /keys/device_signing/upload endpoint. This field is
    /// provided in order to simplify the process of connecting the sender's MSK to their RSK,
    /// particularly in cases where the sender may no longer be in the room or may have even
    /// deactivated their account.
    #[cfg(feature = "unstable-msc3917")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "org.matrix.msc3917.v1.sender_key")]
    pub sender_key: Option<String>,

    /// The ID of the sender's cause-of-membership event.
    #[cfg(feature = "unstable-msc3917")]
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "org.matrix.msc3917.v1.parent_event_id"
    )]
    pub parent_event_id: Option<OwnedEventId>,

    /// docs tbd, rrk of room being tombstoned
    #[cfg(feature = "unstable-msc3917")]
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "org.matrix.msc3917.v1.room_root_key"
    )]
    pub room_root_key: Option<String>,

    /// A signature of this event's content by the sender's RSK, generated using the normal
    /// process for signing JSON objects.
    #[cfg(feature = "unstable-msc3917")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "org.matrix.msc3917.v1.signatures")]
    pub signatures: Option<CrossSigningKeySignatures>,
}

impl RoomTombstoneEventContent {
    /// Creates a new `RoomTombstoneEventContent` with the given body and replacement room ID.
    pub fn new(body: String, replacement_room: OwnedRoomId) -> Self {
        Self {
            body,
            replacement_room,
            #[cfg(feature = "unstable-msc3917")]
            sender_key: None,
            #[cfg(feature = "unstable-msc3917")]
            parent_event_id: None,
            #[cfg(feature = "unstable-msc3917")]
            room_root_key: None,
            #[cfg(feature = "unstable-msc3917")]
            signatures: None,
        }
    }
}

/// The possibly redacted form of [`RoomTombstoneEventContent`].
///
/// This type is used when it's not obvious whether the content is redacted or not.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct PossiblyRedactedRoomTombstoneEventContent {
    /// A server-defined message.
    pub body: Option<String>,

    /// The new room the client should be visiting.
    pub replacement_room: Option<OwnedRoomId>,
}

impl EventContent for PossiblyRedactedRoomTombstoneEventContent {
    type EventType = StateEventType;

    fn event_type(&self) -> Self::EventType {
        StateEventType::RoomTombstone
    }
}

impl PossiblyRedactedStateEventContent for PossiblyRedactedRoomTombstoneEventContent {
    type StateKey = EmptyStateKey;
}

impl StaticEventContent for PossiblyRedactedRoomTombstoneEventContent {
    const TYPE: &'static str = "m.room.tombstone";
}
