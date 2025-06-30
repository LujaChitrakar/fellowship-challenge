use axum::{Json, Router, routing::post};
use std::str::FromStr;

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use spl_token::{ID as TOKEN_PROGRAM_ID, instruction::initialize_mint};
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct CreateMintRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
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
pub struct CreateMintResponse {
    success: bool,
    data: InstructionData,
}

pub async fn create_token(Json(body): Json<CreateMintRequest>) -> Json<CreateMintResponse> {
    // Parse pubkeys
    let mint_pubkey = Pubkey::from_str(&body.mint).unwrap();

    let mint_authority_pubkey = Pubkey::from_str(&body.mintAuthority).unwrap();

    let ix = initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &mint_authority_pubkey,
        None, // no freeze authority
        body.decimals,
    )
    .unwrap();

    let response = CreateMintResponse {
        success: true,
        data: InstructionData {
            program_id: ix.program_id.to_string(),
            accounts: ix
                .accounts
                .into_iter()
                .map(|meta| AccountMetaResponse {
                    pubkey: meta.pubkey.to_string(),
                    is_signer: meta.is_signer,
                    is_writable: meta.is_writable,
                })
                .collect(),
            instruction_data: general_purpose::STANDARD.encode(ix.data),
        },
    };

    Json(response)
}
