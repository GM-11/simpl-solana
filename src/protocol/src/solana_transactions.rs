use std::str::FromStr;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_solana::{
    rpc_client::{ConsensusStrategy, RpcApi, RpcClient, RpcClientConfig},
    system_instruction::transfer,
    types::{
        BlockHash, Cluster, CommitmentLevel, Message, Pubkey, RpcContextConfig,
        RpcSendTransactionConfig, Transaction,
    },
};
use serde_json::Value;

pub const SOLANA_TREASURY_ADDRESS: &str = "29HY6PqigFtdgyBTiiQQ5XCd3MxrXWSFtMWysqgHK5eJ";

pub async fn get_sol_price(timestamp: u64) -> Result<f64, String> {
    // 1682978460
    // 1737212575675693
    ic_cdk::api::print(format!("fetching price at {}", timestamp));
    let headers = vec![HttpHeader {
        name: "User-Agent".to_string(),
        value: "exchange_rate_canister".to_string(),
    }];
    let url = format!(
        "https://api.exchange.coinbase.com/products/SOL-USD/candles?start={}&end={}&granularity={}",
        (timestamp.checked_sub(60u64).unwrap()).to_string(),
        (timestamp.checked_add(60u64).unwrap()).to_string(),
        60.to_string()
    );

    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: None,
        method: HttpMethod::GET,
        headers,
        body: None,
        transform: None,
    };

    let response = http_request(request, 1_603_131_200).await;

    match response {
        Ok(response) => {
            let response_body = String::from_utf8(response.0.body).unwrap();
            ic_cdk::api::print(response_body.clone());
            let response_array: Value = serde_json::from_str(response_body.as_str()).unwrap();
            let array = &response_array[0];

            Ok(array[4].as_f64().unwrap())
        }
        Err(err) => Err(format!("Error creating order: {}", err.1)),
    }
}

pub async fn transfer_sol(from: String, to: String, amount: u64, keypair: [u8; 64]) -> String {
    let from = Pubkey::from_str(from.as_str()).expect("Invalid pubkey");

    let cluster = Cluster::Localnet;
    let rpc = RpcApi::new(cluster.url());
    ic_cdk::api::print(cluster.url());
    let config = RpcClientConfig {
        response_consensus: Some(ConsensusStrategy::Equality),
        response_size_estimate: None,
        request_cost_calculator: None,
        host_validator: None,
        transform_context: None,
        use_compression: false,
        is_demo_active: true,
    };

    let context_config = RpcContextConfig {
        commitment: Some(CommitmentLevel::Finalized),
        min_context_slot: None,
    };

    let client = RpcClient::new(vec![rpc], Some(config));

    let rpc_transaction_config = RpcSendTransactionConfig {
        encoding: Some(ic_solana::types::UiTransactionEncoding::Base64),
        skip_preflight: false,
        preflight_commitment: Some(CommitmentLevel::Finalized),
        max_retries: None,
        min_context_slot: None,
    };

    let to = Pubkey::from_str(to.as_str()).expect("Invalid pubkey");

    let transfer_ix = transfer(&from, &to, amount);

    let mut tx = Transaction::new_unsigned(Message::new(&[transfer_ix], Some(&from)));

    let latest_blockhash = client
        .get_latest_blockhash(Some(context_config))
        .await
        .unwrap()
        .parse_value()
        .blockhash;

    tx.set_latest_blockhash(&BlockHash::from_str(latest_blockhash.as_str()).unwrap());

    let secret_key: [u8; 32] = keypair[0..32].try_into().unwrap();
    tx.sign(0, &secret_key);

    let signature = client.send_transaction(tx, rpc_transaction_config).await;

    ic_cdk::api::print(format!("{:?}", signature));

    match signature {
        Ok(sig) => sig.to_string(),
        Err(err) => err.to_string(),
    }
}
