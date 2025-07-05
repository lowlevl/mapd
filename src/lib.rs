pub mod data;
pub mod ui;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::{logging, mount};

    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();

    logging::log!("Rehydrating `{}` after SSR", env!("CARGO_PKG_NAME"));

    mount::hydrate_body(ui::Ui);
}
