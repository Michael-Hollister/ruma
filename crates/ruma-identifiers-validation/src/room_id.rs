#[cfg(not(feature = "unstable-msc3917"))]
use crate::{validate_delimited_id, Error};

#[cfg(feature = "unstable-msc3917")]
use crate::{validate_id, Error};

#[cfg(not(feature = "unstable-msc3917"))]
pub fn validate(s: &str) -> Result<(), Error> {
    validate_id(s, b'!')
}

#[cfg(feature = "unstable-msc3917")]
pub fn validate(s: &str) -> Result<(), Error> {
    validate_id(s, &['!'])
}
