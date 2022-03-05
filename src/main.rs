extern crate redis;
use config_validator::dao::psubscribe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    // TODO 1 hard code!
    // TODO 2 handle errors...
    if let Err(error) = psubscribe(String::from("__key*__:wild:*")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }
    Ok(())
}
