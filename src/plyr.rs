#[cfg(feature = "options")]
use crate::options::{PlyrOptions, PreviewThumbnailsOptions};

#[cfg(feature = "options")]
use crate::source::SourceInfo;

use crate::{MediaType, Provider};

use js_sys::{Function, Object};
use wasm_bindgen::{prelude::*, JsCast};
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
    pub fn new_with_options_jsvalue(css_selector: &str, options: &JsValue) -> Plyr;

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with html element"]
    pub fn new_with_html_element(html_element: &HtmlElement) -> Plyr;

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with html element and options(jsvalue)"]
    pub fn new_with_html_element_and_options_jsvalue(
        html_element: &HtmlElement,
        options: &JsValue,
    ) -> Plyr;

    #[wasm_bindgen(static_method_of=Plyr)]
    #[doc = "Plyr.setup with css selector"]
    pub fn setup(css_selector: &str) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with css selector and options(jsvalue)"]
    pub fn setup_with_options_jsvalue(css_selector: &str, options: &JsValue) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with html_elements"]
    pub fn setup_with_html_elements(html_elements: Vec<HtmlElement>) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with html_elements and options(jsvalue)"]
    pub fn setup_with_html_elements_and_options_jsvalue(
        html_elements: Vec<HtmlElement>,
        options: &JsValue,
    ) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with node_list"]
    pub fn setup_with_node_list(node_list: &NodeList) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr, js_name="setup")]
    #[doc = "Plyr.setup with node_list and options(jsvalue)"]
    pub fn setup_with_node_list_and_options_jsvalue(
        node_list: &NodeList,
        options: &JsValue,
    ) -> Vec<Plyr>;

    #[wasm_bindgen(static_method_of=Plyr)]
    #[doc = "Plyr.supported with no arguments"]
    pub fn supported() -> Support;

    #[wasm_bindgen(static_method_of=Plyr)]
    #[doc = "Plyr.supported with media_type(jsvalue) and provider(jsvalue) and plays_inline"]
    pub fn supported_with_info_str(media_type: &str, provider: &str, plays_inline: bool)
        -> Support;

    // getter and setter

    #[wasm_bindgen(method, getter = isHTML5)]
    #[doc = "Getter for the `isHTML5` field of this class."]
    pub fn is_html5(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter = isEmbed)]
    #[doc = "Getter for the `isEmbed` field of this class."]
    pub fn is_embed(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `playing` field of this class."]
    pub fn playing(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `paused` field of this class."]
    pub fn paused(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `stopped` field of this class."]
    pub fn stopped(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `ended` field of this class."]
    pub fn ended(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `buffered` field of this class."]
    pub fn buffered(this: &Plyr) -> f32;

    #[wasm_bindgen(method, getter = currentTime)]
    #[doc = "Getter for the `currentTime` field of this class."]
    pub fn current_time(this: &Plyr) -> f32;

    #[wasm_bindgen(method, setter = currentTime)]
    #[doc = "Setter for the `currentTime` field of this class."]
    pub fn set_current_time(this: &Plyr, value: f32);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `seeking` field of this class."]
    pub fn seeking(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `duration` field of this class."]
    pub fn duration(this: &Plyr) -> f32;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `volume` field of this class."]
    pub fn volume(this: &Plyr) -> f32;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `volume` field of this class."]
    pub fn set_volume(this: &Plyr, value: f32);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `muted` field of this class."]
    pub fn muted(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `muted` field of this class."]
    pub fn set_muted(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter = hasAudio)]
    #[doc = "Getter for the `hasAudio` field of this class."]
    pub fn has_audio(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `speed` field of this class."]
    pub fn speed(this: &Plyr) -> f32;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `speed` field of this class."]
    pub fn set_speed(this: &Plyr, value: f32);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `quality` field of this class."]
    pub fn quality(this: &Plyr) -> u32;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `quality` field of this class."]
    pub fn set_quality(this: &Plyr, value: u32);

    #[wasm_bindgen(method, getter = loop)]
    #[doc = "Getter for the `loop` field of this class."]
    pub fn loop_(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `loop` field of this class."]
    pub fn set_loop(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter = source)]
    #[doc = "Getter for the `source` field of this class as JsValue."]
    pub fn source_jsvalue(this: &Plyr) -> JsValue;

    #[wasm_bindgen(method, setter = source)]
    #[doc = "Setter for the `seeking` field of this class with JsValue."]
    pub fn set_source_jsvalue(this: &Plyr, source: &JsValue);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `poster` field of this class."]
    pub fn poster(this: &Plyr) -> String;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `poster` field of this class."]
    pub fn set_poster(this: &Plyr, value: &str);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `autoplay` field of this class."]
    pub fn autoplay(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `autoplay` field of this class."]
    pub fn set_autoplay(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter = currentTrack)]
    #[doc = "Getter for the `currentTrac` field of this class."]
    pub fn current_track(this: &Plyr) -> usize;

    #[wasm_bindgen(method, setter = currentTrack)]
    #[doc = "Setter for the `currentTrack` field of this class."]
    pub fn set_current_track(this: &Plyr, value: usize);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `language` field of this class."]
    pub fn language(this: &Plyr) -> String;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `language` field of this class."]
    pub fn set_language(this: &Plyr, value: &str);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `pip` field of this class."]
    pub fn pip(this: &Plyr) -> bool;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `pip` field of this class."]
    pub fn set_pip(this: &Plyr, value: bool);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `ratio` field of this class as Result of JsValue."]
    pub fn ratio(this: &Plyr) -> Result<String, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `ratio` field of this class."]
    pub fn set_ratio(this: &Plyr, value: String);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `elements` field of this class."]
    pub fn elements(this: &Plyr) -> Elements;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `elements` field of this class."]
    pub fn set_elements(this: &Plyr, elements: &Elements);

    #[wasm_bindgen(method, getter=provider)]
    #[doc = "Getter for the `provider` field of this class as String."]
    fn provider_string(this: &Plyr) -> String;

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `seeking` field of this class as Result of JsValue."]
    pub fn embed(this: &Plyr) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `fullscreen` field of this class as FullscreenControl type."]
    pub fn fullscreen(this: &Plyr) -> FullscreenControl;

    // method

    #[wasm_bindgen(method)]
    #[doc = "The `play` method."]
    pub fn play(this: &Plyr);

    #[cfg(feature = "future")]
    #[wasm_bindgen(method, js_name = "play")]
    #[doc = "The `play` method that returns Promise. *This API requires the following crate features to be activated: `html5`*"]
    pub async fn play_async(this: &Plyr);

    #[wasm_bindgen(method)]
    #[doc = "The `pause` method."]
    pub fn pause(this: &Plyr);

    #[wasm_bindgen(method, js_name = "togglePlay")]
    #[doc = "The `togglePlay` method."]
    pub fn toggle_play(this: &Plyr);

    #[wasm_bindgen(method, js_name = "togglePlay")]
    #[doc = "The `togglePlay` method with toggle bool."]
    pub fn toggle_play_with_toggle(this: &Plyr, toggle: bool);

    #[wasm_bindgen(method)]
    #[doc = "The `stop` method."]
    pub fn stop(this: &Plyr);

    #[wasm_bindgen(method)]
    #[doc = "The `restart` method."]
    pub fn restart(this: &Plyr);

    #[wasm_bindgen(method, js_name = "rewind")]
    #[doc = "The `rewind` method with seek time."]
    pub fn rewind_with_seek_time(this: &Plyr, seek_time: u32);

    #[wasm_bindgen(method)]
    #[doc = "The `rewind` method."]
    pub fn rewind(this: &Plyr);

    #[wasm_bindgen(method)]
    #[doc = "The `forward` method."]
    pub fn forward(this: &Plyr);

    #[wasm_bindgen(method, js_name = "forward")]
    #[doc = "The `forward` method with seek time."]
    pub fn forward_with_seek_time(this: &Plyr, seek_time: u32);

    #[wasm_bindgen(method, js_name = "increaseVolume")]
    #[doc = "The `increaseVolume` method."]
    pub fn increase_volume(this: &Plyr);

    #[wasm_bindgen(method, js_name = "increaseVolume")]
    #[doc = "The `increaseVolume` method with step."]
    pub fn increase_volume_with_step(this: &Plyr, step: u32);

    #[wasm_bindgen(method, js_name = "decreaseVolume")]
    #[doc = "The `decreaseVolume` method."]
    pub fn decrease_volume(this: &Plyr);

    #[wasm_bindgen(method, js_name = "decreaseVolume")]
    #[doc = "The `decreaseVolume` method with step."]
    pub fn decrease_volume_with_step(this: &Plyr, step: u32);

    #[wasm_bindgen(method, js_name = "toggleCaptions")]
    #[doc = "The `toggleCaptions` method."]
    pub fn toggle_captions(this: &Plyr);

    #[wasm_bindgen(method, js_name = "toggleCaptions")]
    #[doc = "The `toggleCaptions` method with toggle bool."]
    pub fn toggle_captions_with_toggle(this: &Plyr, toggle: bool);

    #[wasm_bindgen(method)]
    #[doc = "The `airplay` method."]
    pub fn airplay(this: &Plyr);

    #[wasm_bindgen(method, js_name = "setPreviewThumbnails")]
    #[doc = "The `setPreviewThumbnails` method with source(JsValue)."]
    pub fn set_preview_thumbnails_jsvalue(this: &Plyr, source: &JsValue);

    #[wasm_bindgen(method, js_name = "toggleControls")]
    #[doc = "The `toggleControls` method."]
    pub fn toggle_controls(this: &Plyr, toggle: bool);

    #[wasm_bindgen(method)]
    #[doc = "The `on` method."]
    pub fn on(this: &Plyr, event: &str, callback: &Function);

    #[wasm_bindgen(method)]
    #[doc = "The `once` method."]
    pub fn once(this: &Plyr, event: &str, callback: &Function);

    #[wasm_bindgen(method)]
    #[doc = "The `off` method."]
    pub fn off(this: &Plyr, event: &str, callback: &Function);

    #[wasm_bindgen(method)]
    #[doc = "The `supports` method."]
    pub fn supports(this: &Plyr, supports_type: &str) -> bool;

    #[wasm_bindgen(method)]
    #[doc = "The `destroy` method."]
    pub fn destroy(this: &Plyr);

    #[wasm_bindgen(method, js_name = "destroy")]
    #[doc = "The `destroy` method with callback(`js_sys::Function`)."]
    pub fn destroy_with_callback(this: &Plyr, callback: &Function);

    #[wasm_bindgen(method, js_name = "destroy")]
    #[doc = "The `destroy` method with soft."]
    pub fn destroy_with_soft(this: &Plyr, soft: bool);

    #[wasm_bindgen(method, js_name = "destroy")]
    #[doc = "The `destroy` method with callback(`js_sys::Function`) and soft."]
    pub fn destroy_with_callback_and_soft(this: &Plyr, callback: &Function, soft: bool);

    // -------------------------------------------------------------------------------------------------
    // Support

    #[derive(Debug, Clone)]
    #[doc = "Plyr.Support"]
    pub type Support;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `api` field of this object."]
    pub fn api(this: &Plyr) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `ui` field of this object."]
    pub fn ui(this: &Plyr) -> bool;

    // -------------------------------------------------------------------------------------------------
    // Elements

    #[derive(Debug, Clone)]
    #[doc = "Plyr.Elements.buttons object"]
    pub type Buttons;

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `airplay` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn airplay(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `airplay` field of this object."]
    pub fn set_airplay(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `captions` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn captions(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `captions` field of this object."]
    pub fn set_captions(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `download` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn download(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `download` field of this object."]
    pub fn set_download(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter = fastForward, catch)]
    #[doc = "Getter for the `fast_forward` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn fast_forward(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter = fastForward)]
    #[doc = "Setter for the `fast_forward` field of this object."]
    pub fn set_fast_forward(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `fullscreen` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn fullscreen(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `fullscreen` field of this object."]
    pub fn set_fullscreen(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `mute` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn mute(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `mute` field of this object."]
    pub fn set_mute(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `pip` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn pip(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `pip` field of this object."]
    pub fn set_pip(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `play` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn play(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter = play)]
    #[doc = "Setter for the `play` field of this object."]
    pub fn set_play(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter = play, catch)]
    #[doc = "Getter for the `play` field of this object as Vec<HtmlElement>. If it failed, this method returns Err() of JsValue"]
    pub fn play_multi(this: &Buttons) -> Result<Vec<HtmlButtonElement>, JsValue>;

    #[wasm_bindgen(method, setter = play)]
    #[doc = "Setter for the `play` field of this object with Vec<HtmlElement>."]
    pub fn set_play_multi(this: &Buttons, buttons: Vec<HtmlButtonElement>);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `restart` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn restart(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `restart` field of this object."]
    pub fn set_restart(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `rewind` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn rewind(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `rewind` field of this object."]
    pub fn set_rewind(this: &Buttons, button: HtmlButtonElement);

    #[wasm_bindgen(method, getter, catch)]
    #[doc = "Getter for the `settings` field of this object. If it failed, this method returns Err() of JsValue"]
    pub fn settings(this: &Buttons) -> Result<HtmlButtonElement, JsValue>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `settings` field of this object."]
    pub fn set_settings(this: &Buttons, button: HtmlButtonElement);

    #[derive(Debug, Clone)]
    #[doc = "Plyr.Elements object"]
    pub type Elements;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `buttons` field of this object."]
    pub fn buttons(this: &Elements) -> Buttons;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `buttons` field of this object."]
    pub fn set_buttons(this: &Elements, buttons: &Buttons);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `captions` field of this object."]
    pub fn captions(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `captions` field of this object."]
    pub fn set_captions(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `container` field of this object."]
    pub fn container(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `container` field of this object."]
    pub fn set_container(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `controls` field of this object."]
    pub fn controls(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `controls` field of this object."]
    pub fn set_controls(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `fullscreen` field of this object."]
    pub fn fullscreen(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `fullscreen` field of this object."]
    pub fn set_fullscreen(this: &Elements, element: HtmlElement);

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `wrapper` field of this object."]
    pub fn wrapper(this: &Elements) -> Option<HtmlElement>;

    #[wasm_bindgen(method, setter)]
    #[doc = "Setter for the `wrapper` field of this object."]
    pub fn set_wrapper(this: &Elements, element: HtmlElement);

    // -------------------------------------------------------------------------------------------------
    // Fullscreen

    #[derive(Debug, Clone)]
    #[doc = "Plyr.FullscreenControl Object"]
    pub type FullscreenControl;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `active` field of this object."]
    pub fn active(this: &FullscreenControl) -> bool;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `enabled` field of this object."]
    pub fn enabled(this: &FullscreenControl) -> bool;

    #[wasm_bindgen(method)]
    #[doc = "The `enter` method of this object."]
    pub fn enter(this: &FullscreenControl);

    #[wasm_bindgen(method)]
    #[doc = "The `exit` method of this object."]
    pub fn exit(this: &FullscreenControl);

    #[wasm_bindgen(method)]
    #[doc = "The `toggle` method of this object."]
    pub fn toggle(this: &FullscreenControl);

    // -------------------------------------------------------------------------------------------------
    // PlyrEvent

    #[derive(Debug, Clone)]
    #[doc = "Type of Plyr.PlyrEvent.detail object"]
    pub type PlyrEventDetail;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `plyr` field of this object."]
    pub fn plyr(this: &PlyrEventDetail) -> Plyr;

    #[wasm_bindgen(extends = CustomEvent)]
    #[derive(Debug, Clone)]
    #[doc = "Plyr.PlyrEvent object"]
    pub type PlyrEvent;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `detail` field of this object."]
    pub fn detail(this: &PlyrEvent) -> PlyrEventDetail;

    // -------------------------------------------------------------------------------------------------
    // PlyrStateChangeEvent

    #[derive(Debug, Clone)]
    #[doc = "Type of Plyr.PlyrStateChangeEvent.detail object"]
    pub type PlyrStateChangeEventDetail;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `plyr` field of this object."]
    pub fn plyr(this: &PlyrStateChangeEventDetail) -> Plyr;

    #[wasm_bindgen(method, getter=code)]
    #[doc = "Getter for the `code` field of this object as i8."]
    fn code_int(this: &PlyrStateChangeEventDetail) -> i8;

    #[wasm_bindgen(extends = CustomEvent)]
    #[derive(Debug, Clone)]
    #[doc = "Plyr.PlyrStateChangeEvent object"]
    pub type PlyrStateChangeEvent;

    #[wasm_bindgen(method, getter)]
    #[doc = "Getter for the `detail` field of this object."]
    pub fn detail(this: &PlyrStateChangeEvent) -> PlyrStateChangeEventDetail;
}

impl Plyr {
    /// Plyr Constructor with options
    #[cfg(feature = "options")]
    pub fn new_with_options(css_selector: &str, options: &PlyrOptions) -> Plyr {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::new_with_options_jsvalue(css_selector, &options_js_value)
    }

    /// Plyr Constructor with HtmlElement and options
    #[cfg(feature = "options")]
    pub fn new_with_html_element_and_options(
        html_element: &HtmlElement,
        options: &PlyrOptions,
    ) -> Plyr {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::new_with_html_element_and_options_jsvalue(html_element, &options_js_value)
    }

    /// Plyr.setup with Vec<HtmlElement> and PlyrOptions
    #[cfg(feature = "options")]
    pub fn setup_with_html_elements_and_options(
        html_elements: Vec<HtmlElement>,
        options: &PlyrOptions,
    ) -> Vec<Plyr> {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::setup_with_html_elements_and_options_jsvalue(html_elements, &options_js_value)
    }

    /// Plyr.setup with NodeList and options
    #[cfg(feature = "options")]
    pub fn setup_with_node_list_and_options(
        node_list: &NodeList,
        options: &PlyrOptions,
    ) -> Vec<Plyr> {
        let options_js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::setup_with_node_list_and_options_jsvalue(node_list, &options_js_value)
    }

    /// Plyr.supported
    pub fn supported_with_info(
        media_type: &MediaType,
        provider: &Provider,
        plays_inline: bool,
    ) -> Support {
        Plyr::supported_with_info_str(&media_type.to_string(), &provider.to_string(), plays_inline)
    }

    /// Getter for the `provider` field of this class.
    pub fn provider(&self) -> Provider {
        self.provider_string()
            .try_into()
            .expect("Provider failed convert from String")
    }

    /// The setPreviewThumbnails` method of this class with PreviewThumbnailsOptions.
    #[cfg(feature = "options")]
    pub fn set_preview_thumbnails(&self, options: &PreviewThumbnailsOptions) {
        let js_value = serde_wasm_bindgen::to_value(options)
            .expect("PreviewThumbnailsOptions failed serialization");
        self.set_preview_thumbnails_jsvalue(&js_value);
    }

    /// Getter for the `source` field of this class.
    #[cfg(feature = "options")]
    pub fn source(&self) -> SourceInfo {
        serde_wasm_bindgen::from_value(self.source_jsvalue())
            .expect("SourceInfo failed deserialization")
    }

    /// Setter for the `source` field of this class.
    #[cfg(feature = "options")]
    pub fn set_source(&self, source: &SourceInfo) {
        let js_value =
            serde_wasm_bindgen::to_value(source).expect("SourceInfo failed serialization");
        self.set_source_jsvalue(&js_value);
    }
}

impl Buttons {
    /// Buttons constructor. Internally js empty object created.
    pub fn new() -> Buttons {
        let object_default = Object::default();
        object_default.unchecked_into()
    }
}

impl Default for Buttons {
    fn default() -> Self {
        Buttons::new()
    }
}

impl Elements {
    /// Elements constructor. Internally js empty object created.
    pub fn new() -> Elements {
        let object_default = Object::default();
        object_default.unchecked_into()
    }
}

impl Default for Elements {
    fn default() -> Self {
        Elements::new()
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
    /// Getter for the code field of this object.
    pub fn code(&self) -> YoutubeState {
        self.code_int().into()
    }
}
