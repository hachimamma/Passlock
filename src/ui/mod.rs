pub mod app;
pub mod colors;
pub mod handlers;
pub mod screens;
pub mod widgets;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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
    draw_add_pwd, draw_create_vault, draw_del_pwd, draw_edit_pwd, draw_filter_tags, draw_gen_pwd,
    draw_help_screen, draw_history, draw_loading, draw_main_menu, draw_options_menu,
    draw_search_pwd, draw_settings_screen, draw_theme_selector, draw_unlock_vault, draw_view_pwds,
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

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        if app.should_quit {
            break;
        }

        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
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
}
