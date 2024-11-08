// Simple HTTPS GET client based on hyper-rustls
use bytes::Bytes;
use http::{Method, Request};
use http_body_util::{BodyExt, Empty};
use hyper_rustls::ConfigBuilderExt;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use std::io;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

pub enum ModeUTF8Check {
    Uncheck,
    Check,
    Lossy,
}

pub async fn get_http_body(url: &str, mode_utf8_check: ModeUTF8Check) -> io::Result<String> {
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
        .enable_all_versions()
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
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0",
        )
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
    };

    fut.await
}