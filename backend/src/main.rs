mod certificate;
mod server;

use crate::certificate::generator::generate_cert_and_key;
use crate::server::https_server::start_https_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Generate TLS certificates
    if let Err(e) = generate_cert_and_key() {
        eprintln!("Failed to generate cert and key: {}", e);
        return Err(e);
    } else {
        println!("Successfully generated cert.pem and key.pem");
    }

    // Step 2: Start HTTPS server
    if let Err(e) = start_https_server().await {
        eprintln!("Failed to start HTTPS server: {}", e);
        return Err(e);
    }

    Ok(())
}