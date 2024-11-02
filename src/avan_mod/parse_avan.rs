
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

	

}


