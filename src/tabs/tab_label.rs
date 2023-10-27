use iced::{
    alignment, theme,
    widget::{button, Button, Row, Text},
    Background, Color, Element, Length, Renderer, Theme,
};

use super::ScrollableTabBarMessage;

pub struct TabLabel {
    id: usize,
    label: String,
    active: bool,
}

impl TabLabel {
    pub fn new(id: usize, label: String) -> Self {
        Self {
            id,
            label,
            active: false,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Default for TabLabel {
    fn default() -> Self {
        Self {
            id: 0,
            label: "New tab".to_owned(),
            active: false,
        }
    }
}

impl<'a> TabLabel {
    pub fn view(&self) -> Element<'a, ScrollableTabBarMessage, Renderer> {
        Button::new(
            Row::new()
                .push(
                    Text::new(self.label.to_owned())
                        .size(16.0)
                        .vertical_alignment(alignment::Vertical::Center)
                        .width(Length::Shrink),
                )
                .push(
                    Button::new(
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
        .style(theme::Button::custom(TabLabelStyle::new(self.active)))
        .on_press(ScrollableTabBarMessage::TabSelected(self.id))
        .into()
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
