use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub async fn calculate_month_contracted_mrr(
    date: String,
    vendor_id: String,
    vendor_auth_code: String,
) -> f64 {
    let mut map = HashMap::new();
    map.insert("vendor_id", vendor_id);
    map.insert("vendor_auth_code", vendor_auth_code);

    let client = reqwest::Client::new();
    let res = match client
        .post("https://vendors.paddle.com/api/2.0/subscription/users")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(serde_urlencoded::to_string(&map).unwrap())
        .send()
        .await
    {
        Ok(it) => it.text().await.unwrap(),
        Err(err) => panic!("{}", err),
    };

    let body: Root = match serde_json::from_str(&res) {
        Ok(it) => it,
        Err(err) => panic!("Failed to serialize API response: {}", err),
    };

    body.response
        .into_iter()
        .filter(|sub| sub.next_payment.date.starts_with(&date))
        .map(|sub| sub.next_payment.amount)
        .reduce(|acc, x| acc + x)
        .unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub success: bool,
    pub response: Vec<Response>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(rename = "subscription_id")]
    pub subscription_id: i64,
    #[serde(rename = "plan_id")]
    pub plan_id: i64,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "user_email")]
    pub user_email: String,

    #[serde(rename = "next_payment")]
    pub next_payment: NextPayment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextPayment {
    pub amount: f64,
    pub currency: String,
    pub date: String,
}
