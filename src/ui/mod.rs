pub mod app;
pub mod clipboard;
pub mod colors;
pub mod handlers;
pub mod screens;
pub mod widgets;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io;

use app::App;
use handlers::{
    handle_api, handle_cvi, handle_di, handle_epi, handle_gi, handle_help_screen, handle_mmi,
    handle_options_menu, handle_settings_screen, handle_si, handle_tfi, handle_theme_selector,
    handle_uvi, handle_vhi, handle_vpi,
};
use screens::Screen;
use widgets::{
    draw_add_pwd, draw_context_menu, draw_create_vault, draw_del_pwd, draw_edit_pwd,
    draw_filter_tags, draw_gen_pwd, draw_help_screen, draw_history, draw_loading, draw_main_menu,
    draw_options_menu, draw_search_pwd, draw_settings_screen, draw_theme_selector,
    draw_unlock_vault, draw_view_pwds,
};

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.check_vault();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn handle_context_menu_click(app: &mut App, click_x: u16, click_y: u16) {
    use screens::MessageType;

    let menu_x = app.context_menu_x;
    let menu_y = app.context_menu_y;
    let menu_width = 24u16;
    let menu_height = 7u16;

    if click_x >= menu_x
        && click_x < menu_x + menu_width
        && click_y >= menu_y
        && click_y < menu_y + menu_height
    {
        let item = click_y.saturating_sub(menu_y + 1) as usize;
        let entry_idx = app.context_menu_entry_idx;

        if entry_idx >= app.entry_disp.len() {
            app.context_menu_visible = false;
            return;
        }

        let entry = app.entry_disp[entry_idx].clone();
        const TIMEOUT: u64 = 30;

        match item {
            0 => {
                let result = clipboard::copy_with_timeout(&entry.p, TIMEOUT);
                app.clipboard_countdown = if result.success { Some(result.expires_at) } else { None };
                app.set_msg(&result.message, if result.success { MessageType::Success } else { MessageType::Error });
            }
            1 => {
                let result = clipboard::copy_with_timeout(&entry.u, TIMEOUT);
                app.clipboard_countdown = if result.success { Some(result.expires_at) } else { None };
                app.set_msg(&result.message, if result.success { MessageType::Success } else { MessageType::Error });
            }
            2 => {
                if let Some(ref url) = entry.url {
                    let result = clipboard::copy_with_timeout(url, TIMEOUT);
                    app.clipboard_countdown = if result.success { Some(result.expires_at) } else { None };
                    app.set_msg(&result.message, if result.success { MessageType::Success } else { MessageType::Error });
                } else {
                    app.set_msg("No URL for this entry!", MessageType::Error);
                }
            }
            3 => {
                let entry_id = entry.id.clone();
                app.load_efe(&entry_id);
            }
            4 => {
                app.selected_entry = entry_idx;
                app.screen = Screen::ViewHistory;
            }
            _ => {}
        }
    }

    app.context_menu_visible = false;
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        if app.should_quit {
            break;
        }

        if let Some(expires_at) = app.clipboard_countdown {
            if crate::get_timestamp() >= expires_at {
                app.clipboard_countdown = None;
                app.msg.clear();
            }
        }

        terminal.draw(|f| ui(f, app))?;

        match event::read()? {
            Event::Mouse(mouse_event) => {
                match mouse_event.kind {
                    MouseEventKind::Down(MouseButton::Right) => {
                        if app.screen == Screen::ViewPasswords
                            || app.screen == Screen::SearchPassword
                        {
                            let y = mouse_event.row;
                            let x = mouse_event.column;
                            let header_offset = 5u16;
                            if y > header_offset {
                                let entry_idx = ((y - header_offset) / 7) as usize;
                                if entry_idx < app.entry_disp.len() {
                                    app.context_menu_visible = true;
                                    app.context_menu_entry_idx = entry_idx;
                                    app.context_menu_selected = 0;
                                    app.context_menu_x = x;
                                    app.context_menu_y = y;
                                }
                            }
                        }
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        if app.context_menu_visible {
                            handle_context_menu_click(app, mouse_event.column, mouse_event.row);
                        } else {
                            app.context_menu_visible = false;
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if app.screen == Screen::ViewPasswords && app.selected_entry > 0 {
                            app.selected_entry -= 1;
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if app.screen == Screen::ViewPasswords
                            && app.selected_entry < app.entry_disp.len().saturating_sub(1)
                        {
                            app.selected_entry += 1;
                        }
                    }
                    _ => {}
                }
            }
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if app.context_menu_visible {
                        app.context_menu_visible = false;
                        continue;
                    }

                    match app.screen {
                        Screen::VaultCheck => {}
                        Screen::CreateVault => handle_cvi(app, key_event.code),
                        Screen::UnlockVault => handle_uvi(app, key_event.code),
                        Screen::MainMenu => {
                            if handle_mmi(app, key_event.code) {
                                return Ok(());
                            }
                        }
                        Screen::ViewPasswords => handle_vpi(app, key_event.code),
                        Screen::AddPassword => {
                            if key_event.code == KeyCode::Char('s')
                                && key_event
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL)
                            {
                                app.add_entry();
                            } else {
                                handle_api(app, key_event.code);
                            }
                        }
                        Screen::EditPassword => {
                            if key_event.code == KeyCode::Char('s')
                                && key_event
                                    .modifiers
                                    .contains(crossterm::event::KeyModifiers::CONTROL)
                            {
                                app.edit_entry();
                            } else {
                                handle_epi(app, key_event.code);
                            }
                        }
                        Screen::ViewHistory => handle_vhi(app, key_event.code),
                        Screen::SearchPassword => handle_si(app, key_event.code),
                        Screen::GeneratePassword => handle_gi(app, key_event.code),
                        Screen::DeletePassword => handle_di(app, key_event.code),
                        Screen::FilterByTag => handle_tfi(app, key_event.code),
                        Screen::ThemeSelector => handle_theme_selector(app, key_event.code),
                        Screen::OptionsMenu => {
                            if handle_options_menu(app, key_event.code) {
                                return Ok(());
                            }
                        }
                        Screen::Help => handle_help_screen(app, key_event.code),
                        Screen::Settings => handle_settings_screen(app, key_event.code),
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.size();
    match app.screen {
        Screen::VaultCheck => draw_loading(f, size, app),
        Screen::CreateVault => draw_create_vault(f, size, app),
        Screen::UnlockVault => draw_unlock_vault(f, size, app),
        Screen::MainMenu => draw_main_menu(f, size, app),
        Screen::ViewPasswords => draw_view_pwds(f, size, app),
        Screen::AddPassword => draw_add_pwd(f, size, app),
        Screen::EditPassword => draw_edit_pwd(f, size, app),
        Screen::ViewHistory => draw_history(f, size, app),
        Screen::SearchPassword => draw_search_pwd(f, size, app),
        Screen::GeneratePassword => draw_gen_pwd(f, size, app),
        Screen::DeletePassword => draw_del_pwd(f, size, app),
        Screen::FilterByTag => draw_filter_tags(f, size, app),
        Screen::ThemeSelector => draw_theme_selector(f, size, app),
        Screen::OptionsMenu => draw_options_menu(f, size, app),
        Screen::Help => draw_help_screen(f, size, app),
        Screen::Settings => draw_settings_screen(f, size, app),
    }

    if app.context_menu_visible {
        draw_context_menu(f, app);
    }
}