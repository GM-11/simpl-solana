use std::error::Error;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

pub const SOLANA_URL: &str = "https://api.devnet.solana.com";

use serde_json;

use crate::structs::{
    Instruction, Message, MessageHeader, SolanaApiCommitment, SolanaApiParams, SolanaApiRequest,
    UnsignedTransaction,
};

async fn get_recent_blockhash() -> Result<String, String> {
    let headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let body = SolanaApiRequest {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "getLatestBlockhash".to_string(),
        params: vec![SolanaApiParams::Commitment(SolanaApiCommitment {
            commitment: "processed".to_string(),
        })],
    };
    let json_string = body.to_json_string();
    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: SOLANA_URL.to_string(),
        max_response_bytes: None, //optional for request
        method: HttpMethod::POST,
        headers,
        body: request_body,
        transform: None,
    };

    let response = http_request(request, 1_803_143_600u128).await;

    match response {
        Ok(res) => {
            let body = res.0.body;
            let json_string = String::from_utf8(body).unwrap();
            let json: serde_json::Value = serde_json::from_str(&json_string).unwrap();
            ic_cdk::api::print(json["result"].to_string());
            let result = json["result"]["value"]["blockhash"].as_str();

            match result {
                Some(blockhash) => Ok(blockhash.to_string()),
                None => {
                    ic_cdk::api::print("Error parsing blockhash");
                    ic_cdk::api::print(json.to_string());
                    Err(json["result"].to_string())
                }
            }
        }
        Err(err) => Err(err.1),
    }
}

pub async fn create_unsigned_transaction(
    account_keys: Vec<String>,
    num_required_signatures: u8,
    num_readonly_signed_accounts: u8,
    num_readonly_unsigned_accounts: u8,
    instructions: Vec<Instruction>,
) -> Result<UnsignedTransaction, Box<dyn Error>> {
    let recent_blockhash = get_recent_blockhash().await?;

    let transaction = UnsignedTransaction {
        message: Message {
            header: MessageHeader {
                num_required_signatures,
                num_readonly_signed_accounts,
                num_readonly_unsigned_accounts,
            },
            account_keys,
            recent_blockhash,
            instructions,
        },
    };

    Ok(transaction)
}

pub fn serialize_unsigned_transaction(transaction: &UnsignedTransaction) -> Vec<u8> {
    let mut data = Vec::new();

    data.push(transaction.message.header.num_required_signatures);
    data.push(transaction.message.header.num_readonly_signed_accounts);
    data.push(transaction.message.header.num_readonly_unsigned_accounts);

    data.push(transaction.message.account_keys.len() as u8);
    for key in &transaction.message.account_keys {
        data.extend_from_slice(&bs58::decode(key).into_vec().unwrap());
    }

    data.extend_from_slice(
        &bs58::decode(&transaction.message.recent_blockhash)
            .into_vec()
            .unwrap(),
    );

    data.push(transaction.message.instructions.len() as u8);
    for instruction in &transaction.message.instructions {
        data.push(instruction.program_id_index);
        data.push(instruction.accounts.len() as u8);
        data.extend_from_slice(&instruction.accounts);
        data.push(instruction.data.len() as u8);
        data.extend_from_slice(&instruction.data);
    }

    data
}

pub async fn send_signed_transaction(signed_tx: &str) -> Result<String, String> {
    let headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let body = SolanaApiRequest {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "sendTransaction".to_string(),
        params: vec![SolanaApiParams::Transaction(signed_tx.to_string())],
    };
    let json_string = body.to_json_string();
    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: SOLANA_URL.to_string(),
        max_response_bytes: None, //optional for request
        method: HttpMethod::POST,
        headers,
        body: request_body,
        transform: None,
    };

    let response = http_request(request, 1_803_143_600u128).await;

    match response {
        Ok(res) => {
            let body = res.0.body;
            let json_string = String::from_utf8(body).unwrap();
            let json: serde_json::Value = serde_json::from_str(&json_string).unwrap();
            let result = json["result"].as_str();

            match result {
                Some(blockhash) => Ok(blockhash.to_string()),
                None => Err("Error sending transaction".to_string()),
            }
        }
        Err(err) => Err(err.1),
    }
}
