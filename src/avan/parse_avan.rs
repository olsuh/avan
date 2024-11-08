use crate::client_http2::{get_http_body, ModeUTF8Check};
use chrono::{Months, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io::Write};
type FloatStr = f64;
type Float = f64;
//type DataStr = String;

use serde_aux::prelude::*;
use serde_json::Value;

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Root {
    count: u32,
    //page_count: u32,
    //limit: u32,
    //prev_page: Option<u32>,
    //page: u32,
    //next_page: Option<u32>,
    data: Vec<Item>,
    //#[serde(rename = "rateId")]
    //rate_id: u32,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Item {
    //id: u32,
    //rarity: Option<String>,
    //quality: Option<String>,
    //phase: Option<String>,
    //icon_url: String,
    //slot: Option<String>,
    //type_: Option<String>,
    //type_ru: Option<String>,
    //weapon: String,
    //hero: Option<String>,
    full_name: String,
    //full_name_ru: Option<String>,
    //#[serde(deserialize_with = "deserialize_number_from_string")]
    //steam_price: FloatStr,
    //profit_percentage: Float,
    variants: Vec<Variant>,
    sell_items: Vec<SellItems>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Variant {
    //id: u32,
    //quality: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    sell_price: FloatStr,
    //phase: Option<String>,
}
#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct SellItems {
    //id: u32,
    //float: Option<String>,
    sell_price: Float,
    //unhold_at: DataStr,
    //preview_link: Option<String>,
    //inspect_in_game: Option<String>,
    //item_stickers: Vec<()>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
struct SellDay {
    data: String,
    price: Float,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    count: FloatStr,
}

pub async fn parse_avan() {
    let app_id = "252490";
    let sleep_ms_on_net_error = 2 * 1000;
    let sleep_ms_on_block = 25 * 6 * 1000;
    
    let steam_seller_ratio = 1. - 0.13;

    let url1 =
        format!("https://avan.market/v1/api/users/catalog?app_id={app_id}&currency=1&page=100");
    let body1 = get_http_body(&url1, ModeUTF8Check::Uncheck).await.unwrap();
    dbg!(url1);

    let root: Root = match serde_json::from_str(&body1) {
        Ok(c) => c,
        Err(e) => {
            println!("Ошибка чтения avan JSON : {} ", e);
            let root_0 = Root::default();

            /*
            let mut o_from_file = serde_json::from_str(&body).unwrap();
            let o_def = serde_json::to_value(&root_0).unwrap();

            add_default(&mut o_from_file, &o_def);

            root_0 = serde_json::from_value(o_from_file).unwrap();*/
            root_0
        }
    };

    println!("get items - {}", root.data.len());
    assert_eq!(root.count as usize, root.data.len());
    //assert_eq!(root.page_count, root.page);

    for item in root.data {

        let url = format!(
            "https://steamcommunity.com/market/listings/{app_id}/{}",
            item.full_name
        );
        let url = url::Url::parse(&url).unwrap();

        let mut body;
        let line1 = loop {
            body = match get_http_body(url.as_ref(), ModeUTF8Check::Uncheck).await {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{e:?}, поспим {} ms ...", sleep_ms_on_net_error);
                    tokio::time::sleep(std::time::Duration::from_millis(sleep_ms_on_net_error)).await;
                    continue;
                }
            };

            let substr1 = "line1=";
            let substr2 = "g_timePriceHistoryEarliest";
            let Some(line1) = substr(&body, substr1, substr2) else {
                eprintln!(
                    "{} не нашли line1, длина body {}, поспим {} ms ...",
                    item.full_name,
                    body.len(),
                    sleep_ms_on_block
                );
                tokio::time::sleep(std::time::Duration::from_millis(sleep_ms_on_block)).await;
                continue;
            };
            break (line1);
        };

        let line1 = line1.trim();
        let line1 = &line1[..line1.len() - 1];

        let steam_sell = match serde_json::from_str::<Vec<SellDay>>(line1) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} json::from_str: {}", item.full_name, e);
                continue;
            }
        };

        let to_data = chrono::offset::Local::now()
            .date_naive()
            .checked_sub_months(Months::new(1))
            .unwrap();
        let mut sum_cnt = 0.;
        let mut sum_sum = 0.;
        let mut max_price: Float = 0.;
        let mut min_price: Float = 1000000.;

        for steam_day in steam_sell.iter().rev() {
            let (date, _) = NaiveDate::parse_and_remainder(&steam_day.data, "%b %d %Y").unwrap();

            if date < to_data {
                break;
            }
            sum_cnt += steam_day.count;
            sum_sum += steam_day.count * steam_day.price;
            max_price = max_price.max(steam_day.price);
            min_price = min_price.min(steam_day.price);
        }

        let Some(item_id) = find_steam_item_id(&body) else {
            continue;
        };

        let Some(steam_first_sell_price) = item_first_sell_price(&item_id).await else {
            continue;
        };

        //dbg!(sum_cnt, sum_sum, sum_sum / sum_cnt, min_price, max_price);

        let avan_price = if !item.variants.is_empty() {
            item.variants[0].sell_price
        } else {
            0.0
        };

        println!(
            "{} :{} - тек {} приб {:.2} кол {:.0} сумма {:.2} сред {:.2} мин {} макс {}",
            item.full_name,
            avan_price,
            steam_first_sell_price,
            steam_first_sell_price * steam_seller_ratio - avan_price,
            sum_cnt,
            sum_sum,
            sum_sum / sum_cnt,
            min_price,
            max_price
        );

        get_steam_info(&item).await;
        item_orders_histogram(&item).await;
        //tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;

    }
}

