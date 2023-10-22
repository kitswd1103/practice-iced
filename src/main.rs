use iced::{
    alignment, executor,
    widget::{button, scrollable, scrollable::Properties, Row, Text},
    Application, Command, Element, Length, Settings, Theme,
};

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
    //PracticeTabBar::run(Settings::default())
}

#[derive(Default)]
struct PracticeApp {
    tab_labels: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ScrollableTabBarMessage {
    NewTab,
    TabClosed(usize),
    TabSelected(usize),
    TabLabelChanged(String)
}

impl Application for PracticeApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ScrollableTabBarMessage;
    type Theme = Theme;

    fn new(_flags: ()) -> (PracticeApp, Command<Self::Message>) {
        (
            PracticeApp {
                tab_labels: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("practice_app")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            ScrollableTabBarMessage::NewTab => self.tab_labels.push("New Tab".to_string()),
            ScrollableTabBarMessage::TabClosed(index) => { self.tab_labels.remove(index); }, 
            ScrollableTabBarMessage::TabSelected(index) => println!("selected index: {}", index),
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Row::new().push(button("+").on_press(ScrollableTabBarMessage::NewTab)).push(
            scrollable(
                self.tab_labels
                    .iter()
                    .enumerate()
                    .fold(Row::new(), |row, (index, label)| {
                        row.push(button(
                            Row::new()
                                .push(
                                    Text::new(label.as_str())
                                        .size(16.0)
                                        .vertical_alignment(alignment::Vertical::Center)
                                        .width(Length::Shrink),
                                )
                                .push(
                                    button(
                                        Text::new("X")
                                            .size(8.0)
                                            .vertical_alignment(alignment::Vertical::Center)
                                            .horizontal_alignment(alignment::Horizontal::Center),
                                    )
                                    .width(Length::Shrink)
                                    .height(Length::Shrink)
                                    .on_press(ScrollableTabBarMessage::TabClosed(index)),
                                )
                                .spacing(10),
                        ).on_press(ScrollableTabBarMessage::TabSelected(index))
                    )
                    })
                    .width(Length::Shrink)
                    .padding([0.0, 0.0, 2.0, 0.0]),
            )
            .direction(scrollable::Direction::Horizontal(
                Properties::new().width(2).scroller_width(2),
            ))
        )
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
