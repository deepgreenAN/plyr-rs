use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

// -------------------------------------------------------------------------------------------------
// PlyrOpts

/// Plyrのオプション
#[derive(TypedBuilder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlyrOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub debug: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub controls: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub settings: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub load_sprite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub icon_url: Option<Option<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub icon_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub blank_video: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub autoplay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub autopause: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub seek_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub volume: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub muted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub click_to_play: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub disable_context_menu: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub hide_controls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub reset_on_end: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub keyboard: Option<KeyBoardOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tooltips: Option<ToolTipOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub duration: Option<Option<f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub display_duration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub invert_time: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub toggle_invert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub captions: Option<CaptionOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub fullscreen: Option<FullscreenOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ratio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub storage: Option<StorageOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub speed: Option<SpeedOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub quality: Option<QualityOptions>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "loop")]
    #[builder(default, setter(into, strip_option))]
    pub loop_: Option<LoopOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ads: Option<AdOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub preview_thumbnails: Option<PreviewThumbnailsOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub media_metadata: Option<MediaMetadataOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub markers: Option<MarkerOptions>,
}

// -------------------------------------------------------------------------------------------------
// KeyBoardOptions

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct KeyBoardOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub focused: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub global: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
// ToolTipOptions

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct ToolTipOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub controls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub seek: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
// CaptionOptions

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct CaptionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub update: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// FullscreenOptions

#[derive(TypedBuilder, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FullscreenOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub fallback: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub allow_audio: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ios_native: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub container: Option<Option<String>>,
}

// -------------------------------------------------------------------------------------------------
// StorageOptions

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct StorageOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub key: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// SpeedOptions

#[derive(TypedBuilder, Serialize, Debug)]
pub struct SpeedOptions {
    pub selected: f32,
    pub options: Vec<f32>,
}

// -------------------------------------------------------------------------------------------------
// QualityOptions

#[derive(TypedBuilder, Serialize, Debug)]
pub struct QualityOptions {
    pub default: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub forced: Option<bool>,

    pub options: Vec<u32>,
}

// -------------------------------------------------------------------------------------------------
// LoopOptions

#[derive(TypedBuilder, Serialize, Debug)]
pub struct LoopOptions {
    pub active: bool,
}

// -------------------------------------------------------------------------------------------------
// AdOptions

#[derive(TypedBuilder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdOptions {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub publisher_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tag_url: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// PreviewThumbnailsOptions

#[derive(TypedBuilder, Serialize, Deserialize, Debug)]
pub struct PreviewThumbnailsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub src: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// MediaMetadataOptions

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MediaMetadataOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub artist: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub album: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub artwork: Option<Vec<MediaMetadataArtwork>>,
}

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MediaMetadataArtwork {
    pub src: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub sizes: Option<String>,

    #[serde(rename = "type")]
    pub type_: String,
}

// -------------------------------------------------------------------------------------------------
// MarkerOptions

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MarkerOptions {
    pub enabled: bool,
    pub points: Vec<MarkersPoints>,
}

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MarkersPoints {
    pub time: u32,
    pub label: String,
}
