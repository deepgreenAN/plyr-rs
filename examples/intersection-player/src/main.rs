#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::{core::to_owned, prelude::*};
use gloo_intersection::{IntersectionObserverHandler, IntersectionObserverOptions};
use gloo_utils::document;
use plyr::Plyr;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[inline_props]
fn IntersectionPlayer(cx: Scope, id: String) -> Element {
    let player_state: &UseState<Option<Plyr>> = use_state(&cx, || None);
    let intersection_handler_state: &UseState<Option<IntersectionObserverHandler>> =
        use_state(&cx, || None);
    use_effect(&cx, (), {
        let mut selector = "#".to_string();
        selector.push_str(id);
        to_owned![player_state, intersection_handler_state];
        |_| async move {
            let player_element = document().query_selector(&selector).unwrap().unwrap();
            let player = Plyr::new_with_html_element(
                player_element
                    .clone()
                    .unchecked_into::<HtmlElement>()
                    .as_ref(),
            );
            let intersection_handler = IntersectionObserverHandler::new_with_options(
                {
                    to_owned![player];
                    move |_, _| {
                        player.pause();
                    }
                },
                &IntersectionObserverOptions::builder()
                    .threshold(&[0.0])
                    .build(),
            )
            .unwrap();
            intersection_handler.observe(player_element.as_ref());
            intersection_handler_state.set(Some(intersection_handler));
            player_state.set(Some(player));
        }
    });

    cx.render(rsx! {
        div { class: "plyr__video-embed", id:"{id}", width: "400px",
            iframe {
                src: "https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1",
                allowfullscreen: "true",
                allow: "autoplay"
            }
        },
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {height:"300px", background_color:"grey"}
        IntersectionPlayer{id: "player-1".to_string()}
        div {height:"300px", background_color:"grey"}
        IntersectionPlayer{id: "player-2".to_string()}
        div {height:"300px", background_color:"grey"}
        IntersectionPlayer{id: "player-3".to_string()}
        div {height:"300px", background_color:"grey"}
    })
}

fn main() {
    dioxus::web::launch(App);
}
