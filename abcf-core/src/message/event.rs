pub struct EventAttribute {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub index: bool,
}

pub struct Event {
    pub t: String,
    pub attr: Vec<EventAttribute>,
}
