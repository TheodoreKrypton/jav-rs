#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde_json;
extern crate tokio;

mod jav;

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let avs = block_on(jav::get_newly_released(1));
    println!("{:?}", avs);
}
