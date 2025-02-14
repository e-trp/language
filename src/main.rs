use language::gui::{AppState, DEFAULT_THEME, DEFAULT_WINDOW_SIZE};
use log::debug;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();

    debug!("app init");

    // Configure and run the application
    iced::application("App", AppState::update, AppState::view)
        .window_size(DEFAULT_WINDOW_SIZE)
        .theme(|_| DEFAULT_THEME)
        .run()
        .expect("Failed to run application");

    debug!("app close");
}