extern crate redis;
use config_validator::dao::subscribe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    if let Err(error) = subscribe(String::from("wild")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }
    Ok(())
}
