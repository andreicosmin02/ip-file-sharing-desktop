mod certificate;

use crate::certificate::generator::generate_cert_and_key;

fn main() {
    // Generate TLS certificates
    if let Err(e) = generate_cert_and_key() {
        eprintln!("Failed to generate cert and key: {}", e);
    } else {
        println!("Successfully generated cert.pem and key.pem");
    }

    // TODO: Initialize server and other components
}