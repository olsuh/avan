pub mod avan;
pub mod client_http2;

#[tokio::main]
async fn main() {
    avan::parse_avan::parse_avan().await;

    //avan_mod::parse_avan_old::get_response().await;
}