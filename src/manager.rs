use crate::dao::insert_data;
use crate::dao::Message;

pub fn sanity_check(message: Message, channel: String) {
    if !message.data.tcp.is_none() {
        let ip_address = match message.data.tcp.as_ref().map(|m| &m.ip_address) {
            Some(x) => x,
            _ => "",
        };
        println!("{:?}", ip_address);
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

        insert_data(message, channel).unwrap();
    }
}
