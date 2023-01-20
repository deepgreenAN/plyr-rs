use crate::options::{PlyrOptions, PreviewThumbnailsOptions};

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/dist/main.js")]
extern "C" {
    // -------------------------------------------------------------------------------------------------
    // Plyr

    #[doc = "The Plyr class"]
    pub type Plyr;

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
        Plyr::set_preview_thumbnails_jsvalue(&self, &js_value);
    }
}
