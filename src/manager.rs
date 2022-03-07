use crate::dao::insert_data;
use crate::dao::Message;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Rules {
    pub illegals: Vec<String>,
}

// read regex from file
pub fn load_rules(path: &str) -> Result<Rules, Box<dyn Error>> {
    // Open file
    let mut file = File::open(path)?;

    // Read the file contents into a string
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let rules: Rules = serde_json::from_str(&s)?;

    Ok(rules)
}

pub fn is_legal(user_input: String) -> Result<bool, Box<dyn Error>> {
    // TODO hard code!
    let rules = load_rules("rules.json")?;
    for rule in rules.illegals {
        // println!("{}", rule);
        let re = Regex::new(&rule)?;
        if re.is_match(&user_input) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn sanity_check(message: Message, channel: String) -> Result<(), Box<dyn Error>> {
    if !message.data.tcp.is_none() {
        let ip_address = match message.data.tcp.as_ref().map(|m| &m.ip_address) {
            Some(x) => x,
            _ => "",
        };
        println!("{:?}", ip_address);
        if is_legal(ip_address.to_string())? {
            insert_data(message, channel)?;
            println!("insert to redis...");
        }
    } else if !message.data.http.is_none() {
        let optional_resolve_to = match message.data.http.as_ref().map(|m| &m.optional_resolve_to) {
            Some(Some(x)) => x,
            _ => "",
        };
        println!("{:?}", optional_resolve_to);
        let address = match message.data.http.as_ref().map(|m| &m.address) {
            Some(x) => x,
            _ => "",
        };
        println!("{:?}", address);
        if is_legal(address.to_string())? && is_legal(optional_resolve_to.to_string())? {
            insert_data(message, channel)?;
            println!("insert to redis...");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_ip() {
        let ip = String::from("192.168.90.1");
        assert_eq!(is_legal(ip).unwrap(), false);
    }

    #[test]
    fn illegal_protocols() {
        let address = String::from("ftp://attacker.com/test");
        assert_eq!(is_legal(address).unwrap(), false);
    }

    #[test]
    fn illegal_chars() {
        let address = String::from("https://domain.tld/[test]/home");
        assert_eq!(is_legal(address).unwrap(), false);
    }

    #[test]
    fn test_localhost() {
        let address = String::from("http://localHost:80/test");
        assert_eq!(is_legal(address).unwrap(), false);
    }

    #[test]
    fn legal_ip() {
        let ip = String::from("70.34.21.16");
        assert!(is_legal(ip).unwrap());
    }
}
