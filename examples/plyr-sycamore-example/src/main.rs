use plyr::Plyr;
use sycamore::prelude::*;

#[component]
fn PlyrFrame<G: Html>(cx: Scope) -> View<G> {
    let player_signal: &Signal<Option<Plyr>> = create_signal(cx, None);
    let player_node_ref = create_node_ref(cx);
    on_mount(cx, move || {
        player_signal.set(Some(Plyr::new_with_element_jsvalue(
            player_node_ref.get::<DomNode>().inner_element().as_ref(),
        )))
    });

    view! {cx,
        div(class="plyr__video-embed", ref=player_node_ref, style="width:400px") {
            iframe(
                src="https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1",
                allowfullscreen=true,
                allowtransparency=true,
                allow="autoplay"
            )
        }
        button(on:click= move |_| {(*player_signal.get()).as_ref().unwrap().fullscreen().enter();}){"fullscreen"}
        button(on:click= move |_| {(*player_signal.get()).as_ref().unwrap().play()}){"play"}
        button(on:click= move |_| {(*player_signal.get()).as_ref().unwrap().pause()}){"pause"}
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        PlyrFrame()
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            App()
        }
    });
}
