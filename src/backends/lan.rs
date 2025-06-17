const PROTOCOL_PORT_DEFAULT: i64 = 1716;
const PROTOCOL_PORT_MIN: i64 = 1716;
const PROTOCOL_PORT_MAX: i64 = 1764;

pub struct State {

}

pub enum Message {
    Event(Event),
}

pub enum Event {
    
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self) {

    }

    fn create_certificates() {

    }
}
