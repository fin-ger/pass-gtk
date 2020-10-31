#![recursion_limit="1024"]

mod view;
mod extensions;
mod app_model;
mod app_component;
mod app_view;
mod password_list_model;
mod password_list_component;
mod password_list_view;

mod prelude {
    pub use crate::view::View;
    pub use crate::extensions::*;
}

fn main() {
    std::process::exit(vgtk::run::<app_model::App>());
}
