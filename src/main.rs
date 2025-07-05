#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> eyre::Result<()> {
    use axum::Router;
    use eyre::eyre;
    use leptos_axum::LeptosRoutes;
    use tracing::Level;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env()?,
        )
        .try_init()
        .map_err(|err| eyre!(err))?;

    let lopts = leptos::config::get_configuration(None)?.leptos_options;
    let app = Router::new()
        .leptos_routes(
            &lopts.clone(),
            leptos_axum::generate_route_list(mapd::ui::Ui),
            {
                let lopts = lopts.clone();
                move || mapd::ui::shell(lopts.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(mapd::ui::shell))
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

#[cfg(not(feature = "ssr"))]
fn main() {}
