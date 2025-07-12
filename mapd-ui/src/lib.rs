use leptos::prelude::*;
use leptos_meta::{Html, Meta, Script, Style, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod not_found;
use not_found::NotFound;

mod map;
use map::Map;

mod gjson;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::MetaTags;

    view! {
      <!DOCTYPE html>
      <html>
        <head>
          <MetaTags />

          <HydrationScripts options=options.clone() />
          <AutoReload options />
        </head>
        <body>
          <Ui />
        </body>
      </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::{logging, mount};

    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();

    logging::log!("Rehydrating `{}` after SSR", env!("CARGO_PKG_NAME"));

    mount::hydrate_body(Ui);
}

#[component]
pub fn Ui() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Html {..} lang="en" />
        <Title text="mapd :: an untitled map" />

        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />

        <Script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" />
        <Stylesheet href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />

        <Style>
        r#"
          html, body {
            margin: 0;

            color: #a6a6a6;
            background: #262626;

            font-family: 'Trebuchet MS', sans-serif;
          }

          .not-found {
            position: absolute;

            top: 50%;
            transform: translateY(-50%);

            width: 100vw;
            text-align: center;
          }

          .pin {
            line-height: 1.15em;
          }
        "#
        </Style>

        <Router>
          <main>
            <Routes fallback=NotFound>
              <Route path=path!("/") view=Map />
            </Routes>
          </main>
        </Router>
    }
}
