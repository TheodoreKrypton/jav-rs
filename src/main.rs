#![feature(async_closure)]

extern crate serde_json;
extern crate tokio;

mod jav;

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    // let av = block_on(jav::sources::indexav_com::search_by_actress("飯岡かなこ"));
    // let av = block_on(jav::translate2jp(&("Arina Hashimoto".to_string())));
    let av = block_on(jav::get_brief(&"ABP-123".to_string()));
    println!("{}", serde_json::to_string(&av).unwrap());
}
