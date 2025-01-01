use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePayout {
    pub account_number: String,
    pub fund_account_id: String,
    pub amount: u64,
    pub currency: String,
    pub mode: String,
    pub purpose: String,
}

impl CreatePayout {
    pub fn to_json_string(&self) -> String {
        format!(
            "{{\"account_number\":\"{}\",\"fund_account_id\":\"{}\",\"amount\":{},\"currency\":\"{}\",\"mode\":\"{}\",\"purpose\":\"{}\"}}",
            self.account_number,
            self.fund_account_id,
            self.amount,
            self.currency,
            self.mode,
            self.purpose
        )
    }
}
