use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

use crate::RAZORPAY_BASE_URL;

pub async fn create_order(amount: u64) -> Result<String, String> {
    let razorpay_public_key = option_env!("RAZORPAY_API_KEY").expect("RAZORPAY_API_KEY not set");
    let razorpay_secret_key =
        option_env!("RAZORPAY_SECRET_KEY").expect("RAZORPAY_SECRET_KEY not set");
    let auth_string = format!("{}:{}", razorpay_public_key, razorpay_secret_key);

    let encoded_auth = BASE64.encode(auth_string.as_bytes());
    let headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Basic {}", encoded_auth),
        },
    ];

    let body = serde_json::json!({
        "amount": amount,
        "currency": "INR",
    });

    let body_bytes = serde_json::to_vec(&body).unwrap();

    let request = CanisterHttpRequestArgument {
        url: format!("{}orders", RAZORPAY_BASE_URL),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers,
        body: Some(body_bytes),
        transform: None,
    };

    let response = http_request(request, 1_603_131_200).await;

    match response {
        Ok(response) => {
            let response_body = String::from_utf8(response.0.body).unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_body).unwrap();
            ic_cdk::println!("response_json: {:?}", response_json);
            let id = response_json["id"].as_str().expect("Something went wrong");

            Ok(id.to_string())
        }
        Err(err) => Err(format!("Error creating order: {}", err.1)),
    }
}
