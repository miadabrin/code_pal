use std::io;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Tabs, Text, Widget};
use tui::{Frame, Terminal};

use crate::app::{App, CodePalAction};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
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
            0 => draw_first_tab(&mut f, &app, chunks[1]),
            1 => draw_second_tab(&mut f, &app, chunks[1]),
            _ => {}
        };
    })
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Min(7)].as_ref())
        .split(area);
    draw_text(f, app, chunks[0]);
    draw_charts(f, app, chunks[1]);
}

fn draw_text<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let text: Vec<Text> = app
        .current_text
        .iter()
        .map(|x| Text::raw(format!("{}\n", x)))
        .collect();
    let current_action_title = match app.current_action {
        CodePalAction::AddToDoItem => String::from("Add To Do Item"),
        _ => String::from(""),
    };
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(&current_action_title)
                .title_style(Style::default().modifier(Modifier::BOLD)),
        )
        .wrap(true)
        .render(f, area);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let constraints = vec![Constraint::Percentage(100)];
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Todo List"))
            .items(&app.tasks.items)
            .select(Some(app.tasks.selected))
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .render(f, chunks[0]);
    }
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default().fg(Color::Green);
    let failure_style = Style::default()
        .fg(Color::Red)
        .modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
    let header = ["Server", "Location", "Status"];
    let rows = app.servers.iter().map(|s| {
        let style = if s.status == "Up" {
            up_style
        } else {
            failure_style
        };
        Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
    });
    Table::new(header.into_iter(), rows)
        .block(Block::default().title("Servers").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(10),
        ])
        .render(f, chunks[0]);
}
