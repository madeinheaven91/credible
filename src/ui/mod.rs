mod list_section;
mod selected_section;
mod help_popup;

use crate::{
    shared::types::InputMode,
    ui::{help_popup::help_popup, list_section::list_section, selected_section::selected_section},
    App,
};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::Stylize,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

// Main function that renders all the ui
pub fn ui<B: Backend>(f: &mut Frame, state: &mut App) {
    let main_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let list_section_box = Block::bordered();
    let mut selected_section_box = Block::bordered().border_type(BorderType::Plain).white();
    if let InputMode::Selected = state.mode {
        selected_section_box = selected_section_box.yellow();
    }

    // list section
    f.render_widget(list_section_box, main_chunk[0]);
    list_section(f, state, main_chunk[0]);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(main_chunk[1])[1];
    // selected section
    f.render_widget(selected_section_box, main_chunk[1]);
    selected_section(f, state, center);

    if let InputMode::Help = state.mode {
        help_popup(f, state, f.area());
    }

}

// Component convention:
// components are functions that take frame, state and area (Rect) as inputs
//
// they return nothing and in the end do the f.render_widget thing
//
// whole inputted area should be used to render the component
// (e.g., don't split the area into 3 parts and render component in the center one)