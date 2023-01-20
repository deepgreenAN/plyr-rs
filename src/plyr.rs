use crate::options::{PlyrOptions, PreviewThumbnailsOptions};
use crate::source::SourceInfo;

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/dist/main.js")]
extern "C" {
    // -------------------------------------------------------------------------------------------------
    // Plyr

    #[doc = "The Plyr class"]
    pub type Plyr;

    // method

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor"]
    pub fn new(css_selector: &str) -> Plyr;

    #[wasm_bindgen(constructor)]
    #[doc = "Plyr constructor with options"]
    pub fn new_with_jsvalue(css_selector: &str, opts: &JsValue) -> Plyr;

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
    pub fn on(this: &Plyr, event: &str, function: &Function);

    #[wasm_bindgen(method)]
    pub fn once(this: &Plyr, event: &str, function: &Function);

    #[wasm_bindgen(method)]
    pub fn off(this: &Plyr, event: &str, function: &Function);

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

    // #[wasm_bindgen(method, getter = source)]
    // pub fn source_jsvalue(this: &Plyr) -> JsValue;

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

    // -------------------------------------------------------------------------------------------------
    // Fullscreen
    #[wasm_bindgen(js_namespace = ["Plyr"])]
    #[doc = "Plyr.FullscreenControl"]
    pub type FullscreenControl;

}

impl Plyr {
    /// Plyr Constructor with PlyrOptions
    pub fn new_with_options(css_selector: &str, options: &PlyrOptions) -> Plyr {
        let js_value =
            serde_wasm_bindgen::to_value(options).expect("PlyrOptions failed serialization");
        Plyr::new_with_jsvalue(css_selector, &js_value)
    }

    /// Plyr.setPreviewThumbnails with PreviewThumbnailsOptions
    pub fn set_preview_thumbnails(&self, options: &PreviewThumbnailsOptions) {
        let js_value = serde_wasm_bindgen::to_value(options)
            .expect("PreviewThumbnailsOptions failed serialization");
        self.set_preview_thumbnails_jsvalue(&js_value);
    }

    pub fn set_source(&self, source: SourceInfo) {
        let js_value =
            serde_wasm_bindgen::to_value(&source).expect("SourceInfo failed serialization");
        self.set_source_jsvalue(&js_value);
    }
}
