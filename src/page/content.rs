use iced::{text_input, Column, Container, Element, Length, Text};

// use crate::data::content::ContentItem;
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct ContentPage {
    input_state: text_input::State,
    input_value: String,
}

impl ContentPage {
    pub fn new() -> ContentPage {
        ContentPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            _ => {}
        };
    }

    pub fn view(&self) -> Element<Message> {
        let content_container = Column::new().push(Text::new("TODO: Content page").size(18));

        Container::new(content_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .into()
    }
}
