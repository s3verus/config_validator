use crate::dao::insert_data;
use crate::dao::Message;

pub fn sanity_check(message_obj: Message, channel: String) {
    println!("{:?}", message_obj);
    insert_data(message_obj, channel).unwrap();
}
