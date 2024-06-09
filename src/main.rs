//! Crate to manage something. Haven't decided yet

use std::io;
use todo::{app::App, app::*, app_builder::AppBuilder, errors, ui::ui};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> color_eyre::Result<()> {
    // setup terminal
    errors::install_hooks()?;
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = AppBuilder::default()
        .with_title("AndyCo")
        .with_options([1, 2, 3, 4])
        .into();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(_do_print) = res {
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Menu => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    #[rustfmt::skip]
                    KeyCode::Char('j') | KeyCode::Down => app.change_menu_item(Direction::Up),
                    #[rustfmt::skip]
                    KeyCode::Char('k') | KeyCode::Up => app.change_menu_item(Direction::Down),
                    _ => {}
                },
            }
        }
    }
}
