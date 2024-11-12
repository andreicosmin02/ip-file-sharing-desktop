use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct GreetingResponse {
    message: String,
}

// Command to fetch greeting from the Axum backend
#[tauri::command]
async fn get_greeting(name: String) -> Result<String, String> {
    let url = format!("http://127.0.0.1:3000/greet/{}", name);

    let response = Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    
    let greeting: GreetingResponse = GreetingResponse { message: response.text().await.map_err(|err| err.to_string())? };
    Ok(greeting.message)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_greeting])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}