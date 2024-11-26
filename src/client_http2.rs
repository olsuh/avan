// Simple HTTPS GET client based on hyper-rustls
use bytes::Bytes;
use hashbrown::HashMap;
use http::{Method, Request};
use http_body_util::{BodyExt, Empty};
use hyper_rustls::ConfigBuilderExt;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::{
    fs,
    io::{self, Read},
    sync::{Arc, RwLock},
};
use ureq::AgentBuilder;

use crate::avan::parse_avan::substr;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

pub enum ModeUTF8Check {
    Uncheck,
    Check,
    Lossy,
}
pub async fn get_http_body(
    url: &str,
    mode_utf8_check: ModeUTF8Check,
    proxy_drv: Option<ProxyApp>,
) -> io::Result<String> {
    match proxy_drv {
        None => get_http_body_2(url, mode_utf8_check).await,
        Some(p_d) => get_http_body_0(url, mode_utf8_check, p_d).await,
    }
}

/*#[derive(Debug,Default)]
pub struct Proxy {
    url: String,
    cnt_test_bad: u16,
    cnt_bad: u32,
}*/

// 157.245.59.236:8888
//152.228.134.212:58044

pub fn load_proxy_list(file_name: &str) -> Vec<String> {
    let mut proxies = Vec::<String>::new();
    let mut f = fs::File::open(file_name).expect(&format!("загружаем {file_name}"));
    let mut buf = String::new();
    let _x = f.read_to_string(&mut buf);
    let x = 1;
    let sh = 2;
    let shema = substr(file_name, "", ".").unwrap_or("http");
    //let shema = if shema == "http" {"https"} else {shema};
    for l in buf.lines().enumerate() {
        if l.0 % x == 0 {
            let proxy = if x != 1 {
                let ip = substr(l.1, "", "\t").unwrap();
                let port = substr(l.1, "\t", "\t").unwrap();
                format!("{shema}://{ip}:{port}")
            } else if sh == 2 {
                let shema = if l.1.contains("https") {
                    "http"
                } else {
                    "http"
                };
                let ip = substr(l.1, "ip: \"", "\"").unwrap();
                let port = substr(l.1, "port: \"", "\"").unwrap();
                format!("{shema}://{ip}:{port}")
            } else {
                format!("{shema}://{}", l.1)
            };
            proxies.push(proxy);
            /*let _x = match get_http_body_0("https://httpbin.org/get", ModeUTF8Check::Uncheck, &mut proxies) {
                Ok(b) => b,
                Err(e) => { println!("{e}"); "".to_string() },
            };*/
        }
    }
    println!("загрузили proxy {}", proxies.len());
    proxies
}

pub fn get_http_body_00(
    url: &str,
    _mode_utf8_check: ModeUTF8Check,
    proxies: &mut Vec<String>,
) -> io::Result<String> {
    use ureq::Proxy;
    let body;
    loop {
        let Some(proxy) = proxies.last() else {
            return Err(error("proxy закончились".to_string()));
        };
        let proxy = format!("{proxy}");
        println!("use proxy {proxy} ...");

        let proxy = Proxy::new(&proxy).map_err(|e| error(format!("Proxy: {e:?}")))?;
        let agent = AgentBuilder::new().proxy(proxy).build();

        //let url = "https://httpbin.org/get";

        let resp = agent
            .get(url)
            .set("User-Agent", user_agent(false).as_str())
            .call()
            .map_err(|e| {
                let p = proxies.pop().unwrap();
                eprintln!("proxy {p} - bad");
                error(format!("Get ureq: {e:?} {url}"))
            })?;

        body = resp.into_string().unwrap();
        break;
    }

    //dbg!(&body);

    Ok(body)
}

fn user_agent(no_proxy: bool) -> String {
    use rand;
    const LEN_ARRAY: usize = 6;
    const USER_AGENT: [&str; LEN_ARRAY]  = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.5993.2470 YaBrowser/23.11.0.2470 Yowser/2.5 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 KAKAOTALK 9.0.3"];
    if no_proxy {
        return USER_AGENT[0].to_string(); //3
    }
    let mut rng = rand::thread_rng();
    let i = rand::Rng::gen_range(&mut rng, 0..LEN_ARRAY);
    let from = rand::Rng::gen_range(&mut rng, '0'..'9');
    let to = rand::Rng::gen_range(&mut rng, '0'..'9').to_string();
    let u_a = USER_AGENT[i].replace(from, &to);
    u_a
}

