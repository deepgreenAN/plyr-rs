#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use plyr::Plyr;

#[inline_props]
fn PlyrFrame(cx: Scope, id: String) -> Element {
    let player_state = {
        let mut selector = "#".to_string();
        selector.push_str(&id);
        use_future(
            &cx,
            &selector,
            |selector| async move { Plyr::new(&selector) },
        )
    };
    cx.render(rsx! {
        div { class: "plyr__video-embed", id:"{id}", width: "400px",
            iframe {
                src: "https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1",
                allowfullscreen: "true",
                allow: "autoplay"
            }
        },
        button {
            onclick: move |_| {player_state.value().unwrap().fullscreen().enter();},
            "fullscreen"
        },
        button {
            onclick: move |_| {player_state.value().unwrap().play();},
            "play"
        },
        button {
            onclick: move |_| {player_state.value().unwrap().pause();},
            "pause"
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        PlyrFrame{id: "player".to_string()},
    })
}

fn main() {
    dioxus::web::launch(App);
}
