/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p unsplash-api-demo --bin search_photos -- 'YOUR_ACCESS_KEY' 'dogs'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use unsplash_api::endpoints::search_photos::SearchPhotos;

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_key = env::args().nth(1).unwrap();
    let query = env::args().nth(2).unwrap();

    let client = IsahcClient::new()?;

    let search_photos = SearchPhotos::new(access_key, query);

    let ret = client.respond_endpoint(&search_photos).await?;

    println!("{:?}", ret);

    Ok(())
}
