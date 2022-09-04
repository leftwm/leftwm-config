use ::tui::layout::{Constraint, Direction, Layout, Rect};
use anyhow::{Context, Result};

mod x11_keys;
pub(crate) mod xkeysym_lookup;

pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Min(3),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

//used to transform an option into a result to be able to easily
// propagate the fact that is was empty instead of panicking
pub trait AnyhowUnwrap<T> {
    fn unwrap_anyhow(self) -> Result<T>;
}

impl<T> AnyhowUnwrap<T> for Option<T> {
    fn unwrap_anyhow(self) -> Result<T> {
        self.context("called `Option::unwrap()` on a `None` value")
    }
}
