
use reqwest::header::{
    self, HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, COOKIE, HOST, USER_AGENT, ACCEPT_ENCODING};
use std::collections::HashMap;
use reqwest::Version;
pub struct Item {
    item: HashMap<String, u32>,
}

pub struct ItemData {
    items: Vec<Item>,
}

pub async fn get_response() {

	let client = reqwest::Client::builder()
		.build()
        .unwrap();

	let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(COOKIE, "__cflb=02DiuGa87D2dctS5ktMzMpR3kWJLMUPTn86gaAakqtCq2".parse().unwrap());
	headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0".parse().unwrap());
	headers.insert(ACCEPT, "application/json".parse().unwrap());
	headers.insert(HOST, "avan.market".parse().unwrap());
	headers.insert(ACCEPT_LANGUAGE, "ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br, zstd".parse().unwrap());

	let request = client.get("https://avan.market/v1/api/users/catalog?app_id=252490&currency=2&page=30")
		.headers(headers)
        .send()
        .await
        .unwrap();

	let body = request.text().await.unwrap();

	println!("{}", body);

}


