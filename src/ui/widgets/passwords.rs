use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use super::utility::centered_rect;
use crate::crypto;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

#[allow(clippy::too_many_lines)]
pub fn draw_view_pwds(f: &mut Frame, size: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.green()))
        .title("═══ PASSWORDS ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block.clone(), size);

    let filter_status = if let Some(ref tag) = app.active_tf {
        format!(" (Filtered by: {tag})")
    } else if !app.search_query.is_empty() {
        format!(" (Search: {})", app.search_query)
    } else {
        String::new()
    };

    let clipboard_status = if let Some(expires_at) = app.clipboard_countdown {
        let now = crate::get_timestamp();
        if expires_at > now {
            format!(" | Clipboard clears in {}s", expires_at - now)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let title = Paragraph::new(format!(
        "Total: {} entries{}{} | Right-click entry for options",
        app.entry_disp.len(),
        filter_status,
        clipboard_status
    ))
    .style(Style::default().fg(app.theme.yellow()))
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    if app.entry_disp.is_empty() {
        let empty = if app.active_tf.is_some() || !app.search_query.is_empty() {
            "[ No matching entries found ]"
        } else {
            "[ No passwords saved yet ]"
        };
        let empty_paragraph = Paragraph::new(empty)
            .style(Style::default().fg(app.theme.gray()))
            .alignment(Alignment::Center);
        f.render_widget(empty_paragraph, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .entry_disp
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let is_selected = i == app.selected_entry;
                let prefix = if is_selected { "▶ " } else { "  " };
                let time_ago = App::get_ta(entry.last_modified);
                let mut lines = vec![
                    Line::from(vec![
                        Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                        Span::styled(
                            format!("[{}] ", i + 1),
                            Style::default().fg(app.theme.orange()),
                        ),
                        Span::styled(
                            &entry.n,
                            if is_selected {
                                Style::default()
                                    .fg(app.theme.yellow())
                                    .add_modifier(Modifier::BOLD)
                            } else {
                                Style::default().fg(app.theme.yellow())
                            },
                        ),
                        Span::styled(
                            format!("  (Modified: {time_ago})"),
                            Style::default().fg(app.theme.gray()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled("├─ User: ", Style::default().fg(app.theme.gray())),
                        Span::styled(&entry.u, Style::default().fg(app.theme.blue())),
                    ]),
                    Line::from(vec![
                        Span::raw("     "),
                        Span::styled("├─ Pass: ", Style::default().fg(app.theme.gray())),
                        Span::styled(&entry.p, Style::default().fg(app.theme.green())),
                    ]),
                ];
                if let Some(ref url) = entry.url {
                    lines.push(Line::from(vec![
                        Span::raw("     "),
                        Span::styled("├─ URL:  ", Style::default().fg(app.theme.gray())),
                        Span::styled(url, Style::default().fg(app.theme.aqua())),
                    ]));
                }
                if !entry.history.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("     "),
                        Span::styled(
                            format!("├─ History: {} changes", entry.history.len()),
                            Style::default().fg(app.theme.purple()),
                        ),
                    ]));
                }
                if entry.tags.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("     "),
                        Span::styled("└─", Style::default().fg(app.theme.gray())),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::raw("     "),
                        Span::styled("└─ Tags: ", Style::default().fg(app.theme.gray())),
                        Span::styled(
                            entry.tags.join(", "),
                            Style::default().fg(app.theme.orange()),
                        ),
                    ]));
                }
                lines.push(Line::from(""));
                ListItem::new(lines)
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::NONE));
        f.render_widget(list, chunks[1]);
    }
    let help = Paragraph::new("| F: Clear filter │ Esc: Back |")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

