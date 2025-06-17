use iced::Element;

mod main;

#[derive(Debug)]
pub enum Screen {
    Main(main::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    Main(main::Message),
}

impl Screen {
    pub fn new() -> Self {
        Self::Main(main::State::new())
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            Screen::Main(state) => state.view().map(Message::Main),
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen::new()
    }
}
