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
                "<C-w> support",
                "This todo-app doesn't delete full words when pressing <C-w>",
            ),
            ("desc", "cool2"),
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
                        to_change,
                    } => match key.code {
                        KeyCode::Backspace => drop(
                            match editing {
                                CurrentEdit::Title => title,
                                CurrentEdit::Body => description,
                            }
                            .pop(),
                        ),
                        KeyCode::Esc => app.popup = None,
                        KeyCode::Enter => {
                            if let Some(x) = to_change {
                                app.options[*x] = (
                                    title.to_owned().into_boxed_str(),
                                    description.to_owned().into_boxed_str(),
                                    0,
                                );
                            } else {
                                app.options.push((
                                    title.to_owned().into_boxed_str(),
                                    description.to_owned().into_boxed_str(),
                                    0,
                                ));
                            }
                            app.popup = None;
                        }
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
                    Popup::Help(ref mut x) => match key.code {
                        KeyCode::Char('q') => app.popup = None,
                        KeyCode::Char('j') => *x += 1,
                        KeyCode::Char('k') => *x = x.saturating_sub(1),
                        _ => (),
                    },
                }
            } else {
                match app.current_selection {
                    CurrentSelection::Menu => match key.code {
                        // quit
                        KeyCode::Char('q') => return Ok(true),
                        //
                        KeyCode::Char('?') => app.popup = Some(Popup::Help(0)),

                        // Vim motion + Down key
                        KeyCode::Char('j') | KeyCode::Down => app.change_menu_item(Direction::Up),
                        // Vim motion + Down key
                        KeyCode::Char('k') | KeyCode::Up => app.change_menu_item(Direction::Down),
                        // Enter edit mode
                        KeyCode::Char('e') if app.selected.is_some() => app.edit(),
                        // Enter add mode (Add a new item)
                        KeyCode::Char('a') => app.add(),
                        // Focus the description
                        KeyCode::Enter => {
                            if app.selected.is_some() {
                                app.current_selection = CurrentSelection::Description
                            }
                        }

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
                        _ => (),
                    },

                    CurrentSelection::Description => match key.code {
                        // quit
                        KeyCode::Char('q') => app.current_selection = CurrentSelection::Menu,
                        // Vim motions
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.options[app.selected.unwrap()].2 += 1
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.options[app.selected.unwrap()].2 =
                                app.options[app.selected.unwrap()].2.saturating_sub(1)
                        }
                        _ => (),
                    },
                }
            }
        }
    }
}
