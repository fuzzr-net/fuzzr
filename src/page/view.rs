use iced::{text_input, Column, Command, Container, Element, Length, /* Row, Text, */ TextInput,};
use log::error;

use crate::data::content::ContentItem;
use crate::message::Message;
use crate::ui::content_view;
use crate::ui::style::Theme;

#[derive(Debug, Clone)]
pub struct ViewPage {
    input_state: text_input::State,
    pub input_value: String,
    content: Option<ContentItem>,
}

impl Default for ViewPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewPage {
    pub fn new() -> ViewPage {
        ViewPage {
            input_state: text_input::State::new(),
            input_value: String::new(),
            content: None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::ViewPageInputChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            Message::ViewPageContentLoaded(content_item) => match content_item {
                Ok(content_item) => {
                    self.content = Some(content_item);
                    Command::none()
                }
                Err(err) => {
                    error!("Error loading content item from IPFS: {}", err);
                    Command::none()
                }
            },
            _ => Command::none(),
        }
    }

    pub fn view(&mut self, theme: &Theme) -> Element<Message> {
        let input = TextInput::new(
            &mut self.input_state,
            "Paste Content ID (CID) here",
            &self.input_value,
            Message::ViewPageInputChanged,
        )
        .padding(15)
        .size(16)
        .style(*theme)
        .on_submit(Message::ViewPageLoadContent);

        let content_image = match &self.content {
            Some(content) => Column::new().push(content_view::view(&content)),
            None => Column::new(),
        };

        let content_container = Column::new().push(input).push(content_image);

        Container::new(content_container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .style(*theme)
            .into()
    }
}
