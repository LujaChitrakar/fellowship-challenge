use axum::{routing::post, Json, Router, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};
use std::{net::SocketAddr, str::FromStr};

use base64::{engine::general_purpose, Engine as _};
use bs58;

#[derive(Deserialize)]
pub struct SignRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
pub struct SignResponse {
    success: bool,
    data: Option<SignData>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct SignData {
    signature: String,
    public_key: String,
    message: String,
}

pub async fn sign_message(Json(body): Json<SignRequest>) -> impl IntoResponse {
    // Validate fields
    if body.message.is_empty() || body.secret.is_empty() {
        return Json(SignResponse {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        });
    }

    // Decode base58 secret key
    let secret_bytes = match bs58::decode(&body.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(SignResponse {
                success: false,
                data: None,
                error: Some("Invalid base58 secret key".to_string()),
            });
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(SignResponse {
                success: false,
                data: None,
                error: Some("Invalid secret key format".to_string()),
            });
        }
    };

    let signature = keypair.sign_message(body.message.as_bytes());

    Json(SignResponse {
        success: true,
        data: Some(SignData {
            signature: general_purpose::STANDARD.encode(signature.as_ref()),
            public_key: keypair.pubkey().to_string(),
            message: body.message,
        }),
        error: None,
    })
}