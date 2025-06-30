use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};
use base64::{engine::general_purpose, Engine as _};
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    success: bool,
    data: InstructionData,
}

#[derive(Serialize)]
pub struct InstructionData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

pub async fn send_sol(Json(body): Json<SendSolRequest>) -> Json<SendSolResponse> {
    // Validate inputs
    let from = match Pubkey::from_str(&body.from) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(SendSolResponse {
                success: false,
                data: InstructionData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "Invalid 'from' address".to_string(),
                },
            });
        }
    };
    let to = match Pubkey::from_str(&body.to) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(SendSolResponse {
                success: false,
                data: InstructionData {
                    program_id: "".to_string(),
                    accounts: vec![],
                    instruction_data: "Invalid 'to' address".to_string(),
                },
            });
        }
    };

    if body.lamports == 0 {
        return Json(SendSolResponse {
            success: false,
            data: InstructionData {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: "Lamports must be > 0".to_string(),
            },
        });
    }

    if from == to {
        return Json(SendSolResponse {
            success: false,
            data: InstructionData {
                program_id: "".to_string(),
                accounts: vec![],
                instruction_data: "'from' and 'to' cannot be the same".to_string(),
            },
        });
    }

    // Create transfer instruction
    let ix = system_instruction::transfer(&from, &to, body.lamports);

    let response = SendSolResponse {
        success: true,
        data: InstructionData {
            program_id: ix.program_id.to_string(),
            accounts: ix.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
            instruction_data: general_purpose::STANDARD.encode(ix.data.clone()),
        },
    };

    Json(response)
}