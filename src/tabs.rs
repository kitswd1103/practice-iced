pub mod tab_label;

use iced::{
    widget::{button, scrollable, scrollable::Properties, Row},
    Command, Length,
};

use self::tab_label::TabLabel;

#[derive(Debug, Clone)]
pub enum ScrollableTabBarMessage {
    NewTab,
    TabClosed(usize),
    TabSelected(usize),
}

#[derive(Default)]
pub struct TabBar {
    tabs: Vec<TabLabel>,
    active_tab_id: Option<usize>,
    next_id: usize,
}

impl TabBar {
    const SCROLLER_WIDTH: f32 = 3.0;
    fn add_default_tab(&mut self) {
        self.tabs.push(TabLabel::new(self.next_id));
        self.next_id += 1;
    }
    fn remove_tab_by_id(&mut self, remove_id: usize, update_active_id: bool) -> Option<usize> {
        let removed_index = match self.get_index_by_id(remove_id) {
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
                    if let Some(removed_index) = removed_index {
                        self.set_active_id_from_index(removed_index);
                    }
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
            self.active_tab_id = Some(self.tabs[next_index].id());
            self.tabs[next_index].active(true);
        }
        println!("active_tab_id: {:?}", self.active_tab_id);
    }

    fn get_index_by_id(&self, id: usize) -> Option<usize> {
        self.tabs.iter().position(|tab| tab.id() == id)
    }
}

impl<'a> TabBar {
    pub fn update(&mut self, message: ScrollableTabBarMessage) -> Command<ScrollableTabBarMessage> {
        match message {
            ScrollableTabBarMessage::NewTab => {
                self.add_default_tab();
            }
            ScrollableTabBarMessage::TabClosed(tab_id) => {
                self.remove_tab_by_id(tab_id, true);
            }
            ScrollableTabBarMessage::TabSelected(id) => {
                let prev_active = self.active_tab_id;
                self.active_tab_id = Some(id);
                println!("selected id: {}", id);

                if let Some(prev_id) = prev_active {
                    if let Some(index) = self.get_index_by_id(prev_id) {
                        self.tabs[index].active(false);
                    }
                }
                if let Some(index) = self.get_index_by_id(id) {
                    self.tabs[index].active(true);
                }
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Row<'a, ScrollableTabBarMessage> {
        Row::new()
            .push(button("+").on_press(ScrollableTabBarMessage::NewTab))
            .push(
                scrollable(
                    self.tabs
                        .iter()
                        .fold(Row::new(), |row, tab| row.push(tab.view()))
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

#[cfg(test)]
mod tests {
    use crate::tabs::TabBar;

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
            tab_bar.tabs.iter().map(|tab| tab.id()).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(2));

        tab_bar.remove_tab_by_id(1, true);
        assert_eq!(
            vec![0, 2, 4],
            tab_bar.tabs.iter().map(|tab| tab.id()).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(2));

        tab_bar.remove_tab_by_id(2, true);
        assert_eq!(
            vec![0, 4],
            tab_bar.tabs.iter().map(|tab| tab.id()).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(4));

        tab_bar.remove_tab_by_id(4, true);
        assert_eq!(
            vec![0],
            tab_bar.tabs.iter().map(|tab| tab.id()).collect::<Vec<_>>()
        );
        assert_eq!(tab_bar.active_tab_id, Some(0));

        tab_bar.remove_tab_by_id(0, true);
        assert!(tab_bar.tabs.is_empty());
        assert_eq!(tab_bar.active_tab_id, None);
    }
}
