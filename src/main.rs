pub mod avan;
pub mod client_http2;

#[tokio::main]
async fn main() {
    avan::parse_avan::parse_avan().await;
}
