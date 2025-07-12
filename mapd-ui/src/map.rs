use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use leptos_meta::Style;
use leptos_router::hooks::query_signal;

use super::gjson::GeoJson;

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
        <GeoJson />
      </MapContainer>

    }
}