pub fn substr<'a>(str: &'a str, substr1: &str, substr2: &str) -> Option<&'a str> {
    let Some(beg_pos) = str.find(substr1) else {
        return None;
    };
    let beg_pos = beg_pos + substr1.len();
    let next_str = &str[beg_pos..];
    let Some(end_pos) = next_str.find(substr2) else {
        return None;
    };
    Some(&next_str[..end_pos])
}


fn find_steam_item_id(body: &str) -> Option<String> {
    // Market_LoadOrderSpread( 176250984 );
    let substr1 = "Market_LoadOrderSpread(";
    let substr2 = ")";
    let Some(item_id) = substr(body, substr1, substr2) else {
        return None;
    };

async fn get_steam_info(item: &Item) {
    let app_id = "252490";
    let sleep_ms = 2 * 60 * 1000;
    let item_url = item.full_name.replace(" ", "%20");
    let url = format!("https://steamcommunity.com/market/listings/{app_id}/{item_url}");

    let mut body;
    let line1 = loop {
        body = get_http_body(&url, ModeUTF8Check::Uncheck).await.unwrap();
        //dbg!(url);

        let substr1 = "line1=";
        let substr2 = "g_timePriceHistoryEarliest";
        let Some(line1) = substr(&body, substr1, substr2) else {
            eprintln!(
                "{} не нашли line1, длина body {}",
                item.full_name,
                body.len()
            );
            println!("{url}");
            println!("поспим {} ...", sleep_ms);
            tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
            continue;
        };
        break (line1);
    };

    let line1 = line1.trim();
    let line1 = &line1[..line1.len() - 1];

    let steam_sell = match serde_json::from_str::<Vec<SellDay>>(line1) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} json::from_str: {}", item.full_name, e);
            return ;
        }
    };

    let to_data = chrono::offset::Local::now()
        .date_naive()
        .checked_sub_months(Months::new(1))
        .unwrap();
    let mut sum_cnt = 0.;
    let mut sum_sum = 0.;
    let mut max_price: Float = 0.;
    let mut min_price: Float = 1000000.;
    //add current price in steam
    let mut current_price: Float = 0.;

    for steam_day in steam_sell.iter().rev() {
        let (date, _) = NaiveDate::parse_and_remainder(&steam_day.data, "%b %d %Y").unwrap();

        if date < to_data {
            break;
        }
        sum_cnt += steam_day.count;
        sum_sum += steam_day.count * steam_day.price;
        max_price = max_price.max(steam_day.price);
        min_price = min_price.min(steam_day.price);

    }

    //dbg!(sum_cnt, sum_sum, sum_sum / sum_cnt, min_price, max_price);

    let avan_price = if !item.variants.is_empty() {
        item.variants[0].sell_price
    } else {
        0.0
    };

    println!(
        "{} {} - тек {:.2} кол {:.0} сумма {:.2} сред {:.2} мин {} макс {}",
        item.full_name,
        avan_price,
        current_price,
        sum_cnt,
        sum_sum,
        sum_sum / sum_cnt,
        min_price,
        max_price
    );

    //tokio::time::sleep(std::time::Duration::from_millis(sleep_ms)).await;
}



async fn item_orders_histogram(item: &Item) {
        //	Market_LoadOrderSpread( 176250984 );
        let item_id = item.full_name.replace(" ", "%20");
        let substr1 = "Market_LoadOrderSpread(";
        let substr2 = ")";
        let url = format!("https://steamcommunity.com/market/itemordershistogram?country=UA&language=russian&currency=1&item_nameid={item_id}");
        let body = get_http_body(&url, ModeUTF8Check::Uncheck)
            .await
            .unwrap();
        let Some(item_id) = substr(&body, substr1, substr2) else {
            eprintln!("{} не нашли item_id", item.full_name);
            return;
        };


    let item_id = item_id.trim();
    let item_id = String::from(item_id);
    Some(item_id)
}

async fn item_first_sell_price(item_id: &str) -> Option<f64> {
    let body = item_orders_histogram(item_id).await;
    let v: Value = serde_json::from_str(&body).unwrap();
    let a = v.get("sell_order_graph").unwrap().as_array().unwrap();
    let ret = a[0][0].as_f64();
    //dbg!(ret);
    ret
}


async fn item_orders_histogram(item_id: &str) -> String {
    let url = format!("https://steamcommunity.com/market/itemordershistogram?country=UA&language=russian&currency=1&item_nameid={item_id}");
    let body = get_http_body(&url, ModeUTF8Check::Uncheck).await.unwrap();
    body


    //dbg!(url);
    //let v: Value = serde_json::from_str(&body).unwrap();
    //let body = serde_json::to_string_pretty(&v).unwrap();
    /*let file_name = item.full_name.replace(" ", "_")+".json";
    let mut f = fs::File::create(&file_name).expect(&format!("создаем файл {file_name}"));
    f.write_all(body.as_bytes()).expect(&format!("пишем body в файл {file_name}"));
    println!("записали в файл {file_name}... спим 2 минуты...");*/
}

/*fn add_default(a: &mut Value, def: &Value) {
    if let (&mut Value::Object(ref mut a), &Value::Object(ref def)) = (a, def) {
        for (k, v) in def {
            add_default(a.entry(k.as_str()).or_insert(v.clone()), v);
        }
    }
}*/