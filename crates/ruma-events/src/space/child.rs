//! Types for the [`m.space.child`] event.
//!
//! [`m.space.child`]: https://spec.matrix.org/latest/client-server-api/#mspacechild

#[cfg(feature = "unstable-msc3917")]
use std::collections::BTreeMap;

use ruma_common::{MilliSecondsSinceUnixEpoch, OwnedRoomId, OwnedServerName, OwnedUserId};
#[cfg(feature = "unstable-msc3917")]
use ruma_common::{OwnedEventId, OwnedServerSigningKeyId};
use ruma_macros::{Event, EventContent};
use serde::{Deserialize, Serialize};

/// The content of an `m.space.child` event.
///
/// The admins of a space can advertise rooms and subspaces for their space by setting
/// `m.space.child` state events.
///
/// The `state_key` is the ID of a child room or space, and the content must contain a `via` key
/// which gives a list of candidate servers that can be used to join the room.
#[cfg(not(feature = "unstable-msc3917"))]
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.space.child", kind = State, state_key_type = OwnedRoomId)]
pub struct SpaceChildEventContent {
    /// List of candidate servers that can be used to join the room.
    pub via: Vec<OwnedServerName>,

    /// Provide a default ordering of siblings in the room list.
    ///
    /// Rooms are sorted based on a lexicographic ordering of the Unicode codepoints of the
    /// characters in `order` values. Rooms with no `order` come last, in ascending numeric order
    /// of the origin_server_ts of their m.room.create events, or ascending lexicographic order of
    /// their room_ids in case of equal `origin_server_ts`. `order`s which are not strings, or do
    /// not consist solely of ascii characters in the range `\x20` (space) to `\x7E` (`~`), or
    /// consist of more than 50 characters, are forbidden and the field should be ignored if
    /// received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Space admins can mark particular children of a space as "suggested".
    ///
    /// This mainly serves as a hint to clients that that they can be displayed differently, for
    /// example by showing them eagerly in the room list. A child which is missing the `suggested`
    /// property is treated identically to a child with `"suggested": false`. A suggested child may
    /// be a room or a subspace.
    ///
    /// Defaults to `false`.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub suggested: bool,
}

/// The content of an `m.space.child` event.
///
/// The admins of a space can advertise rooms and subspaces for their space by setting
/// `m.space.child` state events.
///
/// The `state_key` is the ID of a child room or space, and the content must contain a `via` key
/// which gives a list of candidate servers that can be used to join the room.
#[cfg(feature = "unstable-msc3917")]
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.space.child", kind = State, state_key_type = OwnedRoomId)]
pub struct SpaceChildEventContent {
    /// List of candidate servers that can be used to join the room.
    pub via: Vec<OwnedServerName>,

    /// Provide a default ordering of siblings in the room list.
    ///
    /// Rooms are sorted based on a lexicographic ordering of the Unicode codepoints of the
    /// characters in `order` values. Rooms with no `order` come last, in ascending numeric order
    /// of the origin_server_ts of their m.room.create events, or ascending lexicographic order of
    /// their room_ids in case of equal `origin_server_ts`. `order`s which are not strings, or do
    /// not consist solely of ascii characters in the range `\x20` (space) to `\x7E` (`~`), or
    /// consist of more than 50 characters, are forbidden and the field should be ignored if
    /// received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Space admins can mark particular children of a space as "suggested".
    ///
    /// This mainly serves as a hint to clients that that they can be displayed differently, for
    /// example by showing them eagerly in the room list. A child which is missing the `suggested`
    /// property is treated identically to a child with `"suggested": false`. A suggested child may
    /// be a room or a subspace.
    ///
    /// Defaults to `false`.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub suggested: bool,

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

    /// The RRK of the child room. This is already provided by the child room's ID, which is
    /// the state key of the event; however, it is duplicated here so that it will be included
    /// in the signed event content.
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
    pub signatures: Option<BTreeMap<OwnedUserId, BTreeMap<OwnedServerSigningKeyId, String>>>,
}

