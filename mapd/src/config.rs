use std::{collections::BTreeSet, path::PathBuf, time::Duration};

pub struct Ui {
    title: String,
}

pub enum Basemap {
    Tile { url: Url },
    TileWMS { url: Url },
    TileArcGis { url: Url, options: () },
}

pub enum Feature {
    Overpass { query: String, expiration: Duration },
    Local { path: PathBuf },
}

pub struct Layer {
    name: String,
    description: String,
    basemaps: BTreeSet<Basemap>,
    features: BTreeSet<Feature>,
}

pub struct Config {
    ui: Ui,
    layers: BTreeSet<Layer>,
}
