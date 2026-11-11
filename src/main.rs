use dioxus::prelude::*;

mod components;
use components::sidebar::Sidebar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Sidebar)]
    #[route("/")]
    Home {}
}

const FAVICON: Asset = asset!("/assets/images/VideoBot.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const BEER_CSS: Asset = asset!("/assets/beercss/beercss.min.css");
const BEER_JS: Asset = asset!("/assets/beercss/beer.min.js");
const MDC_JS: Asset = asset!("/assets/beercss/material-dynamic-colors.min.js");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: BEER_CSS }
        Router::<Route> {}
        document::Script { type: "module", src: BEER_JS }
        document::Script { type: "module", src: MDC_JS }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        style { "{STYLE}" },
        div {
            class: "main"
        }
    }
}

const STYLE: &str = r#"
    article{
        width: 60%;
    }
    p{
        line-height: 1.6rem;
    }
    a{
        text-decoration: underline;
        font-weight: bold;
    }
"#;