use std::net::SocketAddr;

mod routes;
mod handlers;
mod models;
mod utils;
mod response;
mod repo;
mod auth;


#[tokio::main]
async fn main() {
    
    // load environment variables from the '.env' file.
    dotenvy::dotenv().expect("Failed to load environment variable");

    let routes = routes::routes_init().await;

    let addr = SocketAddr::from(([0,0,0,0], 4546));

    println!("url: http://{addr}");

    let tcp_listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Port is already been used");


    axum::serve(tcp_listener, routes)
        .await
        .expect("Error serving application")
}
