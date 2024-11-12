use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/greet/:name", get(greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            match axum::serve(listener, app).await {
                Ok(_) => println!("Server shut down gracefully."),
                Err(e) => eprintln!("Server error: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to bind to address {}: {}", addr, e),
    }
}
