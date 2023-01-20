use serde::Serialize;
use typed_builder::TypedBuilder;

// -------------------------------------------------------------------------------------------------
// PlyrOpts

/// Plyrのオプション
#[derive(TypedBuilder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlyrOpts {
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
    pub keybord: Option<KeyBoardOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tooltips: Option<ToolTipOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub duration: Option<Option<i32>>,

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
    pub captions: Option<CaptionOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub fullscreen: Option<FullscreenOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ratio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub strage: Option<StrageOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub speed: Option<SpeedOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub quality: Option<QualityOpts>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "loop")]
    #[builder(default, setter(into, strip_option))]
    pub loop_: Option<LoopOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub ads: Option<AdOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub preview_thumbnails: Option<PreviewThumbnailsOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub media_metadata: Option<MediaMetadataOpts>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub markers: Option<MarkerOpts>,
}

// -------------------------------------------------------------------------------------------------
// KeyBoardOpts

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct KeyBoardOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub focused: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub global: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
// ToolTipOpts

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct ToolTipOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub controls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub seek: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
// CaptionOpts

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct CaptionOpts {
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
// FullscreenOpts

#[derive(TypedBuilder, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FullscreenOpts {
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
// StrageOpts

#[derive(TypedBuilder, Serialize, Debug, Default)]
pub struct StrageOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub key: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// SpeedOpts

#[derive(TypedBuilder, Serialize, Debug)]
pub struct SpeedOpts {
    pub selected: f32,
    pub options: Vec<f32>,
}

// -------------------------------------------------------------------------------------------------
// QualityOpts

#[derive(TypedBuilder, Serialize, Debug)]
pub struct QualityOpts {
    pub default: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub forced: Option<bool>,

    pub options: Vec<u32>,
}

// -------------------------------------------------------------------------------------------------
// LoopOpts

#[derive(TypedBuilder, Serialize, Debug)]
pub struct LoopOpts {
    pub active: bool,
}

// -------------------------------------------------------------------------------------------------
// AdOpts

#[derive(TypedBuilder, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdOpts {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub publisher_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tag_url: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// PreviewThumbnails

#[derive(TypedBuilder, Serialize, Debug)]
pub struct PreviewThumbnailsOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub src: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// MediaMetadataOpts

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MediaMetadataOpts {
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
// MarkerOpts

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MarkerOpts {
    pub enabled: bool,
    pub points: Vec<MarkersPoints>,
}

#[derive(TypedBuilder, Serialize, Debug)]
pub struct MarkersPoints {
    pub time: u32,
    pub label: String,
}
