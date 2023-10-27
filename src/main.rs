use iced::{executor, Application, Command, Element, Settings, Theme};
use tabs::*;

pub mod tabs;

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

#[derive(Debug, Copy, Clone)]
enum MyMessage {
    Tab(TabMessage<ContentMessage>),
}

#[derive(Debug, Copy, Clone)]
enum ContentMessage {
    _None,
}

#[derive(Default)]
struct PracticeApp {
    tabs: Tabs<ContentMessage>,
}

impl Application for PracticeApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = MyMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (PracticeApp, Command<Self::Message>) {
        (
            PracticeApp {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("practice_app")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            MyMessage::Tab(message) => self.tabs.update(message),
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.tabs.view().map(MyMessage::Tab)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
