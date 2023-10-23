use iced::{executor, Application, Command, Element, Settings, Theme};
use tabs::{ScrollableTabBarMessage, TabBar};

mod tabs;

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

#[derive(Default)]
struct PracticeApp {
    tab_bar: TabBar,
}

impl Application for PracticeApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ScrollableTabBarMessage;
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
        self.tab_bar.update(message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.tab_bar.view().into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
