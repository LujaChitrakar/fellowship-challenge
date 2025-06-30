pub mod handlers;
pub mod routes;
use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use solana_client::client_error::reqwest::blocking::get;
use tokio::net::TcpListener;

use crate::routes::{
    create_token::create_token, generate_keypair::generate_keypair, mint_token::mint_token, sign_message::sign_message,
};
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate-keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running at {}", addr);

    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, app).await.unwrap();
}