#[allow(clippy::too_many_lines, clippy::cast_sign_loss)]
pub fn draw_add_pwd(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(80, 85, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.green()))
        .title("═══ ADD NEW PASSWORD ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);
    let title = Paragraph::new("Fill in the details below")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    let active_style = Style::default()
        .fg(app.theme.green())
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(app.theme.gray());
    let name_field =
        Paragraph::new(format!("Name: {}", app.n_entry_name)).style(if app.add_fi == 0 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(name_field, chunks[2]);
    let user_field =
        Paragraph::new(format!("Username: {}", app.n_entry_user)).style(if app.add_fi == 1 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(user_field, chunks[3]);
    let pass_field =
        Paragraph::new(format!("Password: {}", app.n_entry_pass)).style(if app.add_fi == 2 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(pass_field, chunks[4]);
    if !app.n_entry_pass.is_empty() && app.add_fi == 2 {
        let strength = crypto::calc_pwd_strength(&app.n_entry_pass);
        let strength_color = match strength.strength.as_str() {
            "Weak" => app.theme.red(),
            "Fair" => app.theme.orange(),
            "Good" => app.theme.yellow(),
            "Strong" => app.theme.green(),
            _ => app.theme.gray(),
        };
        let bar_width = (35 * strength.percentage) / 100;
        let empty_width = 35 - bar_width;
        let bar = format!(
            "[{}{}] {}% - {}",
            "█".repeat(bar_width as usize),
            "─".repeat(empty_width as usize),
            strength.percentage,
            strength.strength
        );
        let strength_display = Paragraph::new(bar)
            .style(Style::default().fg(strength_color))
            .alignment(Alignment::Center);
        f.render_widget(strength_display, chunks[5]);
        if !strength.feedback.is_empty() {
            let feedback_text = format!("↳ {}", strength.feedback.join(", "));
            let feedback = Paragraph::new(feedback_text)
                .style(Style::default().fg(app.theme.gray()))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(feedback, chunks[6]);
        }
    }
    let url_field =
        Paragraph::new(format!("URL (optional): {}", app.n_entry_url)).style(if app.add_fi == 3 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(url_field, chunks[7]);
    let tags_text = if app.add_fi == 4 {
        format!("Tags: {} ← Enter to add", app.tag_input)
    } else {
        "Tags: (Tab to focus)".to_string()
    };
    let tags_input = Paragraph::new(tags_text)
        .style(if app.add_fi == 4 {
            active_style
        } else {
            inactive_style
        })
        .wrap(Wrap { trim: true });
    f.render_widget(tags_input, chunks[8]);
    if !app.n_entry_tags.is_empty() {
        let tags_display = app
            .n_entry_tags
            .iter()
            .enumerate()
            .map(|(i, tag)| format!("[{}]{} ", i + 1, tag))
            .collect::<Vec<_>>()
            .join(" ");
        let tags_widget = Paragraph::new(format!("Added: {tags_display}"))
            .style(Style::default().fg(app.theme.orange()))
            .wrap(Wrap { trim: true });
        f.render_widget(tags_widget, chunks[9]);
    }
    let notes = Paragraph::new(format!("Notes:\n{}", app.n_entry_notes))
        .style(if app.add_fi == 5 {
            active_style
        } else {
            inactive_style
        })
        .wrap(Wrap { trim: false });
    f.render_widget(notes, chunks[10]);
    if !app.msg.is_empty() {
        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };
        let msg = Paragraph::new(app.msg.as_str())
            .style(msg_style)
            .alignment(Alignment::Center);
        f.render_widget(msg, chunks[11]);
    }
    let help =
        Paragraph::new("| Tab: Next field │ Enter: Add tag/Save │ 1-9: Remove tag │ Esc: Cancel |")
            .style(Style::default().fg(app.theme.gray()))
            .alignment(Alignment::Center);
    f.render_widget(help, chunks[12]);
}

#[allow(clippy::too_many_lines, clippy::cast_sign_loss)]
pub fn draw_edit_pwd(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(80, 85, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.orange()))
        .title("═══ EDIT PASSWORD ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);
    let title = Paragraph::new("Edit entry details (password changes are tracked)")
        .style(Style::default().fg(app.theme.yellow()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    let active_style = Style::default()
        .fg(app.theme.green())
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(app.theme.gray());
    let name_field =
        Paragraph::new(format!("Name: {}", app.n_entry_name)).style(if app.add_fi == 0 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(name_field, chunks[2]);
    let user_field =
        Paragraph::new(format!("Username: {}", app.n_entry_user)).style(if app.add_fi == 1 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(user_field, chunks[3]);
    let pass_field =
        Paragraph::new(format!("Password: {}", app.n_entry_pass)).style(if app.add_fi == 2 {
            active_style
        } else {
            inactive_style
        });
    f.render_widget(pass_field, chunks[4]);
    if !app.n_entry_pass.is_empty() && app.add_fi == 2 {
        let strength = crypto::calc_pwd_strength(&app.n_entry_pass);
        let strength_color = match strength.strength.as_str() {
            "Weak" => app.theme.red(),
            "Fair" => app.theme.orange(),
            "Good" => app.theme.yellow(),
            "Strong" => app.theme.green(),
            _ => app.theme.gray(),
        };
        let bar_width = (35 * strength.percentage) / 100;
        let empty_width = 35 - bar_width;
        let bar = format!(
            "[{}{}] {}% - {}",
            "█".repeat(bar_width as usize),
            "─".repeat(empty_width as usize),
            strength.percentage,
            strength.strength
        );
        let strength_display = Paragraph::new(bar)
            .style(Style::default().fg(strength_color))
            .alignment(Alignment::Center);
        f.render_widget(strength_display, chunks[5]);
        if !strength.feedback.is_empty() {
            let feedback_text = format!("↳ {}", strength.feedback.join(", "));
            let feedback = Paragraph::new(feedback_text)
                .style(Style::default().fg(app.theme.gray()))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(feedback, chunks[6]);
        }
    }
    let url_field = Paragraph::new(format!("URL: {}", app.n_entry_url)).style(if app.add_fi == 3 {
        active_style
    } else {
        inactive_style
    });
    f.render_widget(url_field, chunks[7]);
    let tags_text = if app.add_fi == 4 {
        format!("Tags: {} ← Enter to add", app.tag_input)
    } else {
        "Tags: (Tab to focus)".to_string()
    };
    let tags_input = Paragraph::new(tags_text)
        .style(if app.add_fi == 4 {
            active_style
        } else {
            inactive_style
        })
        .wrap(Wrap { trim: true });
    f.render_widget(tags_input, chunks[8]);
    if !app.n_entry_tags.is_empty() {
        let tags_display = app
            .n_entry_tags
            .iter()
            .enumerate()
            .map(|(i, tag)| format!("[{}]{} ", i + 1, tag))
            .collect::<Vec<_>>()
            .join(" ");
        let tags_widget = Paragraph::new(format!("Tags: {tags_display}"))
            .style(Style::default().fg(app.theme.orange()))
            .wrap(Wrap { trim: true });
        f.render_widget(tags_widget, chunks[9]);
    }
    let notes = Paragraph::new(format!("Notes:\n{}", app.n_entry_notes))
        .style(if app.add_fi == 5 {
            active_style
        } else {
            inactive_style
        })
        .wrap(Wrap { trim: false });
    f.render_widget(notes, chunks[10]);
    if !app.msg.is_empty() {
        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };
        let msg = Paragraph::new(app.msg.as_str())
            .style(msg_style)
            .alignment(Alignment::Center);
        f.render_widget(msg, chunks[11]);
    }
    let help =
        Paragraph::new("| Tab: Next │ Enter: Add tag/Save │ 1-9: Remove tag │ Esc: Cancel |")
            .style(Style::default().fg(app.theme.gray()))
            .alignment(Alignment::Center);
    f.render_widget(help, chunks[12]);
}

pub fn draw_history(f: &mut Frame, size: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.purple()))
        .title("═══ PASSWORD HISTORY ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);
    if let Some(ref vault) = app.vault {
        if app.selected_entry < app.entry_disp.len() {
            let entry = &app.entry_disp[app.selected_entry];
            if let Some(vault_entry) = vault.e.iter().find(|e| e.id == entry.id) {
                let title =
                    Paragraph::new(format!("History for: {} (Last 5 changes)", vault_entry.n))
                        .style(Style::default().fg(app.theme.yellow()))
                        .alignment(Alignment::Center);
                f.render_widget(title, chunks[0]);
                if vault_entry.history.is_empty() {
                    let empty = Paragraph::new("[ No password changes recorded ]")
                        .style(Style::default().fg(app.theme.gray()))
                        .alignment(Alignment::Center);
                    f.render_widget(empty, chunks[1]);
                } else {
                    let items: Vec<ListItem> = vault_entry
                        .history
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, hist)| {
                            let time_ago = App::get_ta(hist.changed_at);
                            let lines = vec![
                                Line::from(vec![
                                    Span::styled(
                                        format!("[{}] ", i + 1),
                                        Style::default().fg(app.theme.purple()),
                                    ),
                                    Span::styled(
                                        &hist.password,
                                        Style::default().fg(app.theme.green()),
                                    ),
                                ]),
                                Line::from(vec![
                                    Span::raw("    "),
                                    Span::styled(
                                        format!("Changed: {time_ago}"),
                                        Style::default().fg(app.theme.gray()),
                                    ),
                                ]),
                                Line::from(""),
                            ];
                            ListItem::new(lines)
                        })
                        .collect();
                    let list = List::new(items).block(Block::default().borders(Borders::NONE));
                    f.render_widget(list, chunks[1]);
                }
            }
        }
    }
    let help = Paragraph::new("| Esc: Back |")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

pub fn draw_del_pwd(f: &mut Frame, size: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(size);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.red()))
        .title("═══ DELETE PASSWORD ═══")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);
    let title = Paragraph::new("⚠ Enter the number of the entry to delete")
        .style(Style::default().fg(app.theme.orange()))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    let empty_vec = Vec::new();
    let entries_to_display = if app.entry_disp.is_empty() {
        app.vault.as_ref().map_or(&empty_vec, |v| &v.e)
    } else {
        &app.entry_disp
    };
    if entries_to_display.is_empty() {
        let empty = Paragraph::new("[ No passwords to delete ]")
            .style(Style::default().fg(app.theme.gray()))
            .alignment(Alignment::Center);
        f.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = entries_to_display
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("[{}] ", i + 1),
                        Style::default().fg(app.theme.red()),
                    ),
                    Span::styled(&entry.n, Style::default().fg(app.theme.fg())),
                ]))
            })
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::NONE));
        f.render_widget(list, chunks[1]);
    }
    let input = Paragraph::new(format!("Entry number: {}", app.input_buffer)).style(
        Style::default()
            .fg(app.theme.red())
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(input, chunks[2]);
    let help = Paragraph::new("Type number │ Enter: Delete │ Esc: Cancel")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[3]);
}

