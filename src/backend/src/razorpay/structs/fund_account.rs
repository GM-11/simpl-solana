use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FundAccount {
    pub contact_id: String,
    pub account_type: String,
    pub bank_account: BankAccount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BankAccount {
    pub name: String,
    pub account_number: String,
    pub ifsc: String,
}

impl FundAccount {
    pub fn to_json_string(&self) -> String {
        format!(
            "{{\"contact_id\":\"{}\",\"account_type\":\"{}\",\"bank_account\":{{\"name\":\"{}\",\"account_number\":\"{}\",\"ifsc\":\"{}\"}}}}",
            self.contact_id,
            self.account_type,
            self.bank_account.name,
            self.bank_account.account_number,
            self.bank_account.ifsc
        )
    }
}
