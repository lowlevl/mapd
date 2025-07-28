use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ui {
    pub title: String,
    pub center: (f64, f64),
    pub zoom: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub ui: Ui,
    pub layers: Vec<()>,
}

impl State {
    pub const URI: &str = "/v1/state";

    pub async fn fetch() -> Result<Self, reqwest::Error> {
        reqwest::get(Self::URI).await?.json().await
    }
}
