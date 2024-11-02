pub mod client_http2;
pub mod avan;

#[tokio::main]
async fn main() {
    
    avan::parse_avan::parse_avan().await;

    //avan_mod::parse_avan_old::get_response().await;

}

