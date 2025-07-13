use leptos::prelude::*;
use leptos_leaflet::{leaflet::LatLng, prelude::*};
use leptos_meta::Style;
use leptos_router::hooks::query_signal;

use super::gjson::GeoJson;

#[component]
pub fn Map() -> impl IntoView {
    let (map, set_map) = create_map_signal();

    let (lat, set_lat) = query_signal("lat");
    let (lng, set_lng) = query_signal("lng");
    let (z, set_z) = query_signal("z");

    let moving = StoredValue::new_local(false);
    let events = MapEvents::new().move_end(move |_| {
        if !*moving.read_value() {
            let map = map.read();
            let map = map.as_ref().unwrap();

            set_lat.set(Some(map.get_center().lat()));
            set_lng.set(Some(map.get_center().lng()));
            set_z.set(Some(map.get_zoom()));
        }

        *moving.write_value() = false;
    });

    Effect::new(move || {
        if let Some(map) = map.read().as_ref() {
            let center = LatLng::new(lat.read().unwrap_or(48.84), lng.read().unwrap_or(2.34));
            let zoom = z.read().unwrap_or(14.0);

            *moving.write_value() = true;
            map.set_view(&center, zoom);
        }
    });

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
        events
        class="map"
        min_zoom=13.0
        zoom_control=false
      >
        <TileLayer url="https://cartodb-basemaps-{s}.global.ssl.fastly.net/dark_all/{z}/{x}/{y}.png" min_zoom=0.0 max_zoom=20.0 />
        <GeoJson />
      </MapContainer>

    }
}
