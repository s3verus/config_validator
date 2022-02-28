use crate::dao::insert_data;
use crate::dao::Message;
use regex::Regex;

pub fn is_legal(user_input: String) -> bool {
    // check for illegal protocols:
    let re = Regex::new(r"(?i)(((ftps|gopher|telnet|nntp|file|php|phar|data|dict|sftp|ldap|tftp|mailto|news|ftp):\\*/*))").unwrap();
    if re.is_match(&user_input) {
        return false;
    }

    // check for localhost:
    let re = Regex::new(r"(?i)((https|http)://)?(:(/|\\)+(0|⓪|%80%820)|localhost|(%80%820|①②⑦|⓪|0|127)(\.|%E3)?(%80%820|0|⓪){0,3}(\.|%E3)?(%80%820|0|⓪){0,3}(\.|%E3)(%80%820|%80%821|①|⓪|0|1){1,3})").unwrap();
    if re.is_match(&user_input) {
        return false;
    }

    // check for localhost bypass:
    let re = Regex::new(r"(?i)(\.\.|\[+((0|⓪){1,4}|f{1,4}|:+|(1|①)?|((https|http)://)?(:(/|\\)+(0|⓪)|localhost|(⓪|0|127|①②⑦)\.?(0|⓪){0,3}\.?(0|⓪){0,3}\.(1|0|⓪|①){1,3}))*\]+)").unwrap();
    if re.is_match(&user_input) {
        return false;
    }

    // check for illegal chars:
    let re =
        Regex::new(r"(%7B|%7D|%7C|%5C|%5E|~|%5B|%5D|%60|\[|\]|\|+|\.\.|%0a|;/?|\{|\}|\*)").unwrap();
    if re.is_match(&user_input) {
        return false;
    }

    true
}

pub fn sanity_check(message: Message, channel: String) {
    if !message.data.tcp.is_none() {
        let ip_address = match message.data.tcp.as_ref().map(|m| &m.ip_address) {
            Some(x) => x,
            _ => "",
        };
        println!("{:?}", ip_address);
        if is_legal(ip_address.to_string()) {
            insert_data(message, channel).unwrap();
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
        if is_legal(address.to_string()) && is_legal(optional_resolve_to.to_string()) {
            insert_data(message, channel).unwrap();
            println!("insert to redis...");
        }
    }
}
