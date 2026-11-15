//! Reading direction of the interface.
//!
//! egui has no global right-to-left mode, so layout code asks this module for
//! the *logical* start and end of a row instead of hard-coding left and right.
//! Use it where the layout follows the reading direction (panel sides, which
//! edge a back button sits on, icon mirroring). Keep physical `left_to_right`
//! and `Align::LEFT` for things that never mirror, such as a progress bar or a
//! media timeline.
//!
//! Message and editor *text* is laid out by [`crate::bidi`] from its own
//! content, not from this setting.

use egui::{Align, Layout};

use crate::i18n::Locale;

/// The reading direction of the interface language.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// Left to right: the start of a row is its left edge.
    #[default]
    Ltr,
    /// Right to left: the start of a row is its right edge.
    Rtl,
}

impl From<Locale> for Direction {
    fn from(locale: Locale) -> Self {
        if locale.is_rtl() {
            Self::Rtl
        } else {
            Self::Ltr
        }
    }
}

impl Direction {
    pub fn is_rtl(self) -> bool {
        self == Self::Rtl
    }

    /// Horizontal alignment of the start edge.
    pub fn start_align(self) -> Align {
        match self {
            Self::Ltr => Align::Min,
            Self::Rtl => Align::Max,
        }
    }

    /// Horizontal alignment of the end edge.
    pub fn end_align(self) -> Align {
        match self {
            Self::Ltr => Align::Max,
            Self::Rtl => Align::Min,
        }
    }

    /// A row that fills from the start edge toward the end edge.
    pub fn start_to_end(self, cross: Align) -> Layout {
        match self {
            Self::Ltr => Layout::left_to_right(cross),
            Self::Rtl => Layout::right_to_left(cross),
        }
    }

    /// A row that fills from the end edge toward the start edge.
    pub fn end_to_start(self, cross: Align) -> Layout {
        match self {
            Self::Ltr => Layout::right_to_left(cross),
            Self::Rtl => Layout::left_to_right(cross),
        }
    }

    /// A side panel docked at the start edge: left for left-to-right, right
    /// for right-to-left.
    pub fn start_panel(self, id: impl Into<egui::Id>) -> egui::Panel {
        match self {
            Self::Ltr => egui::Panel::left(id),
            Self::Rtl => egui::Panel::right(id),
        }
    }

    /// Horizontal scale for an icon that points along the reading direction
    /// (reply, forward, send, chevrons): `-1.0` mirrors it, `1.0` keeps it.
    pub fn icon_flip(self) -> f32 {
        match self {
            Self::Ltr => 1.0,
            Self::Rtl => -1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_right_to_left_languages_mirror() {
        assert_eq!(Direction::from(Locale::English), Direction::Ltr);
        assert_eq!(Direction::from(Locale::German), Direction::Ltr);
        assert_eq!(Direction::from(Locale::Hebrew), Direction::Rtl);
        assert_eq!(Direction::default(), Direction::Ltr);
    }

    #[test]
    fn start_and_end_swap_with_the_direction() {
        let ltr = Direction::Ltr;
        let rtl = Direction::Rtl;
        assert_eq!(ltr.start_align(), Align::Min);
        assert_eq!(rtl.start_align(), Align::Max);
        assert_eq!(ltr.end_align(), rtl.start_align());
        assert_eq!(
            ltr.start_to_end(Align::Center),
            Layout::left_to_right(Align::Center)
        );
        assert_eq!(
            rtl.start_to_end(Align::Center),
            Layout::right_to_left(Align::Center)
        );
        assert_eq!(ltr.end_to_start(Align::Min), rtl.start_to_end(Align::Min));
        assert_eq!(ltr.icon_flip(), 1.0);
        assert_eq!(rtl.icon_flip(), -1.0);
    }
}
