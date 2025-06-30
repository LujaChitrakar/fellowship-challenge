use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use spl_token::{instruction::transfer, ID as TOKEN_PROGRAM_ID};
use base64::{engine::general_purpose, Engine as _};
use std::{net::SocketAddr, str::FromStr};

#[derive(Deserialize)]
pub struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct AccountInfo {
    pubkey: String,
    isSigner: bool,
}

#[derive(Serialize)]
pub struct InstructionResponseData {
    program_id: String,
    accounts: Vec<AccountInfo>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    success: bool,
    data: InstructionResponseData,
}

pub async fn send_token(Json(body): Json<SendTokenRequest>) -> Json<SendTokenResponse> {
    let destination = match Pubkey::from_str(&body.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(SendTokenResponse {
                success: false,
                data: InstructionResponseData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "Invalid destination address".to_string(),
                },
            });
        }
    };

    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(SendTokenResponse {
                success: false,
                data: InstructionResponseData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "Invalid mint address".to_string(),
                },
            });
        }
    };

    let owner = match Pubkey::from_str(&body.owner) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(SendTokenResponse {
                success: false,
                data: InstructionResponseData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "Invalid owner address".to_string(),
                },
            });
        }
    };

    // Build the transfer instruction
    let ix = transfer(
        &TOKEN_PROGRAM_ID,
        &body.mint.parse().unwrap(),
        &body.destination.parse().unwrap(),
        &body.owner.parse().unwrap(),
        &[],
        body.amount,
    )
    .unwrap();

    let accounts = ix
        .accounts
        .iter()
        .map(|acc| AccountInfo {
            pubkey: acc.pubkey.to_string(),
            isSigner: acc.is_signer,
        })
        .collect();

    let response = SendTokenResponse {
        success: true,
        data: InstructionResponseData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(ix.data),
        },
    };

    Json(response)
}