pub type ProxyApp = Arc<RwLock<ProxyDriver>>;

/*
#[derive(Debug, Default)]
pub struct Proxy {
    proxy: String,
    ban_proxy: usize,
    ban_site: usize,
}*/

#[derive(Debug, Default)]
pub struct ProxyDriver {
    proxies: HashMap<String, (usize, usize, usize)>,
    _work_weight_index_proxy: HashMap<usize, usize>,
    _site_ban_index_proxy: HashMap<usize, usize>,
    _cur_proxy: usize,
    now_working_direct: bool,
    _moment_ban_site: u64,
    pub sleep_ms_on_net_error: u64,
    pub sleep_ms_on_block: u64,
    pub db_path: String,
}
/*let sleep_ms_on_net_error = 0; //2 * 1000;
let sleep_ms_on_block = 0; //25 * 6 * 1000;
let steam_seller_ratio = 1. - 0.13;*/

impl ProxyDriver {
    pub fn ban_site(&mut self) {
        if self.now_working_direct {
            //let selfmoment_ban_site = std::time::SystemTime::now()..duration_since(earlier)
        };
    }
    pub fn ban_work(&mut self, proxy: &str) {
        if !proxy.is_empty() {
            let (_proxy_k, (work, _ban, _load)) = self.proxies.get_key_value_mut(proxy).unwrap();
            *work += 1;
        };
    }
    /*pub fn default() -> Self {
        let i = Instant::now().elapsed();
        Self{
            proxies: todo!(),
            work_weight_index_proxy: todo!(),
            site_ban_index_proxy: todo!(),
            cur_proxy: todo!(),
            moment_ban_site: Instant::now().,
        }
    }*/
    pub fn new() -> Self {
        let mut self_ = Self {
            db_path: r#"C:\Git\Rust\proxy\free_proxy_pool\proxies.db"#.into(),
            ..Self::default()
        };
        self_.load_proxies();
        self_.now_working_direct = true;
        self_
    }

    pub fn load_proxies(&mut self) {
        use rusqlite::Connection;
        let conn = Connection::open(&self.db_path).unwrap();

        let mut stmt = conn
            .prepare("SELECT ip, port, proxy_type FROM live_proxies;")
            .unwrap();
        let rows = stmt
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .unwrap();

        for row in rows {
            let (ip, port, proxy_type) = row.unwrap();
            let proxy_type = proxy_scheme(&proxy_type, true);
            let proxy_url = format!("{}://{}:{}", proxy_type, ip, port);
            self.proxies.insert(proxy_url, (0, 0, 0));
        }
    }

    pub fn next(&mut self) -> String {
        for need_load in 0..100 {
            for i in self.proxies.iter_mut() {
                let (proxy, (work, ban, load)) = i;
                if *work == 0 && *ban == 0 && need_load >= *load {
                    *load += 1;
                    return proxy.clone();
                }
            }
        }
        "".into()
    }
}

pub async fn get_http_body_0(
    url: &str,
    mode_utf8_check: ModeUTF8Check,
    proxy_drv: ProxyApp,
) -> io::Result<String> {
    let body;
    loop {
        //let url = "https://httpbin.org/get";

        let mut client_builder = reqwest::Client::builder();
        let proxy = proxy_drv.write().unwrap().next();
        let no_proxy = proxy.is_empty();
        if !no_proxy {
            println!("use proxy {proxy} ...");
            client_builder = client_builder.proxy(match reqwest::Proxy::all(&proxy) {
                Ok(p) => p,
                Err(e) => {
                    proxy_drv.write().unwrap().ban_work(&proxy);
                    println!("Client builder, proxy: {} : {:?}", &proxy, e);
                    continue;
                }
            });
        }
        let client = client_builder.build().unwrap();

        let res = client
            .get(url)
            .header("User-Agent", user_agent(no_proxy))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await;

        let res = match res {
            Ok(r) => r,
            Err(e) => {
                proxy_drv.write().unwrap().ban_work(&proxy);
                println!("Get request: {e:?} {}", &proxy);
                //error(format!("Get request: {e:?} {}", &proxy))
                continue;
            }
        };

        let bytes = match res.bytes().await {
            Ok(b) => b,
            Err(e) => {
                proxy_drv.write().unwrap().ban_work(&proxy);
                println!("Bytes request: {e:?} {}", &proxy);
                continue;
            }
        };

        body = body_bite_to_strig(bytes, mode_utf8_check).unwrap();
        break;
    }

    //dbg!(&body);

    Ok(body)
}

