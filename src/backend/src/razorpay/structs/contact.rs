use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub contact: String,
    #[serde(rename = "type")]
    pub contact_type: String,
}

impl Contact {
    pub fn to_json_string(&self) -> String {
        format!(
            "{{\"name\":\"{}\",\"email\":\"{}\",\"contact\":\"{}\",\"type\":\"{}\"}}",
            self.name, self.email, self.contact, self.contact_type
        )
    }
}
