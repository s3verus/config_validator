extern crate redis;
use crate::manager::sanity_check;
use redis::RedisError;
use redis::{Commands, ControlFlow, PubSubCommands};
use serde::{Deserialize, Serialize};
use std::error::Error;

// what's wrong? can use better form for structures?
#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    optional_resolve_to: Option<String>,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tcp {
    ip_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    http: Option<Http>,
    tcp: Option<Tcp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    data: Data,
    message: String,
    is_checked: bool,
}

pub fn psubscribe(channel: String) -> Result<(), Box<dyn Error>> {
    // TODO read about tokio and async...
    let _ = tokio::spawn(async move {
        // TODO add logger and handle unwraps
        // TODO hard code! add config file to project...
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let mut conn = client.get_connection().unwrap();

        let _: () = conn
            .psubscribe(&[channel], |msg| {
                let received: String = msg.get_payload().unwrap();
                let channel: String = msg.get_channel().unwrap();
                let message_obj: Message = serde_json::from_str(&received).unwrap();

                sanity_check(message_obj, channel);

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

// insert data to redis with sound: prefix
pub fn insert_data(message_obj: Message, channel: String) -> Result<(), RedisError> {
    // connect to redis
    // TODO hard code!
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut conn = client.get_connection()?;

    let channel: Vec<&str> = channel.split(":").collect();
    let key = format!("sound:{}", channel[1]);
    let value = serde_json::to_string(&message_obj).unwrap();
    conn.set(key, value)?;
    Ok(())
}