pub async fn get_http_body_2(url: &str, mode_utf8_check: ModeUTF8Check) -> io::Result<String> {
    // Set a process wide default crypto provider.
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Default TLS client config with native roots
    let tls = rustls::ClientConfig::builder()
        .with_native_roots()?
        .with_no_client_auth();

    // Prepare the HTTPS connector
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http2()
        //.enable_all_versions()
        .build();

    // Build the hyper client from the HTTPS connector.
    let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new()).build(https);

    /*
        let handle = core.handle();
        let proxy = {
            let proxy_uri ="http://<your proxy>:port".parse().unwrap();
            let mut proxy = Proxy::new(Intercept::All, proxy_uri);
            proxy.set_authorization(Basic{
                username: "your username",
                password: Some("your passwd"),
            });
            let http_connector = HttpConnector::new(4, &handle);
            ProxyConnector::from_proxy(http_connector, proxy).unwrap()
        };
        let client2 = hyper::Client::configure().connector(proxy).build(&handle);
    */
    // Prepare a chain of futures which sends a GET request, inspects
    // the returned headers, collects the whole body and prints it to
    // stdout.

    let req = Request::builder()
        .header("User-Agent", user_agent(true))
        .header("Cookie", "cf_clearance=yQ0tNmV3VFnPVUfg4RHAwMtGTAZM8swDxOhL4evbv_I-1732655412-1.2.1.1-WqAkNuBo7VUUygcetpYswoJTPiE4yWGbDspb49.HCaHd.6fQZjHf94dWOX87zEG0Z6nDALp85hCgmXSSOSeiRik4MZiIDyH5qWg3iaSqlWTrvHG4QU5u1xyzhDkCX3Mw038wF2NSSzp1zip_AhvSd8YKTNrsPL6wO7fdo6lUApiT4PjpOiQi5AHVOJW2XEMoZagZv0nzQtdojkdxvh.z6aMd6AyLji7QM3KftBznDdT7KhNUxfYjQVGG3xaJDtMOpLloh1JuoTOLJEESXNjHT8o880OUQ2q5ngJax8Pf27M9a_0ix9uE9q9SSGGKAH7qX0Lbl.ToBrp3dMR.WFDqa4Y_K7PxYq1cIC4mRt0R.Jx45.JkpsDZcJNcKy9gsdoOe3XhDDgocIzgoBx8UfL2ei7cuaeDGMk9BynEKK7ztvoPpIQ85QpAn084AKGWPdEtVAVa9xMZhcsonB9cp1tw0ECNFHljq8f1Sxgs9S4.PG4")
        .method(Method::GET)
        .uri(url)
        .body(Empty::new())
        .map_err(|e| {
            error(format!(
                "Request builder: {e:?}, url:{url}, url len:{}, url parse: {:?}",
                url.len(),
                url::Url::parse(url)
            ))
        })?;
    //é

    let fut = async move {
        let res = client
            .request(req)
            .await
            .map_err(|e| error(format!("Could not get: {:?}", e)))?;

        //println!("Status:\n{}", res.status());
        //println!("Headers:\n{:#?}", res.headers());

        let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| error(format!("Could not get body: {:?}", e)))?
            .to_bytes();

        //println!("Body:\n{}", String::from_utf8_lossy(&body));

        let ret = body_bite_to_strig(body, mode_utf8_check).unwrap();
        Ok(ret)
    };

    fut.await
}

pub fn body_bite_to_strig(body: Bytes, mode_utf8_check: ModeUTF8Check) -> io::Result<String> {
    let ret = match mode_utf8_check {
        // вернет без проверки
        ModeUTF8Check::Uncheck => unsafe { String::from_utf8_unchecked(body.to_vec()) },
        // вернет все или ошибку
        ModeUTF8Check::Check => String::from_utf8(body.to_vec())
            .map_err(|e| error(format!("Could not get body: {:?}", e)))?,
        // вернет с потерей символов
        ModeUTF8Check::Lossy => String::from_utf8_lossy(&body).into_owned(),
    };
    Ok(ret)
}

pub fn proxy_scheme(proxy_types: &str, force_https: bool) -> String {
    let proxy_types = proxy_types.to_lowercase();
    let mut proxy_scheme = proxy_types.split(", ").collect::<Vec<_>>()[0];
    if proxy_scheme == "socks4" {
        proxy_scheme = "socks5"
    };
    if force_https && proxy_types.contains("https") {
        proxy_scheme = "https"
    };
    proxy_scheme.to_string()
}
