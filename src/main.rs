use iced::{
    alignment, executor,
    widget::{button, scrollable, scrollable::Properties, Row, Text, Button},
    Application, Command, Element, Length, Settings, Theme,
};

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

#[derive(Default)]
struct PracticeApp {
    tab_bar: TabBar
}

#[derive(Default)]
struct TabBar {
    tabs: Vec<Tab>,
    active_tab_id: Option<usize>,
    next_id: usize,
}

struct Tab {
    id: usize,
    label: String,
}

impl TabBar {
    fn add_default_tab(&mut self) {
        self.tabs.push(Tab { id: self.next_id
            , ..Default::default()
        });
        self.next_id = self.next_id + 1;
    }
    fn remove_tab_by_id(&mut self, remove_id: usize, update_active_id: bool) -> Option<usize> {
        let removed_index = match self.tabs.iter().position(|tab| tab.id == remove_id) {
            Some(index) => {
                self.tabs.remove(index);
                Some(index)
            },
            None => None
        };

        if !update_active_id {
            return removed_index;
        }
        
        if removed_index.is_some() {
            match self.active_tab_id {
                None => {},
                Some(active_id) if active_id != remove_id => {},
                _ => {
                    let removed_index = removed_index.unwrap();
                    self.set_active_id_from_index(removed_index);
                }
            }
        }
        removed_index
    }
    fn set_active_id_from_index(&mut self, index: usize) {
        if self.tabs.is_empty() {
            self.active_tab_id = None;
        } else {
            let next_index =  index.clamp(0, self.tabs.len() - 1);
            self.active_tab_id = Some(self.tabs[next_index].id);
        }
        println!("active_tab_id: {:?}", self.active_tab_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::TabBar;

    #[test]
    fn test_remove_tab() {
        let mut tab_bar = TabBar::default();
        tab_bar.add_default_tab();
        tab_bar.add_default_tab();
        tab_bar.add_default_tab();
        tab_bar.add_default_tab();
        tab_bar.add_default_tab();
        tab_bar.active_tab_id = Some(2);

        tab_bar.remove_tab_by_id(3, true);
        assert_eq!(vec![0,1,2,4], tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>());
        assert_eq!(tab_bar.active_tab_id, Some(2));

        tab_bar.remove_tab_by_id(1, true);
        assert_eq!(vec![0,2,4], tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>());
        assert_eq!(tab_bar.active_tab_id, Some(2));
        
        tab_bar.remove_tab_by_id(2, true);
        assert_eq!(vec![0,4], tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>());
        assert_eq!(tab_bar.active_tab_id, Some(4));
        
        tab_bar.remove_tab_by_id(4, true);
        assert_eq!(vec![0], tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>());
        assert_eq!(tab_bar.active_tab_id, Some(0));
        
        tab_bar.remove_tab_by_id(0, true);
        assert!(tab_bar.tabs.is_empty());
        assert_eq!(tab_bar.active_tab_id, None);
    }
}

impl Default for Tab {
    fn default() -> Self {
        Self {id: 0, label: "New tab".to_owned()}
    }
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
            PracticeApp { ..Default::default() },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("practice_app")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            ScrollableTabBarMessage::NewTab => {
                self.tab_bar.add_default_tab();
            },
            ScrollableTabBarMessage::TabClosed(tab_id) => {
                self.tab_bar.remove_tab_by_id(tab_id, true);
            },
            ScrollableTabBarMessage::TabSelected(index) => {
                self.tab_bar.active_tab_id = Some(index);
                println!("selected id: {}", index);
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.tab_bar.view()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl<'a> TabBar {
    fn view(&self) -> Row<'a, ScrollableTabBarMessage> {
        Row::new().push(button("+").on_press(ScrollableTabBarMessage::NewTab))
            .push(
                scrollable(
                    self.tabs
                        .iter()
                        .fold(Row::new(), |row, tab| {
                            row.push(tab.view())
                        })
                        .width(Length::Shrink)
                        .padding([0.0, 0.0, 2.0, 0.0]),
                )
                .direction(scrollable::Direction::Horizontal(
                    Properties::new().width(2).scroller_width(2),
                ))
        )
    }
}

impl<'a> Tab {
    fn view(&self) -> Button<'a, ScrollableTabBarMessage> {
        Button::new(
            Row::new()
                .push(
                    Text::new(self.label.to_owned())
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
                    .on_press(ScrollableTabBarMessage::TabClosed(self.id)),
                )
                .spacing(10),
        ).on_press(ScrollableTabBarMessage::TabSelected(self.id))
    }
}
