use iced::{widget::Column, Command};

use crate::tabs::tab_bar::TabBar;

mod tab_bar;
mod tab_label;

#[derive(Debug, Clone)]
pub enum ScrollableTabBarMessage {
    NewTab,
    TabClosed(usize),
    TabSelected(usize),
}

#[derive(Default)]
pub struct Tabs {
    tab_bar: TabBar
}

impl<'a> Tabs {
    pub fn update(&mut self, message: ScrollableTabBarMessage) -> Command<ScrollableTabBarMessage> {
        self.tab_bar.update(message)
    }

    pub fn view(&self) -> Column<'a, ScrollableTabBarMessage> {
        Column::new()
            .push(self.tab_bar.view())
    }
}
