use iced::{executor, Application, Command, Element, Settings, Theme, event, Event, mouse};
use tabs::*;

pub mod tabs;

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyMessage {
    Tab(TabMessage<ContentMessage>),
    EventMessage(Event),
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
            MyMessage::EventMessage(Event::Mouse(event))=> {
                match event {
                    mouse::Event::WheelScrolled { delta } => {
                        let delta = match delta {
                            mouse::ScrollDelta::Lines { x, y } => { (x, y) },
                            mouse::ScrollDelta::Pixels { x, y } => { (x, y) }
                        };
                        println!("{:?}", delta);
                    }
                    _ => {}
                }
            },
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.tabs.view().map(MyMessage::Tab)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(MyMessage::EventMessage)
    }
}
