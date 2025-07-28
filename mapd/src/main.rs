use axum::{Json, Router};
use eyre::eyre;
use leptos_axum::LeptosRoutes;
use mapd_ui::state::State;
use tracing::Level;
use tracing_subscriber::EnvFilter;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env()?,
        )
        .try_init()
        .map_err(|err| eyre!(err))?;

    let config = Config::load("mapd.yaml").await?;
    let state = config.into_state().await?;

    tracing::info!("Successfully loaded state: {state:?}");

    let lopts = leptos::config::get_configuration(Some("Cargo.toml"))?.leptos_options;
    let app = Router::new()
        .route(
            State::URI,
            axum::routing::get(move || async move { Json(state) }),
        )
        .leptos_routes(
            &lopts.clone(),
            leptos_axum::generate_route_list(mapd_ui::Ui),
            {
                let lopts = lopts.clone();
                move || mapd_ui::shell(lopts.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(mapd_ui::shell))
        .with_state(lopts.clone());

    tracing::info!(
        "Starting {} on `{:#?}`...",
        env!("CARGO_PKG_NAME"),
        lopts.site_addr
    );

    axum::serve(tokio::net::TcpListener::bind(lopts.site_addr).await?, app)
        .await
        .map_err(Into::into)
}
