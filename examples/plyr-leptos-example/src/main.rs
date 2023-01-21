use std::ops::Deref;

use leptos::*;
use plyr::Plyr;

#[component]
fn PlyrFrame(cx: Scope) -> impl IntoView {
    let (player_state, set_player_state) = create_signal::<Option<Plyr>>(cx, None);
    let player_node_ref = NodeRef::<HtmlElement<Div>>::new(cx);

    let view = view! {cx,
        <div>
            <div class="plyr__video-embed" _ref=player_node_ref style="width:400px">
                <iframe
                    src="https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1"
                    allowfullscreen=true
                    allowtransparency=true
                    allow="autoplay"
                ></iframe>
            </div>
            <button on:click = move |_|{player_state.with(|player|{player.as_ref().unwrap().fullscreen().enter()})}>{"fullscreen"}</button>
            <button on:click = move |_|{player_state.with(|player|{player.as_ref().unwrap().play()})}>{"play"}</button>
            <button on:click = move |_|{player_state.with(|player|{player.as_ref().unwrap().pause()})}>{"pause"}</button>
        </div>
    };
    view.on_mount(move |_| {
        set_player_state.set(Some(Plyr::new_with_html_element(
            player_node_ref.get().unwrap().into_any().deref(),
        )));
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <PlyrFrame/>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}
