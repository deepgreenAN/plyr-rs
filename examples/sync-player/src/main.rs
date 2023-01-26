#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(non_snake_case)]

use dioxus::core::to_owned;
use dioxus::prelude::*;
use plyr::events::{PlyrStandardEventType, PlyrYoutubeEventListener};
use plyr::Plyr;

#[derive(Props)]
struct SyncPlayerProps<'a> {
    id: String,
    playing_id: &'a UseState<String>,
}

fn SyncPlayer<'a>(cx: Scope<'a, SyncPlayerProps<'a>>) -> Element {
    let player_state: &UseState<Option<Plyr>> = use_state(&cx, || None);
    let onplay_event_listener_state: &UseState<Option<PlyrYoutubeEventListener>> =
        use_state(&cx, || None);

    use_effect(&cx, (), {
        let mut selector = "#".to_string();
        selector.push_str(&cx.props.id);

        let playing_id_state = cx.props.playing_id.clone();
        let id = cx.props.id.clone();
        to_owned![player_state, onplay_event_listener_state];

        |_| async move {
            let player = Plyr::new(&selector);
            let onplay_event_listener =
                PlyrYoutubeEventListener::new(&player, PlyrStandardEventType::playing.into(), {
                    move |_| {
                        playing_id_state.set(id.clone());
                    }
                })
                .unwrap();

            player_state.set(Some(player));
            onplay_event_listener_state.set(Some(onplay_event_listener));
        }
    });

    if cx.props.id != *cx.props.playing_id.get() {
        // 現在視聴中の動画じゃない場合
        if let Some(player) = player_state.get() {
            player.pause();
        }
    }

    cx.render(rsx! {
        div { class: "plyr__video-embed", id:"{cx.props.id}", width: "400px",
            iframe {
                src: "https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1",
                allowfullscreen: "true",
                allow: "autoplay"
            }
        },
    })
}

fn App(cx: Scope) -> Element {
    let playing_id = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        SyncPlayer{id: "player-1".to_string(), playing_id: playing_id},
        SyncPlayer{id: "player-2".to_string(), playing_id: playing_id},
        SyncPlayer{id: "player-3".to_string(), playing_id: playing_id}
    })
}

fn main() {
    dioxus::web::launch(App);
}
