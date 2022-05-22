use chrono::Duration;
use paddle_contracted_mrr::calculate_month_contracted_mrr;
use std::env;

#[tokio::main]
async fn main() {
    let selected_date = env::var("DATE");
    let vendor_id = env::var("VENDOR_ID").expect("VENDOR_ID is required");
    let vendor_auth_code = env::var("VENDOR_AUTH_CODE").expect("VENDOR_AUTH_CODE is required");

    match selected_date {
        Ok(date) => {
            let contracted_mrr =
                calculate_month_contracted_mrr(date.clone(), vendor_id, vendor_auth_code).await;

            println!("Contracted MRR in {}: {}", date, contracted_mrr.floor());
        }
        Err(_) => {
            println!("DATE environment variable not provided. Listing next 12 months...");

            for i in 0..12 {
                let date = chrono::Utc::now() + Duration::days(30 * i);
                let formatted_date = date.format("%Y-%m").to_string();
                let contracted_mrr = calculate_month_contracted_mrr(
                    formatted_date.clone(),
                    vendor_id.clone(),
                    vendor_auth_code.clone(),
                )
                .await;

                println!(
                    "Contracted MRR in {}: {}",
                    formatted_date,
                    contracted_mrr.floor()
                );
            }
        }
    }
}
