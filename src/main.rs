#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use hourrs_classic::app;
fn main() {
    hot_reload_init!();
    // launch the dioxus app in a webview
    dioxus_desktop::launch(app);
}
