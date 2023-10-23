use iced::{
    alignment, executor, theme,
    widget::{button, scrollable, scrollable::Properties, Button, Row, Text},
    Application, Background, Color, Command, Element, Length, Renderer, Settings, Theme,
};

fn main() -> iced::Result {
    PracticeApp::run(Settings::default())
}

#[derive(Default)]
struct PracticeApp {
    tab_bar: TabBar,
}

#[derive(Default)]
struct TabBar {
    tabs: Vec<TabLabel>,
    active_tab_id: Option<usize>,
    next_id: usize,
}

struct TabLabel {
    id: usize,
    label: String,
}

impl TabBar {
    const SCROLLER_WIDTH: f32 = 3.0;
    fn add_default_tab(&mut self) {
        self.tabs.push(TabLabel {
            id: self.next_id,
            ..Default::default()
        });
        self.next_id = self.next_id + 1;
    }
    fn remove_tab_by_id(&mut self, remove_id: usize, update_active_id: bool) -> Option<usize> {
        let removed_index = match self.tabs.iter().position(|tab| tab.id == remove_id) {
            Some(index) => {
                self.tabs.remove(index);
                Some(index)
            }
            None => None,
        };

        if !update_active_id {
            return removed_index;
        }

        if removed_index.is_some() {
            match self.active_tab_id {
                None => {}
                Some(active_id) if active_id != remove_id => {}
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
            let next_index = index.clamp(0, self.tabs.len() - 1);
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
        assert_eq!(
            vec![0, 1, 2, 4],
            tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(2));

        tab_bar.remove_tab_by_id(1, true);
        assert_eq!(
            vec![0, 2, 4],
            tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(2));

        tab_bar.remove_tab_by_id(2, true);
        assert_eq!(
            vec![0, 4],
            tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(4));

        tab_bar.remove_tab_by_id(4, true);
        assert_eq!(
            vec![0],
            tab_bar.tabs.iter().map(|tab| tab.id).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(0));

        tab_bar.remove_tab_by_id(0, true);
        assert!(tab_bar.tabs.is_empty());
        assert_eq!(tab_bar.active_tab_id, None);
    }
}

struct TabLabelStyle {
    active_tab: bool,
}
impl TabLabelStyle {
    fn new(active_tab: bool) -> Self {
        Self { active_tab }
    }
}
const INACTICE_TONE: f32 = 0.8;

impl button::StyleSheet for TabLabelStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.active(&theme::Button::default());
        match appearance.background {
            Some(Background::Color(color)) => {
                if self.active_tab {
                    appearance
                } else {
                    let color = Color::from([
                        color.r * INACTICE_TONE,
                        color.g * INACTICE_TONE,
                        color.b * INACTICE_TONE,
                    ]);
                    appearance.background = Some(color.into());
                    appearance
                }
            }
            Some(_) => appearance,
            None => appearance,
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.hovered(&theme::Button::default());
        match appearance.background {
            Some(Background::Color(color)) => {
                if self.active_tab {
                    appearance
                } else {
                    let color = Color::from([
                        color.r * INACTICE_TONE,
                        color.g * INACTICE_TONE,
                        color.b * INACTICE_TONE,
                    ]);
                    appearance.background = Some(color.into());
                    appearance
                }
            }
            Some(_) => appearance,
            None => appearance,
        }
    }
}

impl Default for TabLabel {
    fn default() -> Self {
        Self {
            id: 0,
            label: "New tab".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScrollableTabBarMessage {
    NewTab,
    TabClosed(usize),
    TabSelected(usize),
    TabLabelChanged(String),
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
        match message {
            ScrollableTabBarMessage::NewTab => {
                self.tab_bar.add_default_tab();
            }
            ScrollableTabBarMessage::TabClosed(tab_id) => {
                self.tab_bar.remove_tab_by_id(tab_id, true);
            }
            ScrollableTabBarMessage::TabSelected(index) => {
                self.tab_bar.active_tab_id = Some(index);
                println!("selected id: {}", index);
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.tab_bar.view().into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl<'a> TabBar {
    fn view(&self) -> Row<'a, ScrollableTabBarMessage> {
        Row::new()
            .push(button("+").on_press(ScrollableTabBarMessage::NewTab))
            .push(
                scrollable(
                    self.tabs
                        .iter()
                        .fold(Row::new(), |row, tab| {
                            row.push(tab.view(self.active_tab_id))
                        })
                        .width(Length::Shrink)
                        .padding([0.0, 0.0, Self::SCROLLER_WIDTH, 0.0]),
                )
                .direction(scrollable::Direction::Horizontal(
                    Properties::new()
                        .width(Self::SCROLLER_WIDTH)
                        .scroller_width(Self::SCROLLER_WIDTH),
                )),
            )
    }
}

impl<'a> TabLabel {
    fn view(&self, active_tab_id: Option<usize>) -> Button<'a, ScrollableTabBarMessage, Renderer> {
        let active_tab = if let Some(id) = active_tab_id {
            id == self.id
        } else {
            false
        };

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
        )
        .style(theme::Button::custom(TabLabelStyle::new(active_tab)))
        .on_press(ScrollableTabBarMessage::TabSelected(self.id))
    }
}
