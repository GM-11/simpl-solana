use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use serde::{Deserialize, Serialize};

use super::{BankAccount, Contact, CreatePayout, FundAccount};

const BASE_URL: &str = "https://api.razorpay.com/v1/";

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayoutResponse {
    status: String,
}

pub struct Config {
    pub api_key_encrypted: String,
    pub idempotent_key: String,
}

impl Config {
    pub fn new(api_key_encrypted: String, idempotent_key: String) -> Self {
        Self {
            api_key_encrypted,
            idempotent_key,
        }
    }
    async fn create_contact(&self, contact: Contact) -> Result<String, String> {
        let url = format!("{}contacts", BASE_URL);

        let headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("{}", self.api_key_encrypted),
            },
        ];

        let json_string = contact.to_json_string();
        let json_utf8: Vec<u8> = json_string.into_bytes();
        let request_body: Option<Vec<u8>> = Some(json_utf8);

        let request = CanisterHttpRequestArgument {
            url: url.to_string(),
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
                    Some(res) => Ok(res.to_string()),
                    None => Err("Error sending transaction".to_string()),
                }
            }
            Err(err) => Err(err.1),
        }
    }
    async fn create_fund_account(&self, fund_account: FundAccount) -> Result<String, String> {
        let url = format!("{}fund_accounts", BASE_URL);

        let headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("{}", self.api_key_encrypted),
            },
        ];

        let json_string = fund_account.to_json_string();
        let json_utf8: Vec<u8> = json_string.into_bytes();
        let request_body: Option<Vec<u8>> = Some(json_utf8);

        let request = CanisterHttpRequestArgument {
            url: url.to_string(),
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
                    Some(res) => Ok(res.to_string()),
                    None => Err("Error sending transaction".to_string()),
                }
            }
            Err(err) => Err(err.1),
        }
    }

    async fn create_payout(&self, payout: CreatePayout) -> Result<String, String> {
        let url = format!("{}payouts", BASE_URL);

        let headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("{}", self.api_key_encrypted),
            },
        ];

        let json_string = payout.to_json_string();
        let json_utf8: Vec<u8> = json_string.into_bytes();
        let request_body: Option<Vec<u8>> = Some(json_utf8);

        let request = CanisterHttpRequestArgument {
            url: url.to_string(),
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
                    Some(res) => Ok(res.to_string()),
                    None => Err("Error sending transaction".to_string()),
                }
            }
            Err(err) => Err(err.1),
        }
    }

    async fn approve_payout(&self, payout_id: String) -> Result<String, String> {
        let url = format!("{}/payouts/{}/approve", BASE_URL, payout_id);

        let headers = vec![
            HttpHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("{}", self.api_key_encrypted),
            },
        ];

        let request = CanisterHttpRequestArgument {
            url: url.to_string(),
            max_response_bytes: None, //optional for request
            method: HttpMethod::POST,
            headers,
            body: None,
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
                    Some(res) => Ok(res.to_string()),
                    None => Err("Error sending transaction".to_string()),
                }
            }
            Err(err) => Err(err.1),
        }
    }
    pub async fn pay(
        &self,
        name: String,
        email: String,
        contact: String,
        account_number: String,
        ifsc: String,
        amount: u64,
    ) -> Result<String, String> {
        let contact = Contact {
            name: name.clone(),
            email,
            contact,
            contact_type: "customer".to_string(),
        };

        let contact_id = self.create_contact(contact).await;
        if contact_id.is_err() {
            return Err(contact_id.err().unwrap());
        }

        let fund = self
            .create_fund_account(FundAccount {
                contact_id: contact_id.unwrap(),
                account_type: "bank_account".to_string(),
                bank_account: BankAccount {
                    name: name.clone(),
                    account_number: account_number.clone(),
                    ifsc,
                },
            })
            .await;

        if fund.is_err() {
            return Err(fund.err().unwrap());
        }

        let payout = self
            .create_payout(CreatePayout {
                account_number,
                fund_account_id: fund.unwrap(),
                amount,
                currency: "INR".to_string(),
                mode: "IMPS".to_string(),
                purpose: "payout".to_string(),
            })
            .await;

        if payout.is_err() {
            return Err(payout.err().unwrap());
        }

        let payout_id = payout.unwrap();
        let approve = self.approve_payout(payout_id).await;
        if approve.is_err() {
            return Err(approve.err().unwrap());
        }

        Ok(approve.unwrap().to_string())
    }
}