pub fn draw_context_menu(f: &mut Frame, app: &App) {
    let menu_width = 24u16;
    let menu_height = 7u16;
    let x = app.context_menu_x;
    let y = app.context_menu_y;

    let area = Rect {
        x,
        y,
        width: menu_width,
        height: menu_height,
    };

    let entry_idx = app.context_menu_entry_idx;
    let has_url = if entry_idx < app.entry_disp.len() {
        app.entry_disp[entry_idx].url.is_some()
    } else {
        false
    };

    let selected = app.context_menu_selected;
    let orange = app.theme.orange();
    let fg = app.theme.fg();
    let gray = app.theme.gray();
    let bg1 = app.theme.bg1();

    let items = [("󰆒", "Copy Password"),
        ("󰀄", "Copy Username"),
        ("󰖟", "Copy URL"),
        ("󰏫", "Edit Entry"),
        ("󰔩", "View History")];

    let menu_lines: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(idx, (icon, label))| {
            let is_selected = idx == selected;
            let is_disabled = idx == 2 && !has_url;

            let style = if is_selected {
                Style::default().fg(orange).add_modifier(Modifier::BOLD)
            } else if is_disabled {
                Style::default().fg(gray)
            } else {
                Style::default().fg(fg)
            };

            Line::from(vec![
                Span::raw(" "),
                Span::styled(*icon, style),
                Span::raw(" "),
                Span::styled(*label, style),
            ])
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(orange))
        .style(Style::default().bg(bg1));

    let paragraph = Paragraph::new(menu_lines).block(block);

    f.render_widget(paragraph, area);
}
