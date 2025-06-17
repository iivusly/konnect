use iced::{widget::{column, scrollable, text}, Element};

#[derive(Debug, Default)]
pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Event {}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        scrollable(column![
            "scrolling"
        ]).into()
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
        }
    }
}
