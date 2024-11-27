use std::{
    process::exit,
    sync::{Arc, RwLock},
};

use crate::client_http2::{get_http_body, ModeUTF8Check, ProxyApp, ProxyDriver};
use chrono::{Months, NaiveDate};
use serde::{Deserialize, Serialize};

type FloatStr = f64;
type Float = f64;
type DataStr = String;

use serde_aux::prelude::*;
use serde_json::Value;

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
    id: u32,
    rarity: Option<String>,
    quality: Option<String>,
    phase: Option<String>,
    icon_url: String,
    slot: Option<String>,
    type_: Option<String>,
    type_ru: Option<String>,
    weapon: String,
    hero: Option<String>,
    full_name: String,
    full_name_ru: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    steam_price: FloatStr,
    profit_percentage: Float,
    variants: Vec<Variant>,
    sell_items: Vec<SellItems>,
}

#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct Variant {
    id: u32,
    quality: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    sell_price: FloatStr,
    phase: Option<String>,
}
#[derive(Deserialize, Serialize, PartialEq, Default, Debug)]
#[serde(default)]
pub struct SellItems {
    id: u32,
    float: Option<String>,
    sell_price: Float,
    unhold_at: DataStr,
    preview_link: Option<String>,
    inspect_in_game: Option<String>,
    item_stickers: Vec<()>,
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
    let cookie = "cf_clearance=yQ0tNmV3VFnPVUfg4RHAwMtGTAZM8swDxOhL4evbv_I-1732655412-1.2.1.1-WqAkNuBo7VUUygcetpYswoJTPiE4yWGbDspb49.HCaHd.6fQZjHf94dWOX87zEG0Z6nDALp85hCgmXSSOSeiRik4MZiIDyH5qWg3iaSqlWTrvHG4QU5u1xyzhDkCX3Mw038wF2NSSzp1zip_AhvSd8YKTNrsPL6wO7fdo6lUApiT4PjpOiQi5AHVOJW2XEMoZagZv0nzQtdojkdxvh.z6aMd6AyLji7QM3KftBznDdT7KhNUxfYjQVGG3xaJDtMOpLloh1JuoTOLJEESXNjHT8o880OUQ2q5ngJax8Pf27M9a_0ix9uE9q9SSGGKAH7qX0Lbl.ToBrp3dMR.WFDqa4Y_K7PxYq1cIC4mRt0R.Jx45.JkpsDZcJNcKy9gsdoOe3XhDDgocIzgoBx8UfL2ei7cuaeDGMk9BynEKK7ztvoPpIQ85QpAn084AKGWPdEtVAVa9xMZhcsonB9cp1tw0ECNFHljq8f1Sxgs9S4.PG4";
    let proxy_drv = Arc::new(RwLock::new(ProxyDriver::new()));
    let mut page = 1;
    let mut root = Root::default();
    loop {
        let url = format!(
            "https://avan.market/v1/api/users/catalog?app_id={app_id}&currency=1&page={page}"
        );
        let body = get_http_body(&url, ModeUTF8Check::Uncheck, None, Some(cookie))
            .await
            .unwrap();
        dbg!(url);

        let mut root_i: Root = match serde_json::from_str(&body) {
            Ok(c) => c,
            Err(e) => {
                println!("Ошибка чтения avan JSON : {} ", e);
                println!("{body}");
                exit(1);
            }
        };
        if page == 1 {
            root = root_i;
        } else {
            root.data.append(&mut root_i.data);
        }
        page += 1;
        if page > root.page_count {
            break;
        }
    }

    println!("get items - {}", root.data.len());
    assert_eq!(root.count as usize, root.data.len());
    //assert_eq!(root.page_count, root.page);
    let v = parse_steam(proxy_drv.clone(), &root).await;
    dbg!(v);
}

#[derive(Debug)]
struct SteamItem {
    full_name: String,
    avan_price: f64,
    first_sell_price: f64,
    profit: f64,
    sum_cnt: f64,
    sum_sum: f64,
    price_avg: f64,
    price_min: f64,
    price_max: f64,
}
use futures::stream::{self, StreamExt};

