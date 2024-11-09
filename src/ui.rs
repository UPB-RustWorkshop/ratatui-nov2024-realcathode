use ratatui::{
    widgets::{Block, Borders, BorderType, List, Paragraph, ListItem, Padding},
    style::{Color, Style},
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {    
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(outer_layout[1]);


    frame.render_widget(
        Paragraph::new("outer 0")
            .block(Block::new().borders(Borders::ALL)),
        outer_layout[0]);
    frame.render_widget(
        Paragraph::new("inner 0")
            .block(Block::new().borders(Borders::ALL)),
        inner_layout[0]);
    frame.render_widget(
        Paragraph::new("inner 1")
            .block(Block::new().borders(Borders::ALL)),
        inner_layout[1]);
}