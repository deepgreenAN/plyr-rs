#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::core::to_owned;
use dioxus::prelude::*;
use plyr::events::{PlyrStandardEventType, PlyrYoutubeEventListener};
use plyr::Plyr;

#[inline_props]
fn PlyrFrame(cx: Scope, id: String) -> Element {
    let player_state: &UseState<Option<Plyr>> = use_state(&cx, || None);
    let player_description_state = use_state(&cx, || "".to_string());
    let event_listeners_state: &UseState<Option<Vec<PlyrYoutubeEventListener>>> =
        use_state(&cx, || None);

    use_effect(&cx, (), |_| {
        to_owned![
            player_state,
            player_description_state,
            event_listeners_state
        ];
        let mut selector = "#".to_string();
        selector.push_str(&id);
        async move {
            let player = Plyr::new(&selector);
            let event_listeners = vec![
                PlyrYoutubeEventListener::new(&player, PlyrStandardEventType::playing.into(), {
                    to_owned![player_description_state];
                    move |_| {
                        player_description_state.set("playing".to_string());
                    }
                })
                .unwrap(),
                PlyrYoutubeEventListener::new(&player, PlyrStandardEventType::pause.into(), {
                    to_owned![player_description_state];
                    move |_| {
                        player_description_state.set("pause".to_string());
                    }
                })
                .unwrap(),
            ];

            player_state.set(Some(player));
            event_listeners_state.set(Some(event_listeners));
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
        div {
            [format!("{}", player_description_state.get())]
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
