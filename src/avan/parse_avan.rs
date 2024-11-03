use std::collections::HashMap;

use crate::client_http2;
use rustls::crypto::hash::Hash;
use serde::{Deserialize, Serialize};

type FloatStr = f64;
type Float = f64;

use serde_aux::prelude::*;

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Root {
    count: u32,
    page_count: u32,
    data: Vec<Item>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Item {
    full_name: String,
    variants: Vec<Variant>,
    sell_items: Vec<SellItems>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Variant {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    sell_price: FloatStr,
}


#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct SellItems {
    sell_price: Float,
}

pub async fn parse_avan() {
    let url: &str = "https://avan.market/v1/api/users/catalog?app_id=252490&currency=2&page=30";
    let body = client_http2::run_client_http2(url, client_http2::Mode::Uncheck)
        .await
        .unwrap();

    let root: Root = match serde_json::from_str(&body) {
        Ok(c) => c,
        Err(e) => {
            println!("Ошибка чтения JSON : {} ", e);
            let root_0 = Root::default();

            root_0
        }
    };

    println!("{root:#?}");
}

pub fn convert_item() {
    let item: HashMap<Item, u16> = HashMap::new();

}

