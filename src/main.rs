use iced::{widget::{button, column, row, text, text_input, Column},Size};
use language::word::Word;
use log::debug;
use dotenv::dotenv;



#[derive(Default)]
struct AppState {
   content: String,
   result_string: String
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    SearchButtonPressed,
}


impl AppState {

    fn view(&self) -> Column<Message> {
        column![
            text("Введите слово для поиска: "),
            row![
                text_input("Поле ввода...", &self.content)
                .on_input(Message::ContentChanged), 
                button("Искать").on_press(Message::SearchButtonPressed),
            ],
            text(&self.result_string)
        ]
    }
    
    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.content = content;
            }
            Message::SearchButtonPressed => {
                let mut result_string = String::new();
                if let Ok(search_result)= Word::get_irregular_verb_from_db(&self.content) {
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

fn main() {
    dotenv().ok();
    env_logger::init();
    debug!("app init");
    let _ = iced::application("test", AppState::update, AppState::view)
        .window_size(Size::new(300.0, 400.0))
        .run();
    debug!("app close");
}

