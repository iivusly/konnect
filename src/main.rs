use iced::{widget::Container, window, Element, Event, Length, Task};
use screens::Screen;

mod backends;
mod screens;

#[derive(Debug)]
enum Message {
    Event(iced::Event),
    Screen(screens::Message),
}

#[derive(Debug, Default)]
struct Konnect {
    screen: Screen,
}

impl Konnect {
    fn new(_: ()) -> (Konnect, Task<Message>) {
        let konnect = Konnect {
            screen: Screen::new()
        };

        (konnect, Task::batch(vec![Task::none()]))
    }

    fn title(&self) -> String {
        String::from("Konnect")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Event(event) => self.handle_event(event),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = self.screen.view().map(Message::Screen);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }

    fn handle_event(&mut self, event: Event) -> Task<Message> {
        Task::none()
    }
}

fn main() {
    iced::application(Konnect::title, Konnect::update, Konnect::view)
        .run_with(move || Konnect::new(()));
}
