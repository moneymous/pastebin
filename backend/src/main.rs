mod routes;
mod get_paste;
mod db;
mod make_paste;

#[tokio::main]
async fn main() {
    let app = routes::get_all_routes();

    let addr = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Pastebin's backend is listening at {}", addr);
    axum::serve(listener, app).await.unwrap();
}