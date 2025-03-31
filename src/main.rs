use app::create_app;
use tokio::net::TcpListener;

mod app;

#[tokio::main]
async fn main() {
    let port = 3000;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind port");

    let app = create_app().await;

    println!("Server listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
