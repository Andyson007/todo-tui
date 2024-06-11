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
        .with_options([
            (
                "<C-w> support".to_string(),
                "This todo-app doesn't delete full words when pressing <C-w>".to_string(),
            ),
            ("desc".to_string(), "cool2".to_string()),
        ])
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

fn run_app<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    B: Backend,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            if let Some(ref mut popup) = app.popup {
                match popup {
                    Popup::Edit {
                        ref mut title,
                        ref mut description,
                        ref mut editing,
                    } => match key.code {
                        KeyCode::Backspace => drop(
                            match editing {
                                CurrentEdit::Title => title,
                                CurrentEdit::Body => description,
                            }
                            .pop(),
                        ),
                        KeyCode::Esc => app.popup = None,
                        KeyCode::Enter => app.popup = None,
                        KeyCode::Tab => {
                            *editing = match editing {
                                CurrentEdit::Title => CurrentEdit::Body,
                                CurrentEdit::Body => CurrentEdit::Title,
                            }
                        }
                        KeyCode::Char(x) => match editing {
                            CurrentEdit::Title => title,
                            CurrentEdit::Body => description,
                        }
                        .push(x),
                        _ => (),
                    },
                    Popup::Add { .. } => todo!(),
                }
            } else {
                match app.current_mode {
                    CurrentScreen::Menu => match key.code {
                        // quit
                        KeyCode::Char('q') => return Ok(true),
                        // Vim motion + Down key
                        KeyCode::Char('j') | KeyCode::Down => app.change_menu_item(Direction::Up),
                        // Vim motion + Down key
                        KeyCode::Char('k') | KeyCode::Up => app.change_menu_item(Direction::Down),
                        // Enter edit mode
                        KeyCode::Char('e') if app.selected.is_some() => app.edit(),
                        // Enter add mode (Add a new item)
                        KeyCode::Char('a') => app.add(),

                        // Delete entry
                        KeyCode::Char('d') if app.selected.is_some() => {
                            let selected = unsafe { app.selected.unwrap_unchecked() };
                            app.options.remove(selected);
                            if selected == app.options.len() {
                                if app.options.is_empty() {
                                    app.selected = None
                                } else {
                                    app.selected = Some(selected - 1);
                                }
                            }
                        }
                        _ => {}
                    },

                    CurrentScreen::Description => todo!(),
                }
            }
        }
    }
}
