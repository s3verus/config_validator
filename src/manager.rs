use crate::dao::insert_data;
use crate::dao::Message;

pub fn is_legal(_user_input: String) -> bool {
    // check for illegal protocols:
    // /(((ftps|gopher|telnet|nntp|file|php|phar|data|dict|sftp|ldap|tftp|mailto|news|ftp):\\*\/*))/gi

    // check for localhost:
    // /((https|http):\/\/)?(:(\/|\\)+(0|⓪|%80%820)|localhost|(%80%820|①②⑦|⓪|0|127)(\.|%E3)?(%80%820|0|⓪){0,3}(\.|%E3)?(%80%820|0|⓪){0,3}(\.|%E3)(%80%820|%80%821|①|⓪|0|1){1,3})/gi

    // check for localhost bypass:
    // /(\.\.|\[+((0|⓪){1,4}|f{1,4}|:+|(1|①)?|((https|http):\/\/)?(:(\/|\\)+(0|⓪)|localhost|(⓪|0|127|①②⑦)\.?(0|⓪){0,3}\.?(0|⓪){0,3}\.(1|0|⓪|①){1,3}))*\]+)/gi

    // check for illegal chars:
    // /(%7B|%7D|%7C|%5C|%5E|~|%5B|%5D|%60|\[|\]|\|+|\.\.|\%0a|;\/?|\{|\}|\*)/g
    false
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
        }
    }
}
