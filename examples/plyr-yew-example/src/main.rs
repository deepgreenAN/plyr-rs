use plyr::Plyr;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
fn PlyrFrame() -> Html {
    let player_state: UseStateHandle<Option<Plyr>> = use_state(|| None);
    let player_node_ref = use_node_ref();
    {
        let player_state = player_state.clone();
        let player_node_ref = player_node_ref.clone();
        use_effect_with_deps(
            move |_| {
                let player =
                    Plyr::new_with_html_element(&player_node_ref.cast::<HtmlElement>().unwrap());
                player_state.set(Some(player));
            },
            (),
        );
    }

    let fullscreen = {
        let player_state = player_state.clone();
        Callback::from(move |_| {
            player_state.as_ref().unwrap().fullscreen().enter();
        })
    };

    let play = {
        let player_state = player_state.clone();
        Callback::from(move |_| {
            player_state.as_ref().unwrap().play();
        })
    };

    let pause = {
        let player_state = player_state.clone();
        Callback::from(move |_| {
            player_state.as_ref().unwrap().pause();
        })
    };

    html! {
        <>
        <div class="plyr__video-embed" ref={player_node_ref} style="width:400px">
            <iframe
            src="https://www.youtube.com/embed/bTqVqk7FSmY?origin=https://plyr.io&amp;iv_load_policy=3&amp;modestbranding=1&amp;playsinline=1&amp;showinfo=0&amp;rel=0&amp;enablejsapi=1"
            allowfullscreen=true
            allow="autoplay"
            ></iframe>
        </div>
        <button onclick={fullscreen}>{"fullscreen"}</button>
        <button onclick={play}>{"play"}</button>
        <button onclick={pause}>{"pause"}</button>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <PlyrFrame/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
