mod endpoints;
mod model;

use axum::{
    routing::{get, post},
    Router,
};
use endpoints::{css_rules, evaluation, home, html_rules, image, js_rules, styles};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use crate::endpoints::{image, rules};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build app with routes
    let app = Router::new()
        .route("/", get(home))
        .route("/evaluateCode", post(evaluation))
        .route("/getCssRules", get(css_rules))
        .route("/getJsRules", get(js_rules))
        .route("/getHtmlRules", get(html_rules))
        .route("/styles.css", get(styles))
        .route("/image.png", get(image));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
