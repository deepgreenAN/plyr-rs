use crate::options::PreviewThumbnailsOpts;
use crate::{MediaType, Provider};

use serde::Serialize;
use strum_macros::Display;
use typed_builder::TypedBuilder;

// -------------------------------------------------------------------------------------------------
// SourceInfo

#[derive(TypedBuilder, Serialize, Debug)]
pub struct Source {
    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    #[builder(default, setter(into, strip_option))]
    pub type_: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub provider: Option<Provider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub size: Option<u32>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Clone, Debug, Display)]
#[serde(into = "String")]
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

#[derive(TypedBuilder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub kind: TrackKind,
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub src_lang: Option<String>,

    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub default: Option<bool>,
}

#[derive(TypedBuilder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfo {
    #[serde(rename = "type")]
    pub type_: MediaType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub title: Option<String>,

    pub sources: Vec<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub poster: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tracks: Option<Vec<Track>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub preview_thumbnails: Option<PreviewThumbnailsOpts>,
}