impl SpaceChildEventContent {
    /// Creates a new `SpaceChildEventContent` with the given routing servers.
    pub fn new(via: Vec<OwnedServerName>) -> Self {
        Self {
            via,
            order: None,
            suggested: false,
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

/// An `m.space.child` event represented as a Stripped State Event with an added `origin_server_ts`
/// key.
#[derive(Clone, Debug, Event)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct HierarchySpaceChildEvent {
    /// The content of the space child event.
    pub content: SpaceChildEventContent,

    /// The fully-qualified ID of the user who sent this event.
    pub sender: OwnedUserId,

    /// The room ID of the child.
    pub state_key: String,

    /// Timestamp in milliseconds on originating homeserver when this event was sent.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,
}

#[cfg(test)]
mod tests {
    use js_int::uint;
    #[cfg(feature = "unstable-msc3917")]
    use maplit::btreemap;
    #[cfg(feature = "unstable-msc3917")]
    use ruma_common::{event_id, server_signing_key_id, user_id};
    use ruma_common::{server_name, MilliSecondsSinceUnixEpoch};
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::{HierarchySpaceChildEvent, SpaceChildEventContent};

    #[cfg(not(feature = "unstable-msc3917"))]
    #[test]
    fn space_child_serialization() {
        let content = SpaceChildEventContent {
            via: vec![server_name!("example.com").to_owned()],
            order: Some("uwu".to_owned()),
            suggested: false,
        };

        let json = json!({
            "via": ["example.com"],
            "order": "uwu",
        });

        assert_eq!(to_json_value(&content).unwrap(), json);
    }

    #[cfg(feature = "unstable-msc3917")]
    #[test]
    fn space_child_serialization() {
        let content = SpaceChildEventContent {
            via: vec![server_name!("example.com").to_owned()],
            order: Some("uwu".to_owned()),
            suggested: false,
            sender_key: Some("D67j2Q4RixFBAikBWXb7NjokkRgTDVyeHyEHjl8Ib9".into()),
            parent_event_id: Some(
                event_id!("$OSorlEHbz-xyfIaoy200IxyJAI2oTdOYFubheGwNr7c").to_owned(),
            ),
            room_root_key: Some("/ZK6paR+wBkKcazPx2xijn/0g+m2KCRqdCUZ6agzaaE".into()),
            signatures: Some(btreemap! {
                user_id!("@carl:example.com").to_owned() => btreemap! {
                    server_signing_key_id!("ed25519:rrk").to_owned() =>
                    "iI98hykGBn0MuLopSysQYY/6bSaxuSZL05yRI+20P51RtfL3mwEHxSm7x6B3TMvAauxXX5hwohk8rqiWBDBWCQ".to_owned()
                }
            }),
        };

        let json = json!({
            "via": ["example.com"],
            "order": "uwu",
            "org.matrix.msc3917.v1.sender_key": "D67j2Q4RixFBAikBWXb7NjokkRgTDVyeHyEHjl8Ib9",
            "org.matrix.msc3917.v1.parent_event_id": "$OSorlEHbz-xyfIaoy200IxyJAI2oTdOYFubheGwNr7c",
            "org.matrix.msc3917.v1.room_root_key": "/ZK6paR+wBkKcazPx2xijn/0g+m2KCRqdCUZ6agzaaE",
            "org.matrix.msc3917.v1.signatures": {
                "@carl:example.com": {
                    "ed25519:rrk": "iI98hykGBn0MuLopSysQYY/6bSaxuSZL05yRI+20P51RtfL3mwEHxSm7x6B3TMvAauxXX5hwohk8rqiWBDBWCQ"
                }
            }
        });

        assert_eq!(to_json_value(&content).unwrap(), json);
    }

    #[cfg(not(feature = "unstable-msc3917"))]
    #[test]
    fn space_child_empty_serialization() {
        let content = SpaceChildEventContent { via: vec![], order: None, suggested: false };

        let json = json!({ "via": [] });

        assert_eq!(to_json_value(&content).unwrap(), json);
    }

    #[cfg(feature = "unstable-msc3917")]
    #[test]
    fn space_child_empty_serialization() {
        let content = SpaceChildEventContent {
            via: vec![],
            order: None,
            suggested: false,
            sender_key: None,
            parent_event_id: None,
            room_root_key: None,
            signatures: None,
        };

        let json = json!({ "via": [] });

        assert_eq!(to_json_value(&content).unwrap(), json);
    }

    #[test]
    fn hierarchy_space_child_deserialization() {
        let json = json!({
            "content": {
                "via": [
                    "example.org"
                ]
            },
            "origin_server_ts": 1_629_413_349,
            "sender": "@alice:example.org",
            "state_key": "!a:example.org",
            "type": "m.space.child"
        });

        let ev = from_json_value::<HierarchySpaceChildEvent>(json).unwrap();
        assert_eq!(ev.origin_server_ts, MilliSecondsSinceUnixEpoch(uint!(1_629_413_349)));
        assert_eq!(ev.sender, "@alice:example.org");
        assert_eq!(ev.state_key, "!a:example.org");
        assert_eq!(ev.content.via, ["example.org"]);
        assert_eq!(ev.content.order, None);
        assert!(!ev.content.suggested);
    }
}
