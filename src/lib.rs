pub mod error;
pub mod events;
pub mod plyr;

#[cfg(feature = "options")]
pub mod options;

#[cfg(feature = "options")]
pub mod source;

#[cfg(feature = "options")]
use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumString};

pub use error::Error;
pub use plyr::Plyr;
// -------------------------------------------------------------------------------------------------
// Provider

/// plyr.Provider
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Display, EnumString)]
#[cfg_attr(feature = "options", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "options", serde(into = "String", try_from = "String"))]
pub enum Provider {
    html5,
    youtube,
    vimeo,
}

impl From<Provider> for String {
    fn from(value: Provider) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for Provider {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .parse()
            .map_err(|_| "Provider enum failed Parse".to_string())
    }
}

// -------------------------------------------------------------------------------------------------
// MediaType

/// plyr.MediaType
#[allow(non_camel_case_types)]
#[derive(Clone, Display, Debug, EnumString)]
#[cfg_attr(feature = "options", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "options", serde(into = "String", try_from = "String"))]
pub enum MediaType {
    audio,
    video,
}

impl From<MediaType> for String {
    fn from(value: MediaType) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for MediaType {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .parse()
            .map_err(|_| "MediaType enum failed Parse".to_string())
    }
}
