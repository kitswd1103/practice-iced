use iced::{
    alignment, executor,
    widget::{button, scrollable, scrollable::Properties, Row, Text, Button},
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
        Self::tab_bar(&self.tab_labels, true)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl<'a> PracticeApp {
    fn tab_bar(labels: &'a Vec<String>, view_add_button: bool) -> Row<'a, ScrollableTabBarMessage> {
        let mut tab_bar = Row::new();
        if view_add_button {
            tab_bar = tab_bar.push(button("+").on_press(ScrollableTabBarMessage::NewTab));
        }
        tab_bar.push(
            scrollable(
                labels
                    .iter()
                    .enumerate()
                    .fold(Row::new(), |row, (index, label)| {
                        row.push(Self::tab(label, index))
                    })
                    .width(Length::Shrink)
                    .padding([0.0, 0.0, 2.0, 0.0]),
            )
            .direction(scrollable::Direction::Horizontal(
                Properties::new().width(2).scroller_width(2),
            ))
        )
    } 
    fn tab(label: & 'a String, index: usize) -> Button<'a, ScrollableTabBarMessage> {
        Button::new(
            Row::new()
                .push(
                    Text::new(label)
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
    }
}
