#[cfg(not(feature = "unstable-msc3917"))]
use crate::{validate_delimited_id, Error};

#[cfg(feature = "unstable-msc3917")]
use crate::{validate_delimited_id, validate_id, Error};

#[cfg(not(feature = "unstable-msc3917"))]
pub fn validate(s: &str) -> Result<(), Error> {
    match s.as_bytes().first() {
        Some(b'#') => crate::room_alias_id::validate(s),
        Some(b'!') => crate::room_id::validate(s),
        _ => Err(Error::MissingLeadingSigil),
    }
}

#[cfg(feature = "unstable-msc3917")]
pub fn validate(s: &str) -> Result<(), Error> {
    let alias_result = validate_delimited_id(s, &['#']);
    let room_id_result = validate_id(s, &['!']);

    if room_id_result.is_ok() || alias_result.is_ok() {
        Ok(())
    } else {
        alias_result?;
        room_id_result
    }
}
