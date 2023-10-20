use iced::{Application, executor, Theme, Command, Settings, Element};

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

struct PracticeApp;

impl Application for PracticeApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (PracticeApp, Command<Self::Message>) {
        (PracticeApp, Command::none())
    }

    fn title(&self) -> String {
        String::from("practice_app")
    }

    fn update(&mut self, _message: Self::Message) ->Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Practice app".into()
    }
}