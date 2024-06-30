use leptos::*;

use crate::pages::app::App;

mod components {
    pub mod todo_item;
}
mod pages {
    pub mod app;
}


fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}