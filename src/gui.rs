use iced::{widget::{button, column, row, text, text_input, Column}, Size, Theme};
use log::debug;
use crate::word::Word;

pub const DEFAULT_THEME: Theme = Theme::Dark;
pub const DEFAULT_WINDOW_SIZE: Size = Size::new(300.0, 400.0);

#[derive(Default)]
pub struct AppState {
    pub content: String,
    pub result_string: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    SearchButtonPressed,
}

impl AppState {
    pub fn view(&self) -> Column<Message> {
        column![
            text("Введите слово для поиска: "),
            row![
                text_input("Поле ввода...", &self.content)
                    .on_input(Message::ContentChanged),
                button("Искать").on_press(Message::SearchButtonPressed),
            ]
            .spacing(10),
            text(&self.result_string)
        ]
        .spacing(10)
        .padding(10)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
            }
            Message::SearchButtonPressed => {
                let mut result_string = String::new();
                if let Ok(search_result) = Word::get_irregular_verb_from_db(&self.content) {
                    for (_word, irregular_verb) in search_result {
                        debug!("found irregular verb run {}", &irregular_verb.base_form);
                        let table_string = format!(
                            "{}| {}| {}",
                            irregular_verb.base_form,
                            irregular_verb.past_simple,
                            irregular_verb.past_participle
                        );
                        result_string.push_str(&table_string);
                    }
                }
                self.result_string = result_string;
            }
        }
    }
}