use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsignedTransaction {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub signatures: Vec<String>,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub header: MessageHeader,
    pub account_keys: Vec<String>,
    pub recent_blockhash: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaApiRequest {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: Vec<SolanaApiParams>,
}

impl SolanaApiRequest {
    pub fn to_json_string(&self) -> String {
        // Build the params array string
        let params_string = self
            .params
            .iter()
            .map(|param| match param {
                SolanaApiParams::Commitment(commitment) => {
                    format!("{{\"commitment\":\"{}\"}}", commitment.commitment)
                }
                SolanaApiParams::Address(address) => address.to_string(),
                SolanaApiParams::Transaction(transaction) => transaction.to_string(),
            })
            .collect::<Vec<String>>()
            .join(",");

        // Construct the final JSON string
        format!(
            "{{\"id\":{},\"jsonrpc\":\"{}\",\"method\":\"{}\",\"params\":[{}]}}",
            self.id,
            self.jsonrpc,
            self.method.trim(),
            params_string
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaApiResponse {
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SolanaApiParams {
    Address(String),
    Transaction(String),
    Commitment(SolanaApiCommitment),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaApiCommitment {
    pub commitment: String,
}
