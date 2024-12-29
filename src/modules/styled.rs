use core::fmt;
use std::fmt::Formatter;

use crate::colors;
use crate::colors::Style;
use crate::icons::Icon;
use crate::modules::Show;

#[derive(Clone, Copy)]
pub struct StyledModule<T> {
    content: T,
    icon: Option<Icon>,
    style: Style,
}

impl<T> StyledModule<T> {
    pub fn new(content: T, icon: Option<Icon>, style: colors::Style) -> Self {
        Self {
            content,
            icon,
            style,
        }
    }
}

impl<T: Show> fmt::Display for StyledModule<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {}{}",
            self.style,
            self.icon.unwrap_or(Icon::Empty),
            self.content.show().unwrap_or_default(),
            Style::default()
        )
    }
}
