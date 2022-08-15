use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::generator::tui::ErrorKind;

macro_rules! pt {
    ($text:literal) => {
        crate::parser::Item::PlainText($text)
    };
}

macro_rules! elem {
        (@tags, $($s:literal),+) => {
            vec![$(crate::parser::LSpan::new_extra($s, 1)),+]
        };
        ($($tags:tt),* ; $($items:expr),* $(,)?) => {
            crate::parser::Item::Element(elem!(@tags, $($tags),*), vec![$($items),*])
        };
    }

macro_rules! test_ok {
        ($item:expr => $($result:expr),* $(,)?) => {
            assert_eq!(
                crate::generator::TuiTextGenerator::<()>::default().item($item, None),
                Ok(vec![$($result),*]),
            )
        };
        ($custom:expr ; $item:expr => $($result:expr),* $(,)?) => {
            assert_eq!(
                crate::generator::TuiTextGenerator::new($custom).item($item, None),
                Ok(vec![$($result)*]),
            )
        };
    }

macro_rules! test_fail {
    ($elem:expr => $span:literal, $kind:expr) => {
        let err = crate::generator::TuiTextGenerator::<()>::default()
            .item($elem, None)
            .unwrap_err();
        assert_eq!(*err.span.fragment(), $span);
        assert_eq!(err.kind(), $kind);
    };
}

#[test]
fn test_normal_element() {
    test_ok!(elem!("green" ; pt!("xxx")) => Span::styled("xxx", Style::default().fg(Color::Green)));
    test_ok!(elem!("fg:red" ; pt!("xxx")) => Span::styled("xxx", Style::default().fg(Color::Red)));
    test_ok!(elem!("bg:yellow" ; pt!("xxx")) => Span::styled("xxx", Style::default().bg(Color::Yellow)));
    test_ok!(elem!("b" ; pt!("xxx")) => Span::styled("xxx", Style::default().add_modifier(Modifier::BOLD)));
    test_ok!(elem!("mod:i" ; pt!("xxx")) => Span::styled("xxx", Style::default().add_modifier(Modifier::ITALIC)));
}

#[test]
fn test_nested_element() {
    test_ok!(
        elem!("bg:blue" ; pt!("one "), elem!("green" ; pt!("two"))) =>
        Span::styled("one ", Style::default().bg(Color::Blue)),
        Span::styled("two", Style::default().bg(Color::Blue).fg(Color::Green)),
    );
}

#[test]
fn test_multi_tag_element() {
    test_ok!(
        elem!("bg:blue", "green", "b" ; pt!("one")) =>
        Span::styled("one", Style::default().bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD)),
    );
}

#[test]
fn test_custom_tag_element() {
    let s = Style::default()
        .bg(Color::Blue)
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    test_ok!(
        |tag: &str| match tag {
            "keyboard" => Some(s),
            _ => None,
        } ; elem!("keyboard" ; pt!("W")) =>
        Span::styled("W", s),
    );
}

#[test]
fn test_invalid_element() {
    test_fail!(elem!("qwerty" ; pt!("one")) => "qwerty", ErrorKind::InvalidTag);
}
