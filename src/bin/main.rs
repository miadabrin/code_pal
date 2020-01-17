use code_pal::app;

use std::{
    io::{stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

use structopt::StructOpt;
use tui::{backend::CrosstermBackend, Terminal};

use crate::app::{ui, App, AppState};
use crossterm::terminal::LeaveAlternateScreen;

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "tick-rate", default_value = "250")]
    tick_rate: u64,
    #[structopt(long = "log")]
    log: bool,
}

fn main() -> Result<(), failure::Error> {
    let cli = Cli::from_args();
    stderrlog::new().quiet(!cli.log).verbosity(4).init()?;

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if let Ok(_) = event::poll(Duration::from_millis(cli.tick_rate)) {
                if let Ok(CEvent::Key(key)) = event::read() {
                    tx.send(Event::Input(key)).unwrap_or_default();
                }
            }

            tx.send(Event::Tick).unwrap_or_default();
        }
    });

    let app_state = AppState::new();
    let mut app = App::new("Code Pal", app_state);

    terminal.clear()?;

    loop {
        ui::draw(&mut terminal, &mut app)?;
        match rx.recv()? {
            Event::Input(event) => match (event.code, event.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    break;
                }
                (_, _) => app.on_key(event),
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
