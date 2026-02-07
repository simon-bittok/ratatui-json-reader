use std::{error::Error, io};

use json_editor::{App, CurrentScreen, CurrentlyEditing, ui};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // special case; normally using Stdout is just fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app)).map_err(|err| {
            eprintln!("ERROR: {err:?}");
            io::Error::new(io::ErrorKind::Other, "Terminal backend error".to_string())
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen() {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.set_current_screen(CurrentScreen::Editing);
                        app.set_currently_editing(Some(CurrentlyEditing::Key));
                    }
                    KeyCode::Char('q') => {
                        app.set_current_screen(CurrentScreen::Exiting);
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = app.currently_editing() {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.set_currently_editing(Some(CurrentlyEditing::Value));
                                }
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.set_current_screen(CurrentScreen::Main);
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = app.currently_editing() {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.pop_key();
                                }
                                CurrentlyEditing::Value => {
                                    app.pop_value();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.set_current_screen(CurrentScreen::Main);
                        app.set_currently_editing(None);
                    }
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = app.currently_editing() {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.push_key(value);
                                }
                                CurrentlyEditing::Value => {
                                    app.push_value(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
