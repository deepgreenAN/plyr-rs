#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::core::to_owned;
use dioxus::prelude::*;
use plyr::Plyr;

#[inline_props]
fn PlyrFrame(cx: Scope, id: String) -> Element {
    let player_state: &UseState<Option<Plyr>> = use_state(&cx, || None);

    use_effect(&cx, (), |_| {
        to_owned![player_state];
        let mut selector = "#".to_string();
        selector.push_str(id);
        async move {
            let player = Plyr::new(&selector);
            player_state.set(Some(player));
        }
    });

    let onclick_async = move |_| {
        cx.spawn({
            to_owned![player_state];
            async move {
                (*player_state.current())
                    .as_ref()
                    .unwrap()
                    .play_async()
                    .await;
            }
        });
    };

    cx.render(rsx! {
        video {
            id: "{id}",
            playsinline: "true",
            controls: "true",
            source {
                src: "/assets/sample_movie.mp4",
                "type": "video/mp4"
            }
        }
        button {
            onclick: move |_| {player_state.get().as_ref().unwrap().fullscreen().enter();},
            "fullscreen"
        },
        button {
            onclick: onclick_async,
            "play"
        },
        button {
            onclick: move |_| {player_state.get().as_ref().unwrap().pause();},
            "pause"
        }
    })
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        PlyrFrame{id: "player".to_string()}
    })
}
fn main() {
    dioxus::web::launch(App);
}
