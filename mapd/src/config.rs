use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use mapd_ui::state::{self, State};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Service {
    ui: state::Ui,
    cache: PathBuf,
    overpass: Url,
}

#[derive(Debug, Deserialize)]
pub enum Basemap {
    Tile { url: Url },
    TileWMS { url: Url },
    TileArcGis { url: Url, options: () },
}

#[derive(Debug, Deserialize)]
pub enum Feature {
    Overpass { query: String, expiration: Duration },
    Local { path: PathBuf },
}

#[derive(Debug, Deserialize)]
pub struct Layer {
    name: String,
    description: String,
    basemaps: Vec<Basemap>,
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    service: Service,
    layers: Vec<Layer>,
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let file = tokio::fs::read_to_string(path).await?;

        Ok(serde_yaml::from_str(&file)?)
    }

    pub async fn into_state(self) -> eyre::Result<&'static State> {
        tokio::fs::create_dir_all(&self.service.cache).await?;
        tracing::debug!("Created directory at {}", self.service.cache.display());

        for layer in self.layers {
            for feature in layer.features {
                match feature {
                    Feature::Overpass { query, expiration } => {}
                    Feature::Local { path } => {}
                }
            }
        }

        let state = State {
            ui: self.service.ui,
            layers: Default::default(),
        };

        Ok(Box::leak(state.into()))
    }
}
