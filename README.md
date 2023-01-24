# Plyr-rs

Rust binding for [Plyr](https://github.com/sampotts/plyr)．Please check the official plyr page for more details.

## Installation
Specify it as a dependency in `Cargo.toml`
```toml
[dependencies]
plyr = { git = "https://github.com/deepgreenAN/plyr-rs"}
```

## Usage
### Html or Rsx and CSS
See the official page or [simple example](https://github.com/deepgreenAN/plyr-rs/tree/master/examples/simple-plyr-use) for how to write in html or rsx and css.

### Rust
Give the css selector as an argument as follows,
```rust
use plyr::Plyr;
let player = Plyr::new("#player");
```
or `web_sys::HtmlElement`.
```rust
use gloo_utils::document;
use plyr::Plyr;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

let player_element = document().query_selector("#player").unwrap().unwrap();
let player = Plyr::new_with_html_element(&player_element.unchecked_into::<HtmlElement>());
```

If you want to pass options to the constructor, modify the features.
```toml
[dependencies]
plyr = { git = "https://github.com/deepgreenAN/plyr-rs", features = ["options"]}
```

```rust
use plyr::options::PlyrOptions;
use plyr::Plyr;

fn main() {
    let _player = Plyr::new_with_options("#player", &PlyrOptions::builder().duration(50.0).build());
}
```

## Cdn
You can uee the cdn for reducing the bundle size of js snippets.
```html
<script src="https://cdn.plyr.io/3.7.3/plyr.polyfilled.js"></script>
```

```toml
[dependencies]
plyr = { git = "https://github.com/deepgreenAN/plyr-rs", features = ["cdn"]}
```

## Examples
- [yew example](https://github.com/deepgreenAN/plyr-rs/tree/master/examples/plyr-yew-example)
- [dioxus example](https://github.com/deepgreenAN/plyr-rs/tree/master/examples/plyr-dioxus-example)
- [sycamore example](https://github.com/deepgreenAN/plyr-rs/tree/master/examples/plyr-sycamore-example)
- [leptos example](https://github.com/deepgreenAN/plyr-rs/tree/master/examples/plyr-leptos-example)
- [event listener example](https://github.com/deepgreenAN/plyr-rs/tree/master/examples/event-listener-example)
Check [examples](https://github.com/deepgreenAN/plyr-rs/tree/master/examples) directory for other examples.