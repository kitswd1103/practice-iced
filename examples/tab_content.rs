use iced::{
    executor,
    widget::{Button, Column, Row},
    Application, Command, Element, Settings, Theme,
};
use practice_iced::tabs::*;

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

#[derive(Debug, Copy, Clone)]
enum MyMessage {
    Tab(TabMessage<ExampleMessage>),
}

#[derive(Default)]
struct PracticeApp {
    tabs: Tabs<ExampleMessage>,
}

impl Application for PracticeApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = MyMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (PracticeApp, Command<Self::Message>) {
        let mut tabs = Tabs::default();
        tabs.add("Example1".to_owned(), Example::Content1(Example1));
        tabs.add("Example2".to_owned(), Example::Content2(Example2));
        (PracticeApp { tabs }, Command::none())
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

#[derive(Default, Copy, Clone, Debug)]
enum ExampleMessage {
    #[default]
    None,
    Test1(ExampleMessage1),
    Test2(ExampleMessage2),
}
#[derive(Clone, Copy, Debug)]
enum ExampleMessage1 {
    Example1_1,
    Example1_2,
}
#[derive(Clone, Copy, Debug)]
enum ExampleMessage2 {
    Example2_1,
    Example2_2,
}
struct Example1;
struct Example2;

enum Example {
    Content1(Example1),
    Content2(Example2),
}
impl TabContent<ExampleMessage> for Example {
    fn update(&mut self, message: ExampleMessage) {
        match message {
            ExampleMessage::None => {}
            ExampleMessage::Test1(message) => match message {
                ExampleMessage1::Example1_1 => println!("example1_1"),
                ExampleMessage1::Example1_2 => println!("example1_2"),
            },
            ExampleMessage::Test2(message) => match message {
                ExampleMessage2::Example2_1 => println!("example2_1"),
                ExampleMessage2::Example2_2 => println!("example2_2"),
            },
        }
    }

    fn view(&self) -> Element<ExampleMessage> {
        match self {
            Self::Content1(_) => Into::<Element<_>>::into(
                Row::new()
                    .push(Button::new("Example1_1").on_press(ExampleMessage1::Example1_1))
                    .push(Button::new("Example1_2").on_press(ExampleMessage1::Example1_2)),
            )
            .map(ExampleMessage::Test1),
            Self::Content2(_) => Into::<Element<_>>::into(
                Column::new()
                    .push(Button::new("Example2_1").on_press(ExampleMessage2::Example2_1))
                    .push(Button::new("Example2_2").on_press(ExampleMessage2::Example2_2)),
            )
            .map(ExampleMessage::Test2),
        }
    }
}
