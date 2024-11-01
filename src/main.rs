pub mod client_http2;
pub mod avan_mod;

#[tokio::main]
async fn main() {
    
    let url: &str = "https://avan.market/v1/api/users/catalog?app_id=252490&currency=2&page=30";
    let body = client_http2::run_client_http2(url, client_http2::Mode::Uncheck).await.unwrap();

    println!("{body}");

    //avan_mod::parse_avan::get_response().await;

}

