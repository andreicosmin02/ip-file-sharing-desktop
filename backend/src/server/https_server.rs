use axum::{routing::post, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::error::Error;

use crate::server::file_handler::receive_file;

pub async fn start_https_server() -> Result<(), Box<dyn Error>> {
    // Load the certificate and key files for HTTPS
    let tls_config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .map_err(|e| {
            eprintln!("Failed to load TLS configuration: {}", e);
            e
        })?;

    // Define routes
    let app = Router::new().route("/receive", post(receive_file));

    // Define the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("HTTPS server listening on https://{}", addr);

    // Start the server with the TLS configuration
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            eprintln!("Server error: {}", e);
            e
        })?;

    Ok(())
}
