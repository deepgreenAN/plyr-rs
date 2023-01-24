//!
//! SourceInfo is the type for the setter and getter of Plyr.source. This allows changing the player source and type on the fly.
//!

use crate::options::PreviewThumbnailsOptions;
use crate::{MediaType, Provider};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typed_builder::TypedBuilder;

// -------------------------------------------------------------------------------------------------
// SourceInfo

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
pub struct Source {
    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type", default)]
    #[builder(default, setter(strip_option))]
    pub type_: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub provider: Option<Provider>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub size: Option<u32>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug, Display, EnumString)]
#[serde(into = "String", try_from = "String")]
pub enum TrackKind {
    subtitles,
    captions,
    descriptions,
    chapters,
    metadata,
}

impl From<TrackKind> for String {
    fn from(value: TrackKind) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for TrackKind {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .parse()
            .map_err(|_| "TrackKind enum failed parse".to_string())
    }
}

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub kind: TrackKind,
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub src_lang: Option<String>,

    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub default: Option<bool>,
}

/// This struct is used for the setter and getter of Plyr.source
#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfo {
    #[serde(rename = "type")]
    pub type_: MediaType,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,

    pub sources: Vec<Source>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub poster: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub tracks: Option<Vec<Track>>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[builder(default, setter(strip_option))]
    pub preview_thumbnails: Option<PreviewThumbnailsOptions>,
}
