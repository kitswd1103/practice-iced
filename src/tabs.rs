use iced::{Application, Theme, Command, widget::{button, Row}, Element, Length};
use iced_aw::{TabBar, TabLabel};

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(usize),
    TabClosed(usize),
    NewTab,
}

#[derive(Default)]
pub struct PracticeTabBar {
    _active_tab: usize,
    tabs: Vec<String>
}

impl Application for PracticeTabBar {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn new(_flags: ()) -> (PracticeTabBar, Command<Message>) {
        (PracticeTabBar::default(), Command::none())
    }
    fn title(&self) -> String {
        String::from("PracticeTabBar")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NewTab => {
                let tab_label = self.tabs.len().to_string();
                self.tabs.push(tab_label);
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Row::new()
            .push(
                button("add").on_press(Message::NewTab)
            ).push(
            Row::new()
                .push(
                    self.tabs.iter().fold(
                        TabBar::new(Message::TabSelected),
                        |tab_bar, tab_label| {
                            let size = tab_bar.size();
                            tab_bar.push(size, TabLabel::Text(tab_label.to_owned()))
                        }
                    )
                    .on_close(Message::TabClosed)
                    .tab_width(Length::Shrink)
                    .spacing(5.0)
                    .padding(5.0)
                    .text_size(10.0),
                )
            )
            .height(32)
            .into()
    }
}
