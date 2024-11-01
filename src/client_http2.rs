// Simple HTTPS GET client based on hyper-rustls
//
// First parameter is the mandatory URL to GET.
// Second parameter is an optional path to CA store.
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_rustls::ConfigBuilderExt;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tokio::io;

//use http::Uri;
//use std::str::FromStr;


fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

pub enum Mode {
    Uncheck,
    Check,
    Lossy,
}

pub async fn run_client_http2(url: &str, mode: Mode) -> io::Result<String> {
    // Set a process wide default crypto provider.
    let _ = rustls::crypto::ring::default_provider().install_default();

    //let url= Uri::from_str(url).map_err(|e| error(format!("{}", e)))?;

    // Default TLS client config with native roots
    let tls = rustls::ClientConfig::builder()
            .with_native_roots()?
            .with_no_client_auth();
    
    // Prepare the HTTPS connector
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        //.enable_http1()
        .enable_http2()
        .build();

    // Build the hyper client from the HTTPS connector.
    let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new())
    .build(https);

    // Prepare a chain of futures which sends a GET request, inspects
    // the returned headers, collects the whole body and prints it to
    // stdout.
    use hyper::{Method, Request};
    
    let req = Request::builder()
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0")
    .method(Method::GET)
    .uri(url)
    .body(Empty::new())
    .expect("request builder");

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

        let ret = match mode {
            // вернет без проверки
            Mode::Uncheck => unsafe { String::from_utf8_unchecked(body.to_vec())},
            // вернет все или ошибку
            Mode::Check => String::from_utf8(body.to_vec()).map_err(|e| error(format!("Could not get body: {:?}", e)))?,
            // вернет с потерей символов 
            Mode::Lossy => String::from_utf8_lossy(&body).into_owned(),
        };
        
        Ok(ret)
    };

    fut.await
}