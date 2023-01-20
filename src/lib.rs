pub mod options;
pub mod plyr;
pub mod source;

use serde::Serialize;
use strum_macros::Display;

pub use options::PlyrOptions;
pub use plyr::Plyr;
// -------------------------------------------------------------------------------------------------
// Provider

/// plyr.Provider
#[allow(non_camel_case_types)]
#[derive(Serialize, Clone, Debug, Display)]
#[serde(into = "String")]
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

// -------------------------------------------------------------------------------------------------
// MediaType

/// plyr.MediaType
#[allow(non_camel_case_types)]
#[derive(Serialize, Clone, Display, Debug)]
#[serde(into = "String")]
pub enum MediaType {
    audio,
    video,
}

impl From<MediaType> for String {
    fn from(value: MediaType) -> Self {
        value.to_string()
    }
}
