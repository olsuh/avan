<<<<<<< HEAD
use std::collections::HashMap;

use crate::client_http2;
use rustls::crypto::hash::Hash;
=======
use std::{fs, io::Write, time::Duration};

use crate::client_http2::{get_http_body, ModeUTF8Check};
>>>>>>> 51afc98d71518a30773b9764ff0c4cf6312e2e40
use serde::{Deserialize, Serialize};

type FloatStr = f64;
type Float = f64;

use serde_aux::prelude::*;
use serde_json::Value;
use tokio::time::sleep;

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
    let app_id = "252490";
    let url = format!("https://avan.market/v1/api/users/catalog?app_id={app_id}&currency=1&page=100");
    let body = get_http_body(&url, ModeUTF8Check::Uncheck)
        .await
        .unwrap();
    dbg!(url);

    let root: Root = match serde_json::from_str(&body) {
        Ok(c) => c,
        Err(e) => {
            println!("Ошибка чтения JSON : {} ", e);
            let root_0 = Root::default();

            root_0
        }
    };

<<<<<<< HEAD
    println!("{root:#?}");
}

pub fn convert_item() {
    let item: HashMap<Item, u16> = HashMap::new();
=======
    println!("get items - {}", root.data.len());
    assert_eq!(root.count as usize, root.data.len());
    assert_eq!(root.page_count, root.page);

    for item in root.data {

        let item_url = item.full_name.replace(" ", "%20");

        let url = format!("https://steamcommunity.com/market/listings/{app_id}/{item_url}");
        let body = get_http_body(&url, ModeUTF8Check::Uncheck)
            .await
            .unwrap();
        dbg!(url);
        //	Market_LoadOrderSpread( 176250984 );
        let substr1 = "Market_LoadOrderSpread(";
        let substr2 = ")";
        let Some(beg_pos) = body.find(substr1) else {continue};
        let beg_pos = beg_pos + substr1.len();
        let next_str = &body[beg_pos..];
        let Some(end_pos) = next_str.find(substr2) else {continue};
        let item_id = &next_str[..end_pos];

        let item_id = String::from(item_id);
        let item_id = item_id.trim();
        dbg!(&item_id);

        let url = format!("https://steamcommunity.com/market/itemordershistogram?country=UA&language=russian&currency=1&item_nameid={item_id}");
        let body = get_http_body(&url, ModeUTF8Check::Uncheck)
            .await
            .unwrap();

        dbg!(url);
        let v: Value = serde_json::from_str(&body).unwrap();
        let body = serde_json::to_string_pretty(&v).unwrap();
        let file_name = item.full_name.replace(" ", "_")+".json";
        let mut f = fs::File::create(&file_name).expect(&format!("создаем файл {file_name}"));
        f.write_all(body.as_bytes()).expect(&format!("пишем body в файл {file_name}"));
        println!("записали в файл {file_name}... спим 2 минуты...");

        sleep(Duration::from_millis(2*60*1000)).await;
    }


>>>>>>> 51afc98d71518a30773b9764ff0c4cf6312e2e40

}

