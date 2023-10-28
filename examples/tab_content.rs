use std::cell::Cell;

use iced::{
    executor,
    widget::{Button, Column, Row, text},
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
        let num = Cell::new(0);

        tabs.register_add_clojure(Box::new(move || {
            let label = format!("Example {}", num.get());
            let content: Box<dyn TabContent<ExampleMessage>> = if num.get() % 2 == 0 {
                Box::new(Example::Content1(Example1 { num: num.get() }))
            } else {
                Box::new(Example::Content2(Example2 { num: num.get() }))
            };

            num.set(num.get() + 1);
            (label, content)
        }));
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
struct Example1 {
    num: usize,
}
struct Example2 {
    num: usize,
}

enum Example {
    Content1(Example1),
    Content2(Example2),
}
impl TabContent<ExampleMessage> for Example {
    fn update(&mut self, message: ExampleMessage) {
        let num = match self {
            Example::Content1(c) => c.num,
            Example::Content2(c) => c.num,
        };
        match message {
            ExampleMessage::None => {}
            ExampleMessage::Test1(message) => match message {
                ExampleMessage1::Example1_1 => println!("example1_1_{}", num),
                ExampleMessage1::Example1_2 => println!("example1_2_{}", num),
            },
            ExampleMessage::Test2(message) => match message {
                ExampleMessage2::Example2_1 => println!("example2_1_{}", num),
                ExampleMessage2::Example2_2 => println!("example2_2_{}", num),
            },
        }
    }

    fn view(&self) -> Element<ExampleMessage> {
        match self {
            Self::Content1(content) => Into::<Element<_>>::into(
                Row::new()
                    .push(Button::new(text(format!("Example1_1_{}", content.num))).on_press(ExampleMessage1::Example1_1))
                    .push(Button::new(text(format!("Example1_2_{}", content.num))).on_press(ExampleMessage1::Example1_2)),
            )
            .map(ExampleMessage::Test1),
            Self::Content2(content) => Into::<Element<_>>::into(
                Column::new()
                    .push(Button::new(text(format!("Example1_2_{}", content.num))).on_press(ExampleMessage2::Example2_1))
                    .push(Button::new(text(format!("Example1_2_{}", content.num))).on_press(ExampleMessage2::Example2_2)),
            )
            .map(ExampleMessage::Test2),
        }
    }
}
