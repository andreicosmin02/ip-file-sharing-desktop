use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn receive_file(mut multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        // Safely handle the retrieval of the file name
        let file_name = field.file_name().unwrap_or("received_file").to_string();
        let content = field.bytes().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Create and write to the file asynchronously
        let mut file = File::create(file_name)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        file.write_all(&content)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        println!("File received and saved successfully");
    }

    Ok((StatusCode::OK, "File uploaded successfully"))
}
