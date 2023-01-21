#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use plyr::Plyr;

#[inline_props]
fn PlyrFrame(cx: Scope) -> Element {
    let id = "player".to_string();

    let player_state = {
        let mut id = id.clone();
        id.insert_str(0, "#");
        use_future(&cx, (), |_| async move { Plyr::new(&id) })
    };

    let fullscren = move |_| {
        player_state.value().unwrap().fullscreen().enter();
    };

    let play = move |_| {
        player_state.value().unwrap().play();
    };

    let pause = move |_| {
        player_state.value().unwrap().pause();
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
            onclick: fullscren,
            "フルスクリーン"
        },
        button {
            onclick: play,
            "再生"
        },
        button {
            onclick: pause,
            "一時停止"
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        PlyrFrame{},
    })
}

fn main() {
    dioxus::web::launch(App);
}
