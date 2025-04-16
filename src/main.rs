mod backend;

use axum::{routing::get, Router, Server};
use backend::arch_tree;
use backend::component_tree;
use backend::filter_and_classify;
use backend::mf_tree;
use backend::project_arch_tree;
use http::Method;
use std::error::Error;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();


    // Create a CORS middleware.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/filter_and_classify/:tag_name", get(filter_and_classify))
        .route("/arch_tree/:cross_module", get(arch_tree))
        .route("/component_tree/:cross_module", get(component_tree))
        .route("/mf_tree/:cross_module", get(mf_tree))
        .route("/project_tree/:project", get(project_arch_tree))
        .layer(cors);
    axum_server::bind_rustls(addr, config).serve(app.into_make_service()).await?;

    Ok(())
}
