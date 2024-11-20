use rcgen::{generate_simple_self_signed, CertifiedKey};
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn generate_cert_and_key() -> Result<(), Box<dyn Error>> {
    let subject_alt_names = vec!["localhost".to_string(), "device.local".to_string()];
    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names)
        .map_err(|e| {
            eprintln!("Failed to generate self-signed certificate: {}", e);
            e
        })?;

    // Write the certificate to `cert.pem`
    let mut cert_file = File::create("cert.pem")?;
    cert_file.write_all(cert.pem().as_bytes())?;

    // Write the private key to `key.pem`
    let mut key_file = File::create("key.pem")?;
    key_file.write_all(key_pair.serialize_pem().as_bytes())?;

    Ok(())
}