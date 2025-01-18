use candid::CandidType;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use serde::Deserialize;

use crate::RAZORPAY_BASE_URL;

async fn create_contact(
    name: String,
    email: String,
    contact: String,
    auth: String,
) -> Result<String, String> {
    let headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Basic {}", auth),
        },
    ];

    let body = serde_json::json!({
        "name": name,
        "email": email,
        "contact": contact,
    });

    let body_bytes = serde_json::to_vec(&body).unwrap();

    let request = CanisterHttpRequestArgument {
        url: format!("{}contacts", RAZORPAY_BASE_URL),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers,
        body: Some(body_bytes),
        transform: None,
    };

    let response = http_request(request, 1_603_150_400).await;

    match response {
        Ok(response) => {
            let response_body = String::from_utf8(response.0.body).unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
            ic_cdk::println!("response_json: {:?}", response_json);
            let id = response_json["id"].as_str().unwrap();

            Ok(id.to_string())
        }
        Err(err) => Err(format!("Error creating contact: {}", err.1)),
    }
}

async fn create_fund_account(
    name: String,
    ifsc: String,
    account_number: String,
    contact_id: String,
    auth: String,
) -> Result<String, String> {
    let headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Basic {}", auth),
        },
    ];

    let body = serde_json::json!({
        "contact_id": contact_id,
        "account_type": "bank_account",
        "bank_account" : {
          "ifsc": ifsc,
          "name": name,
          "account_number": account_number,
        },
    });

    let body_bytes = serde_json::to_vec(&body).unwrap();

    let request = CanisterHttpRequestArgument {
        url: format!("{}fund_accounts", RAZORPAY_BASE_URL),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers,
        body: Some(body_bytes),
        transform: None,
    };

    let response = http_request(request, 1_603_184_000).await;

    match response {
        Ok(response) => {
            let response_body = String::from_utf8(response.0.body).unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
            ic_cdk::println!("response_json: {:?}", response_json);
            let id = response_json["id"].as_str().unwrap();

            Ok(id.to_string())
        }
        Err(err) => Err(format!("Error creating fund account: {}", err.1)),
    }
}

async fn create_payout(
    account_number: String,
    fund_account_id: String,
    amount: u64,
    auth: String,
) -> Result<String, String> {
    let headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Basic {}", auth),
        },
    ];

    let body = serde_json::json!({
        "fund_account_id": fund_account_id,
        "account_number": account_number,
        "amount": amount,
        "currency": "INR",
        "mode": "IMPS",
        "purpose": "payout"
    });

    let body_bytes = serde_json::to_vec(&body).unwrap();

    let request = CanisterHttpRequestArgument {
        url: format!("{}payouts", RAZORPAY_BASE_URL),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers,
        body: Some(body_bytes),
        transform: None,
    };

    let response = http_request(request, 1_603_172_400).await;

    match response {
        Ok(response) => {
            let response_body = String::from_utf8(response.0.body).unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
            ic_cdk::println!("response_json: {:?}", response_json);
            let id = response_json["id"].as_str().unwrap();

            Ok(id.to_string())
        }
        Err(err) => Err(format!("Error creating payout: {}", err.1)),
    }
}

#[derive(CandidType, Deserialize)]
pub struct PayoutArgs {
    name: String,
    email: String,
    contact: String,
    ifsc: String,
    account_number: String,
    amount: u64,
}

pub async fn payout(
    args: PayoutArgs,
    razorpay_x_acccount: String,
    auth: String,
) -> Result<String, String> {
    let contact_id =
        create_contact(args.name.clone(), args.email, args.contact, auth.clone()).await;

    if contact_id.is_err() {
        return Err(contact_id.err().unwrap());
    }

    let contact_id = contact_id.unwrap();

    let create_fund_account = create_fund_account(
        args.name,
        args.ifsc,
        args.account_number.clone(),
        contact_id,
        auth.clone(),
    )
    .await;

    if create_fund_account.is_err() {
        return Err(create_fund_account.err().unwrap());
    }

    let fund_account_id = create_fund_account.unwrap();

    let create_payout =
        create_payout(razorpay_x_acccount, fund_account_id, args.amount, auth).await;

    if create_payout.is_err() {
        return Err(create_payout.err().unwrap());
    }

    let payout_id = create_payout.unwrap();

    Ok(payout_id)
}
