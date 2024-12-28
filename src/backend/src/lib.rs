use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use structs::{Instruction, SolanaApiParams, SolanaApiRequest};

pub mod helpers;
pub mod structs;

#[ic_cdk::update]
async fn transfer_sol(from: String, to: String, lamports: u64) -> Result<Vec<u8>, String> {
    let ix = Instruction {
        program_id_index: 2,  // System program
        accounts: vec![0, 1], // From and To account indices
        data: {
            let mut data = vec![2, 0, 0, 0]; // Transfer instruction
            data.extend_from_slice(&lamports.to_le_bytes());
            data
        },
    };

    let unsigned = helpers::create_unsigned_transaction(
        vec![
            from.clone(),
            to.clone(),
            "11111111111111111111111111111111".to_string(),
        ],
        1,
        0,
        2,
        vec![ix],
    )
    .await;

    match unsigned {
        Ok(tx) => Ok(helpers::serialize_unsigned_transaction(&tx)),
        Err(err) => Err(format!("Error creating unsigned transaction: {err}")),
    }
}

#[ic_cdk::update]
pub async fn send_signed_transaction(signed_tx: Vec<u8>) -> String {
    let url = "https://api.devnet.solana.com";
    let headers = vec![HttpHeader {
        name: "content-type".to_string(),
        value: "application/json".to_string(),
    }];

    let serialized_tx = bs58::encode(signed_tx).into_string();

    let body = SolanaApiRequest {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "sendTransaction".to_string(),
        params: vec![SolanaApiParams::Transaction(serialized_tx)],
    };

    let json_string = serde_json::to_string(&body).unwrap();
    let request_body: Option<Vec<u8>> = Some(json_string.into_bytes());

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None,
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
            let result = json["result"]["value"]["blockhash"]
                .as_str()
                .unwrap()
                .to_string();
            result
        }
        Err((r, m)) => {
            format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}")
        }
    }
}

ic_cdk::export_candid!();
