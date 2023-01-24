//!
//! # Plyr-rs
//! Rust binding for [Plyr](https://github.com/sampotts/plyr).
//!
//! ## Example
//! ```rust
//! use plyr::Plyr;
//!
//! let player = Plyr::new("#player");
//! ```
//!

/// This module defines the Error used in this crate.
pub mod error;
/// This module contains event listeners for Standard, Html5, and Youtube.
pub mod events;
/// This module contains `Plyr` type and other information enum type.
pub mod plyr;

/// This module contains `Options` type like `Plyr.Options` of original Js library.  *This module requires the following crate features to be activated: `options`*
#[cfg(feature = "options")]
pub mod options;

/// This module contains `Source` type like `Plyr.Source` of original Js library. *This module requires the following crate features to be activated: `options`*
#[cfg(feature = "options")]
pub mod source;

#[cfg(feature = "options")]
use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumString};

pub use error::Error;
pub use plyr::Plyr;

#[cfg(feature = "options")]
pub use options::PlyrOptions;
// -------------------------------------------------------------------------------------------------
// Provider

/// enum like Plyr.Provider
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

/// enum like Plyr.MediaType
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
