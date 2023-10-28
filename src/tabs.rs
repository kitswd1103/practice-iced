use iced::{widget::Column, Element};

use crate::tabs::tab_bar::TabBar;

mod tab_bar;
mod tab_label;

type ContentBuilder<Message> = Box<dyn Fn() -> (String, Box<dyn TabContent<Message>>)>;

#[derive(Debug, Copy, Clone)]
pub enum TabMessage<ContentMessage: Clone + Copy> {
    TabBarMessage(ScrollableTabBarMessage),
    ContentMessage(ContentMessage),
}

#[derive(Debug, Copy, Clone)]
pub enum ScrollableTabBarMessage {
    NewTab,
    TabClosed(usize),
    TabSelected(usize),
}

pub struct Tabs<Message: Clone + Copy> {
    tab_bar: TabBar,
    tab_contents: Vec<(usize, Box<dyn TabContent<Message>>)>,
    add_content_clojure: Option<ContentBuilder<Message>>,
}

impl<Message: Clone + Copy> Default for Tabs<Message> {
    fn default() -> Self {
        Self {
            tab_bar: TabBar::default(),
            tab_contents: Vec::default(),
            add_content_clojure: None,
        }
    }
}

impl<Message: Clone + Copy> Tabs<Message> {
    pub fn update(&mut self, message: TabMessage<Message>) {
        match message {
            TabMessage::TabBarMessage(message) => {
                match message {
                    ScrollableTabBarMessage::NewTab => {
                        if let Some(clojure) = &self.add_content_clojure {
                            let new_content = clojure();
                            self.add_box(new_content.0, new_content.1);
                        } else {
                            self.tab_bar.update(message);
                        }
                    }
                    _ => self.tab_bar.update(message),
                }

            }
            TabMessage::ContentMessage(message) => {
                if let Some(content) = self.get_active_mut_content() {
                    content.update(message);
                }
            }
        }

        if let TabMessage::TabBarMessage(ScrollableTabBarMessage::TabClosed(id)) = message {
            self.remove_by_id(id)
        }
    }

    pub fn view(&self) -> Element<TabMessage<Message>> {
        let mut ret = Column::new().push(self.tab_bar.view().map(TabMessage::TabBarMessage));

        ret = if let Some(content) = self.get_active_content() {
            ret.push(content.view().map(TabMessage::ContentMessage))
        } else {
            ret
        };

        ret.into()
    }
    pub fn register_add_clojure(&mut self, clojure: ContentBuilder<Message>) {
        self.add_content_clojure = Some(clojure);
    }

    pub fn add(&mut self, label_name: String, content: impl TabContent<Message> + 'static) {
        self.tab_contents
            .push((self.tab_bar.add(label_name), Box::new(content)));
    }
    pub fn add_box(&mut self, label_name: String, content: Box<dyn TabContent<Message> + 'static>) {
        self.tab_contents
            .push((self.tab_bar.add(label_name), content));
    }
    pub fn get_active_content(&self) -> Option<&dyn TabContent<Message>> {
        if let Some(active_id) = self.get_active_id() {
            match self
                .tab_contents
                .iter()
                .find(|&content| content.0 == active_id)
            {
                Some(content) => Some(content.1.as_ref()),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn get_active_mut_content(&mut self) -> Option<&mut Box<dyn TabContent<Message>>> {
        if let Some(active_id) = self.get_active_id() {
            match self
                .tab_contents
                .iter_mut()
                .find(|content| content.0 == active_id)
            {
                Some(content) => Some(&mut content.1),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn remove_by_id(&mut self, id: usize) {
        if let Some(index) = self
            .tab_contents
            .iter()
            .position(|contents| contents.0 == id)
        {
            self.tab_contents.remove(index);
        }
    }
    fn get_active_id(&self) -> Option<usize> {
        self.tab_bar.get_active_id()
    }
}

pub trait TabContent<Message> {
    fn update(&mut self, _message: Message) {}
    fn view(&self) -> Element<Message>;
}
