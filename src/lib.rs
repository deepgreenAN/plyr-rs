pub mod error;
pub mod events;
pub mod options;
pub mod plyr;
pub mod source;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub use error::Error;
pub use options::PlyrOptions;
pub use plyr::Plyr;
// -------------------------------------------------------------------------------------------------
// Provider

/// plyr.Provider
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug, Display, EnumString)]
#[serde(into = "String", try_from = "String")]
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
#[derive(Serialize, Deserialize, Clone, Display, Debug, EnumString)]
#[serde(into = "String", try_from = "String")]
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
