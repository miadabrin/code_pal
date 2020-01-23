use std::io;

use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::{Frame, Terminal};

use crate::app::{ui_component::UIComponent, App};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());
        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title(app.title))
            .titles(&app.tabs.titles)
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.index)
            .render(&mut f, chunks[0]);
        match app.tabs.index {
            0 => draw_first_tab(&mut f, app, chunks[1]),
            1 => draw_second_tab(&mut f, app, chunks[1]),
            _ => {}
        };
    })
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Min(7)].as_ref())
        .split(area);
    app.todo_items.draw(f, chunks[0]);
    app.notes.draw(f, chunks[1]);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Min(7)].as_ref())
        .split(area);
    app.projects.draw(f, chunks[0]);
}
