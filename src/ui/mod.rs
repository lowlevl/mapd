use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use leptos_meta::{Html, Meta, Script, Style, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::query_signal,
    path,
};

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::MetaTags;

    view! {
      <!DOCTYPE html>
      <html>
        <head>
          <HydrationScripts options=options.clone() />
          <AutoReload options />

          <MetaTags />
        </head>
        <body>
          <Ui />
        </body>
      </html>
    }
}

#[component]
pub fn Map() -> impl IntoView {
    let (lat, set_lat) = query_signal("lat");
    let (lng, set_lng) = query_signal("lng");
    let (z, set_z) = query_signal("z");

    let (map, set_map) = create_map_signal();

    let events = MapEvents::new()
        .move_end(move |_| {
            if let Some(map) = map.read().as_ref() {
                set_lat.set(Some(map.get_center().lat()));
                set_lng.set(Some(map.get_center().lng()));
            }
        })
        .zoom_end(move |_| {
            if let Some(map) = map.read().as_ref() {
                set_z.set(Some(map.get_zoom()));
            }
        });

    let center = || Position::new(lat.get().unwrap_or(48.84), lng.get().unwrap_or(2.34));
    let zoom = || z.get().unwrap_or(14.0);

    view! {
      <Script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" />
      <Stylesheet href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />

      <Style>
      r#"
        .map {
          width: 100vw;
          height: 100vh;
          background: transparent;
        }

        .leaflet-control-attribution {
          display: none;
        }
      "#
      </Style>

      <MapContainer
        map=set_map
        zoom=zoom()
        center=center()
        events
        min_zoom=13.0
        zoom_control=false
        class="map"
      >
        <TileLayer url="https://cartodb-basemaps-{s}.global.ssl.fastly.net/dark_all/{z}/{x}/{y}.png" min_zoom=0.0 max_zoom=20.0 />
      </MapContainer>

    }
}

#[component]
fn NotFound() -> impl IntoView {
    const PIN: &str = r#"
#         #####          
  #   ###       ###      
    ##             ##    
   ####   #####      #   
  #######       #     #  
  ##### ###      #    #  
  ##### ####     #    #  
  ###### #####  #     #  
   ####### #####     #   
    ##############  #    
     ###############     
      ############# #    
        #########     #  
         #######        #
           ###           
            #            
"#;

    view! {
      <div class="not-found">
        <pre class="pin">{PIN}</pre>
        <span>404: this page does not exist.</span>
      </div>
    }
}

#[component]
pub fn Ui() -> impl IntoView {
    leptos_meta::provide_meta_context();

    view! {
        <Html {..} lang="en" />
        <Title text="mapd" />

        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />

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
