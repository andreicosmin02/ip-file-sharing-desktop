use mdns_sd::{ServiceDaemon, ServiceInfo};
use hostname::get;
use std::error::Error;
use local_ipaddress;

pub fn announce_service(port: u16) -> Result<(), Box<dyn Error>> {
    println!("Starting to announce the service over mDNS...");

    // Step 1: Create a new mDNS service daemon
    let mdns = ServiceDaemon::new().map_err(|e| {
        eprintln!("Failed to create mDNS daemon: {}", e);
        e
    })?;

    // Step 2: Get the device name
    let instance_name = get_device_name()?;

    // Step 3: Define the service type and properties
    let service_type = "_fileshare._tcp.local."; // Service type in mDNS format

    // Get the local IP address (for simplicity, use loopback; replace with LAN IP for real use)
    let local_ip = match local_ipaddress::get() {
        Some(ip) => ip.to_string(),
        None => {
            eprintln!("Could not determine local IP address");
            return Err("Could not determine local IP address".into());
        }
    };
    let host_name = format!("{}.local.", instance_name);

    // Define optional properties
    let properties = [
        ("device".to_string(), instance_name.clone()),
        ("description".to_string(), "A sample mDNS service".to_string()),
    ];

    // Step 4: Create the ServiceInfo with the details from above
    println!("Creating service info with name '{}' on port {}", instance_name, port);
    let my_service = ServiceInfo::new(
        service_type,           // Service type (e.g., "{name}_fileshare._tcp.local.")
        &instance_name,         // Instance name (unique name for this device)
        &host_name,             // Hostname, must end with ".local."
        local_ip,               // The IP address (e.g., "192.168.x.x")
        port,                   // The port to announce (e.g., 3000)
        &properties[..],        // Properties as slice
    )
        .map_err(|e| {
            eprintln!("Failed to create service info: {}", e);
            e
        })?;

    // Step 5: Register the service with the daemon
    println!("Registering the service with mDNS daemon...");
    mdns.register(my_service).map_err(|e| {
        eprintln!("Failed to register mDNS service: {}", e);
        e
    })?;

    println!("mDNS Service Announced: {} on Port: {}", instance_name, port);

    // Graceful shutdown (optional)
    // Since our server will keep running, we don't want to immediately shut down.
    // However, if you want to end the announcement, use the shutdown method:
    // mdns.shutdown().unwrap();

    Ok(())
}

fn get_device_name() -> Result<String, Box<dyn Error>> {
    match get() {
        Ok(name) => {
            let hostname = name.to_string_lossy().into_owned();
            Ok(format!("{}_fileshare", hostname))
        },
        Err(_) => {
            // Fallback to a default name in case hostname retrieval fails
            Ok("UnknownDevice_fileshare".to_string())
        }
    }
}
