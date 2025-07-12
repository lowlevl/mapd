use leptos::prelude::*;
use leptos_leaflet::{leaflet, prelude::*};
use wasm_bindgen::{JsValue, prelude::*};

#[wasm_bindgen]
extern "C" {
    /// [`GeoJSON`](https://leafletjs.com/reference-1.9.4.html#geojson)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = leaflet::GeoJson, extends = leaflet::Layer)]
    pub type GeoJson;

    /// [`L.geoJSON`](https://leafletjs.com/reference-1.9.4.html#geojson-l-geojson)
    #[wasm_bindgen(constructor, js_namespace = L, js_class = geoJSON)]
    pub fn new(data: &JsValue) -> GeoJson;
}

#[component]
pub fn GeoJson() -> impl IntoView {
    let ctx = use_context::<LeafletMapContext>()
        .expect("a GeonJson shall only be used in the context of a MapContainer");

    Effect::new(move |_| {
        let layer = GeoJson::new(&JsValue::undefined());

        ctx.add_layer(&layer);
    });
}
