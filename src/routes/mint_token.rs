use axum::{Json, Router, routing::post};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use spl_token::{ID as TOKEN_PROGRAM_ID, instruction::mint_to};
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct MintToRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
pub struct InstructionData {
    program_id: String,
    accounts: Vec<AccountMetaResponse>,
    instruction_data: String,
}

#[derive(Serialize)]
pub struct MintToResponse {
    success: bool,
    data: InstructionData,
}

pub async fn mint_token(Json(body): Json<MintToRequest>) -> Json<MintToResponse> {
    let mint = Pubkey::from_str(&body.mint).unwrap();
    let destination = Pubkey::from_str(&body.destination).unwrap();
    let authority = Pubkey::from_str(&body.authority).unwrap();

    let ix = mint_to(
        &TOKEN_PROGRAM_ID,
        &mint,
        &destination,
        &authority,
        &[], // no multisig
        body.amount,
    )
    .unwrap();

    let response = MintToResponse {
        success: true,
        data: InstructionData {
            program_id: ix.program_id.to_string(),
            accounts: ix
                .accounts
                .into_iter()
                .map(|acc| AccountMetaResponse {
                    pubkey: acc.pubkey.to_string(),
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
                .collect(),
            instruction_data: general_purpose::STANDARD.encode(ix.data),
        },
    };

    Json(response)
}
