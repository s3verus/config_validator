extern crate redis;
use serde::{Serialize, Deserialize};
use std::error::Error;
use redis::{ControlFlow, PubSubCommands};
use crate::manager::sanity_check;

#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    optional_resolve_to: Option<String>,
    address: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tcp {
    ip_address: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    http: Option<Http>,
    tcp: Option<Tcp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    is_checked: bool,
    message: String,
    data: Data
}

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();

    let mut conn = client.get_connection().unwrap();

    let _: () = conn.subscribe(&[channel], |msg| {            
        let received: String = msg.get_payload().unwrap();
        let config_obj: Config = serde_json::from_str(&received).unwrap();

        sanity_check(config_obj);

        return ControlFlow::Continue;
    }).unwrap();
    });

    Ok(())
}
