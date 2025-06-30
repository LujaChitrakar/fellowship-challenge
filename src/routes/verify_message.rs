use axum::{
    routing::post,
    Json, Router,
    extract::State,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
};
use base64::{decode};

#[derive(Deserialize)]
pub struct VerifyRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    success: bool,
    data: Option<VerifyData>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct VerifyData {
    valid: bool,
    message: String,
    pubkey: String,
}

pub async fn verify_message(Json(req): Json<VerifyRequest>) -> (StatusCode, Json<VerifyResponse>) {
    // Decode pubkey and signature
    let pubkey = match Pubkey::from_str(&req.pubkey) {
        Ok(pk) => pk,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(VerifyResponse {
                success: false,
                data: None,
                error: Some("Invalid pubkey".into()),
            }));
        }
    };

    let signature_bytes = match decode(&req.signature) {
        Ok(sig) => sig,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(VerifyResponse {
                success: false,
                data: None,
                error: Some("Invalid base64 signature".into()),
            }));
        }
    };

    if signature_bytes.len() != 64 {
        return (StatusCode::BAD_REQUEST, Json(VerifyResponse {
            success: false,
            data: None,
            error: Some("Signature must be 64 bytes".into()),
        }));
    }

    let signature = match Signature::new(&signature_bytes).verify(pubkey.as_ref(), req.message.as_bytes()) {
        true => true,
        false => false,
    };

    let response = VerifyResponse {
        success: true,
        data: Some(VerifyData {
            valid: signature,
            message: req.message.clone(),
            pubkey: req.pubkey.clone(),
        }),
        error: None,
    };

    (StatusCode::OK, Json(response))
}
