use crate::handlers::common::*;
use axum::{Json, response::IntoResponse};
use base58::{FromBase58, ToBase58};
use serde::Serialize;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
#[derive(Serialize)]
pub struct KeypairResponse {
    pubkey: String,
    secret: String,
}

pub async fn generate_keypair() -> impl IntoResponse {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = keypair.to_bytes().to_base58();

    let response = SuccessResponse {
        success: true,
        data: KeypairResponse { pubkey, secret },
    };

    Json(response)
}
