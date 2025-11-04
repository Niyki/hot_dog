mod backend;
mod frontend;

use dioxus::prelude::*;
use frontend::*;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
