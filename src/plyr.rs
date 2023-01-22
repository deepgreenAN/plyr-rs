use crate::options::{PlyrOptions, PreviewThumbnailsOptions};
use crate::source::SourceInfo;
use crate::{MediaType, Provider};

use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, HtmlButtonElement, HtmlElement, NodeList};

#[cfg_attr(feature = "cdn", wasm_bindgen(module = "/dist_cdn/main.js"))]
#[cfg_attr(not(feature = "cdn"), wasm_bindgen(module = "/dist/main.js"))]
extern "C" {
    // -------------------------------------------------------------------------------------------------
    // Plyr
    #[derive(Debug, Clone)]
    #[doc = "The Plyr class"]
    pub type Plyr;

    // constructor and static method
    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with css selector"]
    pub fn new(css_selector: &str) -> Plyr;

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with options(jsvalue)"]
    fn new_with_options_jsvalue(css_selector: &str, options: &JsValue) -> Plyr;

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with element(jsvalue)"]
    fn new_with_element_jsvalue(element_jsvalue: &JsValue) -> Plyr;

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with element and options(jsvalue)"]
    fn new_with_element_jsvalue_and_options_jsvalue(
        element_jsvalue: &JsValue,
        options: &JsValue,
    ) -> Plyr;

    #[wasm_bindgen(static_method_of=Plyr)]
    #[doc = "Plyr.setup with css selector"]
    pub fn setup(css_selector: &str) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with css selector and options(jsvalue)"]
    fn setup_with_options_jsvalue(css_selector: &str, options: &JsValue) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with html_elements(Vec<JsValue>)"]
    fn setup_with_html_elements_jsvalue(html_elements: Vec<JsValue>) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with html_elements(vector of jsvalue) and Options(jsvalue)"]
    fn setup_with_html_elements_jsvalue_and_options_jsvalue(
        html_elements: Vec<JsValue>,
        options: &JsValue,
    ) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with node_list(jsvalue)"]
    fn setup_with_node_list_jsvalue(node_list: &JsValue) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with node_list(jsvalue) and options(jsvalue)"]
    fn setup_with_node_list_jsvalue_and_options_jsvalue(
        node_list: &JsValue,
        options: &JsValue,
    ) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr)]
    #[doc = "Plyr.supported with no arguments"]
    pub fn supported() -> Support;

    #[wasm_bindgen(static_method_of=Plyr)]
    #[doc = "Plyr.supported with media_type(jsvalue) and provider(jsvalue) and palays_inline"]
    fn supported_with_info_jsvalues(
        media_type: &JsValue,
        provider: &JsValue,
        plays_inline: bool,
    ) -> Support;

    // getter and setter

    #[wasm_bindgen(method, getter = isHTML5)]
    pub fn is_html5(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter = isEmbed)]
    pub fn is_embed(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn playing(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn paused(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn stopped(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn ended(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn buffered(this: &Plyr) -> f32;

    #[wasm_bindgen(method, getter = currentTime)]
    pub fn current_time(this: &Plyr) -> f32;

    #[wasm_bindgen(method, setter = currentTime)]
    pub fn set_current_time(this: &Plyr, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn seeking(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn duration(this: &Plyr) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn volume(this: &Plyr) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_volume(this: &Plyr, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn muted(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_muted(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter = hasAudio)]
    pub fn has_audio(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn speed(this: &Plyr) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_speed(this: &Plyr, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn quality(this: &Plyr) -> u32;

    #[wasm_bindgen(method, setter)]
    pub fn set_quality(this: &Plyr, value: u32);

    #[wasm_bindgen(method, getter = loop)]
    pub fn loop_(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_loop(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter = source)]
    fn source_jsvalue(this: &Plyr) -> JsValue;

    #[wasm_bindgen(method, setter = source)]
    pub fn set_source_jsvalue(this: &Plyr, source: &JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn poster(this: &Plyr) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_poster(this: &Plyr, value: &str);

    #[wasm_bindgen(method, getter)]
    pub fn autoplay(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_autoplay(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter = currentTrack)]
    pub fn current_track(this: &Plyr) -> usize;

    #[wasm_bindgen(method, setter = currentTrack)]
    pub fn set_current_track(this: &Plyr, value: usize);

    #[wasm_bindgen(method, getter)]
    pub fn language(this: &Plyr) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_language(this: &Plyr, value: &str);

    #[wasm_bindgen(method, getter)]
    pub fn pip(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_pip(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn ratio(this: &Plyr) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_ratio(this: &Plyr, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn elements(this: &Plyr) -> Elements;

    #[wasm_bindgen(method, setter)]
    pub fn set_elements(this: &Plyr, elements: &Elements);

    #[wasm_bindgen(method, getter=provider)]
    fn provider_string(this: &Plyr) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn embed(this: &Plyr) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn fullscreen(this: &Plyr) -> FullscreenControl;

    // method

    #[wasm_bindgen(method)]
    pub fn play(this: &Plyr);

    #[wasm_bindgen(method)]
    pub fn pause(this: &Plyr);

    #[wasm_bindgen(method, js_name = "togglePlay")]
    pub fn toggle_play(this: &Plyr);

    #[wasm_bindgen(method, js_name = "togglePlay")]
    pub fn toggle_play_with_toggle(this: &Plyr, toggle: bool);

    #[wasm_bindgen(method)]
    pub fn stop(this: &Plyr);

    #[wasm_bindgen(method)]
    pub fn restart(this: &Plyr);

    #[wasm_bindgen(method, js_name = "rewind")]
    pub fn rewind_with_seek_time(this: &Plyr, seek_time: u32);

    #[wasm_bindgen(method)]
    pub fn rewind(this: &Plyr);

    #[wasm_bindgen(method)]
    pub fn forward(this: &Plyr);

    #[wasm_bindgen(method, js_name = "forward")]
    pub fn forward_with_seek_time(this: &Plyr, seek_time: u32);

    #[wasm_bindgen(method, js_name = "increaseVolume")]
    pub fn increase_volume(this: &Plyr);

    #[wasm_bindgen(method, js_name = "increaseVolume")]
    pub fn increase_volume_with_step(this: &Plyr, step: u32);

    #[wasm_bindgen(method, js_name = "decreaseVolume")]
    pub fn decrease_volume(this: &Plyr);

    #[wasm_bindgen(method, js_name = "decreaseVolume")]
    pub fn decrease_volume_with_step(this: &Plyr, step: u32);

    #[wasm_bindgen(method, js_name = "toggleCaptions")]
    pub fn toggle_captions(this: &Plyr);

    #[wasm_bindgen(method, js_name = "toggleCaptions")]
    pub fn toggle_captions_with_toggle(this: &Plyr, toggle: bool);

    #[wasm_bindgen(method)]
    pub fn airplay(this: &Plyr);

    #[wasm_bindgen(method, js_name = "setPreviewThumbnails")]
    pub fn set_preview_thumbnails_jsvalue(this: &Plyr, source: &JsValue);

    #[wasm_bindgen(method, js_name = "toggleControls")]
    pub fn toggle_controls(this: &Plyr, toggle: bool);

    #[wasm_bindgen(method)]
    pub fn on(this: &Plyr, event: &str, callback: &Function);

    #[wasm_bindgen(method)]
    pub fn once(this: &Plyr, event: &str, callback: &Function);

    #[wasm_bindgen(method)]
    pub fn off(this: &Plyr, event: &str, callback: &Function);

    #[wasm_bindgen(method)]
    pub fn supports(this: &Plyr, supports_type: &str) -> bool;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Plyr);

    #[wasm_bindgen(method, js_name = "destroy")]
    pub fn destroy_with_callback(this: &Plyr, callback: &Function);

    #[wasm_bindgen(method, js_name = "destroy")]
    pub fn destroy_with_soft(this: &Plyr, soft: bool);

    #[wasm_bindgen(method, js_name = "destroy")]
    pub fn destroy_with_callback_and_soft(this: &Plyr, callback: &Function, soft: bool);

    // -------------------------------------------------------------------------------------------------
    // Support

    #[derive(Debug, Clone)]
    #[doc = "Plyr.Support"]
    pub type Support;

    #[wasm_bindgen(method, getter)]
    pub fn api(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn ui(this: &Plyr) -> bool;

    // -------------------------------------------------------------------------------------------------
    // Elements

    #[derive(Debug, Clone)]
    #[doc = "Plyr.Elements.buttons"]
    pub type Buttons;

    #[wasm_bindgen(method, getter)]
    pub fn airplay(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_airplay(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn captions(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_captions(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn download(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_download(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter = fastForward)]
    pub fn fast_forward(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter = fastForward)]
    pub fn set_fast_forward(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn fullscreen(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_fullscreen(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn mute(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_mute(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn pip(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_pip(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter = play)]
    pub fn play(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter = play)]
    pub fn set_play(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter = play)]
    pub fn play_multi(this: &Buttons) -> Option<Vec<HtmlButtonElement>>;

    #[wasm_bindgen(method, setter = play)]
    pub fn set_play_multi(this: &Buttons, buttons: Vec<HtmlButtonElement>);

    #[wasm_bindgen(method, getter)]
    pub fn restart(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_restart(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn rewind(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_rewind(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter)]
    pub fn settings(this: &Buttons) -> Option<HtmlButtonElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_settings(this: &Buttons, button: HtmlButtonElement);

    #[derive(Debug, Clone)]
    #[doc = "Plyr.Elements"]
    pub type Elements;

    #[wasm_bindgen(method, getter)]
    pub fn buttons(this: &Elements) -> Buttons;

    #[wasm_bindgen(method, setter)]
    pub fn set_buttons(this: &Elements, buttons: &Buttons);

    #[wasm_bindgen(method, getter)]
    pub fn captions(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_captions(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    pub fn container(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_container(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    pub fn controls(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_controls(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    pub fn fullscreen(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_fullscreen(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    pub fn wrapper(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    pub fn set_wrapper(this: &Elements, element: HtmlElement);

    // -------------------------------------------------------------------------------------------------
    // Fullscreen

    #[derive(Debug, Clone)]
    #[doc = "Plyr.FullscreenControl"]
    pub type FullscreenControl;

    #[wasm_bindgen(method, getter)]
    pub fn active(this: &FullscreenControl) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn enabled(this: &FullscreenControl) -> bool;

    #[wasm_bindgen(method)]
    pub fn enter(this: &FullscreenControl);

    #[wasm_bindgen(method)]
    pub fn exit(this: &FullscreenControl);

    #[wasm_bindgen(method)]
    pub fn toggle(this: &FullscreenControl);

    // -------------------------------------------------------------------------------------------------
    // PlyrEvent

    #[derive(Debug, Clone)]
    #[doc = "Type of Plyr.PlyrEvent.detail"]
    pub type PlyrEventDetail;

    #[wasm_bindgen(method, getter)]
    pub fn plyr(this: &PlyrEventDetail) -> Plyr;

    #[wasm_bindgen(extends = CustomEvent)]
    #[derive(Debug, Clone)]
    #[doc = "Plyr.PlyrEvent"]
    pub type PlyrEvent;

    #[wasm_bindgen(method, getter)]
    pub fn detail(this: &PlyrEvent) -> PlyrEventDetail;

    // -------------------------------------------------------------------------------------------------
    // PlyeStateChangeEvent

    #[derive(Debug, Clone)]
    #[doc = "Type of Plyr.PlyrStateChangeEvent.detail"]
    pub type PlyrStateChangeEventDetail;

    #[wasm_bindgen(method, getter)]
    pub fn plyr(this: &PlyrStateChangeEventDetail) -> Plyr;

    #[wasm_bindgen(method, getter=code)]
    fn code_int(this: &PlyrStateChangeEventDetail) -> i8;

    #[wasm_bindgen(extends = CustomEvent)]
    #[derive(Debug, Clone)]
    #[doc = "Plyr.PlyrStateChangeEvent"]
    pub type PlyrStateChangeEvent;

    #[wasm_bindgen(method, getter)]
    pub fn detail(this: &PlyrStateChangeEvent) -> PlyrStateChangeEventDetail;
}

impl Plyr {
    /// Plyr Constructor with options
    pub fn new_with_options(css_selector: &str, options: &PlyrOptions) -> Plyr {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::new_with_options_jsvalue(css_selector, &options_js_value)
    }

    /// Plyr Constructor with HtmlElement
    pub fn new_with_html_element(html_element: &HtmlElement) -> Plyr {
        Plyr::new_with_element_jsvalue(html_element.as_ref())
    }

    /// Plyr Constructor with HtmlElement and options
    pub fn new_with_html_element_and_options(
        html_element: &HtmlElement,
        options: &PlyrOptions,
    ) -> Plyr {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::new_with_element_jsvalue_and_options_jsvalue(html_element.as_ref(), &options_js_value)
    }

    /// Plyr.setup with vector of HtmlElement
    pub fn setup_with_html_elements(html_elements: Vec<HtmlElement>) -> Vec<Plyr> {
        let html_elements_jsvalue: Vec<JsValue> = html_elements
            .into_iter()
            .map(|element| element.into())
            .collect();
        Plyr::setup_with_html_elements_jsvalue(html_elements_jsvalue)
    }

    /// Plyr.setup with Vec<HtmlElement> and PlyrOptions
    pub fn setup_with_html_elements_and_options(
        html_elements: Vec<HtmlElement>,
        options: &PlyrOptions,
    ) -> Vec<Plyr> {
        let html_elements_jsvalue: Vec<JsValue> = html_elements
            .into_iter()
            .map(|element| element.into())
            .collect();
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::setup_with_html_elements_jsvalue_and_options_jsvalue(
            html_elements_jsvalue,
            &options_js_value,
        )
    }

    /// Plyr.setup with NodeList
    pub fn setup_with_node_list(node_list: &NodeList) -> Vec<Plyr> {
        Plyr::setup_with_node_list_jsvalue(node_list.as_ref())
    }

    /// Plyr.setup with NodeList and options
    pub fn setup_with_node_list_and_options(
        node_list: &NodeList,
        options: &PlyrOptions,
    ) -> Vec<Plyr> {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::setup_with_node_list_jsvalue_and_options_jsvalue(
            node_list.as_ref(),
            &options_js_value,
        )
    }

    /// Plyr.supported
    pub fn supported_with_info(
        media_type: &MediaType,
        provider: &Provider,
        plays_inline: bool,
    ) -> Support {
        let media_type_jsvalue =
            serde_wasm_bindgen::to_value(media_type).expect("MediaType failed serialization");
        let provider_jsvalue =
            serde_wasm_bindgen::to_value(provider).expect("Provider failed serialization");
        Plyr::supported_with_info_jsvalues(&media_type_jsvalue, &provider_jsvalue, plays_inline)
    }

    /// Plyr.provider
    pub fn provider(&self) -> Provider {
        self.provider_string()
            .try_into()
            .expect("Provider failed convert from String")
    }

    /// Plyr.setPreviewThumbnails with PreviewThumbnailsOptions
    pub fn set_preview_thumbnails(&self, options: &PreviewThumbnailsOptions) {
        let js_value = serde_wasm_bindgen::to_value(options)
            .expect("PreviewThumbnailsOptions failed serialization");
        self.set_preview_thumbnails_jsvalue(&js_value);
    }

    /// get Plyr.source
    pub fn source(&self) -> SourceInfo {
        serde_wasm_bindgen::from_value(self.source_jsvalue())
            .expect("SourceInfo failed deserialization")
    }

    /// set Plyr.source
    pub fn set_source(&self, source: &SourceInfo) {
        let js_value =
            serde_wasm_bindgen::to_value(source).expect("SourceInfo failed serialization");
        self.set_source_jsvalue(&js_value);
    }
}

impl Buttons {
    /// Buttons constructor. Internally js empty object created.
    pub fn new() -> Buttons {
        let js_value_default = JsValue::default();
        js_value_default.into()
    }
}

impl Elements {
    /// Elements constructor. Internally js empty object created.
    pub fn new() -> Elements {
        let js_value_default = JsValue::default();
        js_value_default.into()
    }
}

pub enum YoutubeState {
    Unstarted,
    Ended,
    Playing,
    Paused,
    Buffering,
    Cued,
}

impl From<i8> for YoutubeState {
    fn from(value: i8) -> Self {
        match value {
            -1 => Self::Unstarted,
            0 => Self::Ended,
            1 => Self::Playing,
            2 => Self::Paused,
            3 => Self::Buffering,
            5 => Self::Cued,
            _ => panic!("Undefined Youtube State"),
        }
    }
}

impl PlyrStateChangeEventDetail {
    pub fn code(&self) -> YoutubeState {
        self.code_int().into()
    }
}
