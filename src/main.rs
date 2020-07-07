extern crate serde_json;
extern crate tokio;

mod jav;

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    // let av = block_on(jav::sources::indexav_com::search_by_actress("飯岡かなこ"));
    let av = block_on(jav::sources::avgle_com::search_by_code("ABP-1"));

    println!("{}", serde_json::to_string(&av.ok()).unwrap());
}
