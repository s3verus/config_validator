extern crate redis;
use crate::manager::sanity_check;
use redis::{Commands, ControlFlow, PubSubCommands};
use serde::{Deserialize, Serialize};
use std::error::Error;

// what's wrong? can use better form for structures?
#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    pub optional_resolve_to: Option<String>,
    pub address: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tcp {
    pub ip_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub http: Option<Http>,
    pub tcp: Option<Tcp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub data: Data,
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
                let channel: String = msg.get_channel().unwrap();
                // TODO should we check for vec len?
                let channel: Vec<&str> = channel.split(":").collect();
                let channel = match channel.last() {
                    Some(last) => last.to_string(),
                    _ => "".to_string(),
                };

                let message_obj = get_data(&channel).unwrap();
                sanity_check(message_obj, channel).unwrap();
                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

pub fn get_data(key: &str) -> Result<Message, Box<dyn Error>> {
    // TODO hard code!
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut conn = client.get_connection()?;
    // TODO hard code!
    let received: String = conn.get(format!("wild:{}", key))?;
    println!("{}", received);
    let message_obj: Message = serde_json::from_str(&received)?;
    Ok(message_obj)
}

// insert data to redis with sound: prefix
pub fn insert_data(message_obj: Message, channel: String) -> Result<(), Box<dyn Error>> {
    // connect to redis
    // TODO hard code!
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut conn = client.get_connection()?;

    let key = format!("sound:{}", channel);
    let value = serde_json::to_string(&message_obj)?;
    conn.set(key, value)?;
    Ok(())
}
