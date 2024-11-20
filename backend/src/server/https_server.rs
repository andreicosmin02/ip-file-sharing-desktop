use axum::{routing::post, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::{SocketAddr, TcpListener};
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

    // Iterate over a range of ports to find an available one using `SocketAddr`
    let mut addr = None;
    for port in 3000..4000 {
        let socket_addr = SocketAddr::from(([0, 0, 0, 0], port));
        match TcpListener::bind(socket_addr) {
            Ok(_) => {
                addr = Some(socket_addr);
                println!("Successfully bound to port: {}", port);
                break;
            }
            Err(_) => {
                // Port is not available, try the next one
                continue;
            }
        }
    }

    // If no port was found, return an error
    let addr = addr.ok_or("Could not find an available port in the specified range")?;

    // Print the assigned address to the console
    println!("HTTPS server listening on https://{}", addr);

    // Start the HTTPS server using the `RustlsConfig` and the assigned address
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .map_err(|e| {
            eprintln!("Server error: {}", e);
            e
        })?;

    Ok(())
}