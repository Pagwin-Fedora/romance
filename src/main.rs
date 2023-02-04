extern crate tokio;
extern crate dirs;
extern crate async_trait;
extern crate serde;
extern crate serde_json;
extern crate uuid;
mod job;
mod env;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
