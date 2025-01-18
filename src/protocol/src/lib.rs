use std::str::FromStr;
pub mod solana_transactions;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ic_solana::types::Pubkey;
use razorpay::PayoutArgs;
use solana_transactions::transfer_sol;

#[ic_cdk::update]
pub async fn transfer_sol_from_treasury(to: String, amount: u64) -> String {
    let keypair_string = option_env!("SOLANA_TREASURY_KEYPAIR").unwrap();

    let keypair: [u8; 64] = keypair_string
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();

    let from =
        Pubkey::from_str(solana_transactions::SOLANA_TREASURY_ADDRESS).expect("Invalid pubkey");
    transfer_sol(from.to_string(), to, amount, keypair).await
}

#[ic_cdk::update]
pub async fn transfer_sol_to_treasury(from: String, amount: u64, keypair: Vec<u8>) -> String {
    let to =
        Pubkey::from_str(solana_transactions::SOLANA_TREASURY_ADDRESS).expect("Invalid pubkey");
    let keypair: [u8; 64] = keypair.as_slice().try_into().unwrap();
    transfer_sol(from, to.to_string(), amount, keypair).await
}

#[ic_cdk::update]
pub async fn transfer_inr(args: PayoutArgs) -> String {
    let razorpay_public_key = option_env!("RAZORPAY_API_KEY").expect("RAZORPAY_API_KEY not set");
    let razorpay_secret_key =
        option_env!("RAZORPAY_SECRET_KEY").expect("RAZORPAY_SECRET_KEY not set");
    let razorpay_x_acccount =
        option_env!("RAZORPAY_X_ACCCOUNT").expect("RAZORPAY_X_ACCCOUNT not set");
    let auth_string = format!("{}:{}", razorpay_public_key, razorpay_secret_key);

    let encoded_auth = BASE64.encode(auth_string.as_bytes());
    let result = razorpay::payout(args, razorpay_x_acccount.to_string(), encoded_auth).await;
    match result {
        Ok(res) => res,
        Err(e) => e,
    }
}

#[ic_cdk::update]
pub async fn create_order(amount: u64) -> String {
    let razorpay_public_key = option_env!("RAZORPAY_API_KEY").expect("RAZORPAY_API_KEY not set");
    let razorpay_secret_key =
        option_env!("RAZORPAY_SECRET_KEY").expect("RAZORPAY_SECRET_KEY not set");
    let auth_string = format!("{}:{}", razorpay_public_key, razorpay_secret_key);

    let encoded_auth = BASE64.encode(auth_string.as_bytes());
    let result = razorpay::create_order(amount, encoded_auth).await;
    match result {
        Ok(res) => res,
        Err(e) => e,
    }
}

#[ic_cdk::update]
pub async fn get_sol_price() -> String {
    let timestamp = ic_cdk::api::time().checked_div(1000_000_000u64).unwrap();
    let price = solana_transactions::get_sol_price(timestamp).await;
    match price {
        Ok(price) => price.to_string(),
        Err(e) => e,
    }
}

ic_cdk::export_candid!();
