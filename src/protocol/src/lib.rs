use std::str::FromStr;
pub mod solana_transactions;

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
    let result = razorpay::payout(args).await;
    match result {
        Ok(res) => res,
        Err(e) => e,
    }
}

#[ic_cdk::update]
pub async fn create_order(amount: u64) -> String {
    let result = razorpay::create_order(amount).await;
    match result {
        Ok(res) => res,
        Err(e) => e,
    }
}

ic_cdk::export_candid!();