async fn parse_steam(proxy_drv: ProxyApp, root: &Root) -> Vec<SteamItem> {
    let check_futures = stream::iter(root.data.iter().map(|item| {
        let value = proxy_drv.clone();
        async move { parse_steam_item(value, &item).await }
    }));

    check_futures.buffered(30).collect().await
}
async fn parse_steam_item(proxy_drv: ProxyApp, item: &Item) -> SteamItem {
    let steam_seller_ratio = 1. - 0.13;
    let app_id = "252490";
    let full_name = item.full_name.clone();
    let url = format!(
        "https://steamcommunity.com/market/listings/{app_id}/{}",
        full_name
    );
    let url = url::Url::parse(&url).unwrap();
    loop {
        let mut body;
        let line1 = loop {
            body = match get_http_body(
                url.as_ref(),
                ModeUTF8Check::Check,
                Some(proxy_drv.clone()),
                None,
            )
            .await
            {
                Ok(b) => b,
                Err(e) => {
                    eprintln!(
                        "{e:?}, поспим {} ms ...",
                        proxy_drv.read().unwrap().sleep_ms_on_net_error
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(
                        proxy_drv.read().unwrap().sleep_ms_on_net_error,
                    ))
                    .await;
                    continue;
                }
            };

            let substr1 = "line1=";
            let substr2 = "g_timePriceHistoryEarliest";
            let Some(line1) = substr(&body, substr1, substr2) else {
                eprintln!("{} не нашли line1, длина body {}...", full_name, body.len());
                proxy_drv.write().unwrap().ban_site("");
                /*tokio::time::sleep(std::time::Duration::from_millis(
                    proxy_drv.read().unwrap().sleep_ms_on_block,
                ))
                .await;*/
                continue;
            };
            break (line1);
        };

        let line1 = line1.trim();
        let line1 = &line1[..line1.len() - 1];

        let steam_sell = match serde_json::from_str::<Vec<SellDay>>(line1) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} json::from_str: {}", full_name, e);
                continue;
            }
        };

        let to_data = chrono::offset::Local::now()
            .date_naive()
            .checked_sub_months(Months::new(1))
            .unwrap();
        let mut sum_cnt = 0.;
        let mut sum_sum = 0.;
        let mut price_max: Float = 0.;
        let mut price_min: Float = 1000000.;

        for steam_day in steam_sell.iter().rev() {
            let (date, _) = NaiveDate::parse_and_remainder(&steam_day.data, "%b %d %Y").unwrap();

            if date < to_data {
                break;
            }
            sum_cnt += steam_day.count;
            sum_sum += steam_day.count * steam_day.price;
            price_max = price_max.max(steam_day.price);
            price_min = price_min.min(steam_day.price);
        }

        let Some(item_id) = find_steam_item_id(&body) else {
            continue;
        };

        let Some(first_sell_price) = item_first_sell_price(&item_id).await else {
            continue;
        };

        //dbg!(sum_cnt, sum_sum, sum_sum / sum_cnt, min_price, max_price);

        let avan_price = if !item.variants.is_empty() {
            item.variants[0].sell_price
        } else {
            0.0
        };

        let profit = first_sell_price * steam_seller_ratio - avan_price;
        let price_avg = sum_sum / sum_cnt;
        println!(
            "{} :{} - тек {} приб {:.2} кол {:.0} сумма {:.2} сред {:.2} мин {} макс {}",
            full_name,
            avan_price,
            first_sell_price,
            profit,
            sum_cnt,
            sum_sum,
            price_avg,
            price_min,
            price_max
        );
        return SteamItem {
            full_name,
            avan_price,
            first_sell_price,
            profit,
            sum_cnt,
            sum_sum,
            price_avg,
            price_min,
            price_max,
        };
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
    for _i in 0..3 {
        let url = format!("https://steamcommunity.com/market/itemordershistogram?country=UA&language=russian&currency=1&item_nameid={item_id}");
        let body = get_http_body(&url, ModeUTF8Check::Uncheck, None, None).await;
        match body {
            Ok(b) => return b,
            Err(e) => {
                println!("{url} {e:?}");
            }
        }
    }
    "".to_string()
}
