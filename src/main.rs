use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let API_KEY: String = String::from("api key removed");

    let mut params = HashMap::new();
    params.insert("symbol", "BTC");

    let client = reqwest::Client::new();

    let resp = client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest")
        .header("X-CMC_PRO_API_KEY", API_KEY)
        .query(&params)
        .send()
        .await?;

    let resp = resp.text().await?;

    println!("{:#?}", resp);

    Ok(())
}
