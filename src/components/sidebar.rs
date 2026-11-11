use dioxus::prelude::*;

use crate::Route;

const PAGES: &[(&str, Route)] = &[("Home", Route::Home {})];

const ICON: Asset = asset!("/assets/images/VideoBot.png");

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        style { "{STYLES}" },
        div{
            class: "sidebar",
            div{
                class: "header",
                div {
                    class: "shape sided-cookie6 medium rotate",
                    img {
                        src: ICON,
                        alt: "Local Idiot",
                        class: "responsive",
                    }
                }
            }
            for (name, route) in PAGES {
                a {
                    href: route.to_string(),
                    "{name}"
                }
            }
            Outlet::<Route> {}
        }
    }
}

const STYLES: &str = r#"
    .header{
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .sidebar{
        display: flex;
        flex-direction: column;
        gap: 2rem;
        border-right: solid #37353D 2px;
        padding: 1rem;
        min-width: 12rem;

        a{
            text-decoration: none;
        }
    }
"#;
