use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PaymentBody {
    pub amount: u64,
    pub currency: String,
}

pub async fn pay_to_treasury(amount_in_inr: u64, auth: String) -> String {
    let url = format!("https://api.razorpay.com/v1/orders");

    let headers = vec![
        HttpHeader {
            name: "content-type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Basic {}", auth),
        },
    ];

    let body = PaymentBody {
        amount: amount_in_inr,
        currency: "INR".to_string(),
    };

    let json_string = serde_json::to_string(&body).unwrap();
    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None, //optional for request
        method: HttpMethod::POST,
        headers,
        body: request_body,
        transform: None, //optional for request
    };

    let response = http_request(request, 1_803_143_600u128).await;

    match response {
        Ok((res,)) => {
            String::from_utf8(res.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((r, m)) => {
            format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}")
        }
    }
}
