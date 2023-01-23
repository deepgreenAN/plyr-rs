use gloo_utils::document;
use plyr::Plyr;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn main() {
    let player_element = document().query_selector("#player").unwrap().unwrap();
    let _player = Plyr::new_with_html_element(&player_element.unchecked_into::<HtmlElement>());
}
