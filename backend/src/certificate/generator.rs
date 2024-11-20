use rcgen::generate_simple_self_signed;
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn generate_cert_and_key() -> Result<(), Box<dyn Error>> {
    let subject_alt_names = vec!["localhost".to_string(), "device.local".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names)?;

    // Write the certificate to `cert.pem`
    let mut cert_file = File::create("cert.pem")?;
    cert_file.write_all(cert.serialize_pem()?.as_bytes())?;

    // Write the private key to `key.pem`
    let mut key_file = File::create("key.pem")?;
    key_file.write_all(cert.serialize_private_key_pem().as_bytes())?;

    Ok(())
}
