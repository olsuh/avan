use crate::client_http2;
use serde::{Deserialize, Serialize};

type FloatStr = f64;
type Float = f64;
type DataStr = String;

use serde_aux::prelude::*;

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Root {
    count: u32,
    page_count: u32,
    limit: u32,
    prev_page: Option<u32>,
    page: u32,
    next_page: Option<u32>,
    data: Vec<Item>,
    #[serde(rename = "rateId")]
    rate_id: u32,
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

            /*
            let mut o_from_file = serde_json::from_str(&body).unwrap();
            let o_def = serde_json::to_value(&root_0).unwrap();

            add_default(&mut o_from_file, &o_def);

            root_0 = serde_json::from_value(o_from_file).unwrap();*/
            root_0
        }
    };

    println!("{root:?}");
}

/*fn add_default(a: &mut Value, def: &Value) {
    if let (&mut Value::Object(ref mut a), &Value::Object(ref def)) = (a, def) {
        for (k, v) in def {
            add_default(a.entry(k.as_str()).or_insert(v.clone()), v);
        }
    }
}*/
