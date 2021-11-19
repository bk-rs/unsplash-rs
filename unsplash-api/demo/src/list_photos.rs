/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p unsplash-api-demo --bin list_photos -- 'YOUR_ACCESS_KEY'
*/

use std::{env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use unsplash_api::endpoints::list_photos::ListPhotos;

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_key = env::args().nth(1).unwrap();

    let client = IsahcClient::new()?;

    let list_photos = ListPhotos::new(access_key);

    let ret = client.respond_endpoint(&list_photos).await?;

    println!("{:?}", ret);

    Ok(())
}
