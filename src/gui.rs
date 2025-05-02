use std::str::FromStr;

use iced::{
    widget::{button, column, row, text, text_input, Column},
    Size, Theme, Length, alignment, Element
};
use log::debug;
use crate::word::{VerbForms, Word};

use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};

pub const DEFAULT_THEME: Theme = Theme::Dark;
pub const DEFAULT_WINDOW_SIZE: Size = Size::new(400.0, 400.0);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Result {
    data: String, 
    error_count: u32,
}


#[derive(Debug, Clone)]
pub enum MenuItem  {
    Finder(Option<String>),
    IrrQuiz(Option<Result>),
    TextQuiz(Option<Result>),
    Dictionary(Option<String>)
}


#[derive(Debug, Clone)]
pub struct AppState {
    pub content: String,
    pub menu: MenuItem,
    pub search_string: String,
    pub past_simple_text_input: String,
    pub past_participle_text_input: String,
    pub words: Option<Vec<VerbForms>>
}


impl Default for  AppState {
    fn default() -> Self {
        Self {
            content: "".to_string(), 
            menu: MenuItem::Dictionary(None),
            search_string: "".to_string(), 
            past_participle_text_input: "".to_string(), 
            past_simple_text_input: "".to_string(), 
            words: None

        } 
    }
}


#[derive(Debug, Clone)]
pub enum Message {
    Debug(String),
    ContentChanged(String),
    SearchButtonPressed,
    DictionaryButtonPressed,
    MenuButton(MenuItem),
    PastSimple(String),
    PastParticiple(String),
    CheckIrregularVerb
}


impl AppState {
    pub fn view(&self) -> Column<Message> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);

        
        let menu_quiz = menu_bar!(
            (debug_button_s("Тренировки"), menu_tpl_1(menu_items!(
                (labeled_button("Текст", Message::MenuButton(MenuItem::TextQuiz(None))))
                (labeled_button("Неправильные глаголы", Message::MenuButton(MenuItem::IrrQuiz(None))))
            )).width(240.0))
        );

        let menu_dictionary = menu_bar!(
            (debug_button_s("Словарь"), menu_tpl_1(menu_items!(
                (labeled_button("Формы глаголов", Message::MenuButton(MenuItem::Finder(None))))
                (labeled_button("Найти слово", Message::MenuButton(MenuItem::Dictionary(None))))
            )).width(240.0))
        );

        let window: Column<'_, Message> = column![
            row![
            menu_quiz,
            menu_dictionary,
            ],
        ]
        .spacing(10)
        .padding(10);


        match &self.menu {
            MenuItem::Finder(_) => {
                let find_view = window.extend( 
                    [
                        text("Введите неправильный глагол для поиска: ").into(),
                        row![
                            text_input("Поле ввода...", &self.content)
                            .on_input(Message::ContentChanged),
                        button("Искать").on_press(Message::SearchButtonPressed),
                        ].spacing(10).into(),
                        text(&self.search_string).into()
                    ]
                );
                find_view
            },
            MenuItem::TextQuiz(_) => {window},
            MenuItem::IrrQuiz(_) => {
                let irregular_quiz_view = window.extend([
                    text("Укажите v2-v3 формы неправильного глагола: ").into(),
                    row![
                        text_input("past simple", &self.past_simple_text_input).on_input(Message::PastSimple),
                        text_input("past participle", &self.past_participle_text_input).on_input(Message::PastParticiple),
                    ].spacing(10).into(),
                    labeled_button("Проверить", Message::CheckIrregularVerb).into()
                ]);
                irregular_quiz_view
            
            },
            MenuItem::Dictionary(_) => {
                let dictionary_view = window.extend(
                    [text("Введите слово для поиска: ").into(),
                    row![
                        text_input("Поле ввода...", &self.content)
                        .on_input(Message::ContentChanged),
                    button("Искать").on_press(Message::DictionaryButtonPressed),
                    ].spacing(10).into(),
                    text(&self.search_string).into()
                ]);
                dictionary_view
            }
        }

    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
            }
            Message::PastSimple(value) => {
                self.past_simple_text_input = value;
            },
            Message::PastParticiple(value) => {
                self.past_participle_text_input = value;
            },
            Message::SearchButtonPressed => {
                let mut search_string = String::new();
                if let Ok(search_result) = Word::search_irregular_verb_from_db(&self.content) {
                    for (_word, irregular_verb) in search_result {
                        debug!("found irregular verb: {}", &irregular_verb.base_form);
                        let table_string = format!(
                            "{}| {}| {}",
                            irregular_verb.base_form,
                            irregular_verb.past_simple,
                            irregular_verb.past_participle
                        );
                        search_string.push_str(&table_string);
                    }
                }
                self.search_string = search_string;
            }
            Message::DictionaryButtonPressed => {
                if let Ok(word) = Word::from_str(&self.content) {
                    self.search_string = word.description.unwrap();
                };
            },
            Message::MenuButton(menu_item) => {
                match menu_item {
                    MenuItem::Finder(_) => {
                        self.menu = MenuItem::Finder(None);
                    },
                    MenuItem::Dictionary(_) => {
                        self.menu = MenuItem::Dictionary(None);
                    }, 
                    MenuItem::IrrQuiz(_) => {
                        self.words = Some(Word::fetch_irregular_verbs().unwrap());
                        self.menu = MenuItem::IrrQuiz(None);
                    }
                    _ => {}
                }

            },
            Message::CheckIrregularVerb => {
                debug!("verb check");
            },

            Message::Debug(var) => { debug!("{}", var);},

        }
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> button::Button<'a, Message> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

fn labeled_button(
    label: &str,
    msg: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    base_button(text(label).align_y(alignment::Vertical::Center), msg).width(Length::Fill)
}


fn debug_button_s(label: &str) -> button::Button<Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into())).width(Length::Shrink)
}
