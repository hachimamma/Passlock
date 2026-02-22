use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

#[allow(clippy::too_many_lines)]
pub fn draw_main_menu(f: &mut Frame, size: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, size);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(7), // banner
            Constraint::Length(5), // status boxes
            Constraint::Length(6), // status
            Constraint::Min(8),    // menu + tag distribution
            Constraint::Length(3), // footer
        ])
        .split(size);

    let banner_lines = vec![
        Line::from(vec![Span::styled(
            "██████╗  █████╗ ███████╗███████╗██╗      ██████╗  ██████╗██╗  ██╗",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔══██╗██╔══██╗██╔════╝██╔════╝██║     ██╔═══██╗██╔════╝██║ ██╔╝",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██████╔╝███████║███████╗███████╗██║     ██║   ██║██║     █████╔╝ ",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔═══╝ ██╔══██║╚════██║╚════██║██║     ██║   ██║██║     ██╔═██╗ ",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║     ██║  ██║███████║███████║███████╗╚██████╔╝╚██████╗██║  ██╗",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "╚═╝     ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝",
            Style::default()
                .fg(app.theme.red())
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "                  Secure Password Manager v2.3.5",
            Style::default().fg(app.theme.gray()),
        )]),
    ];
    let banner = Paragraph::new(banner_lines).alignment(Alignment::Center);
    f.render_widget(banner, main_layout[0]);

    let status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    if let Some(ref vault) = app.vault {
        let vault_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    "UNLOCKED ✓",
                    Style::default()
                        .fg(app.theme.green())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Location: ", Style::default().fg(app.theme.gray())),
                Span::styled("~/.passlock.vault", Style::default().fg(app.theme.blue())),
            ]),
            Line::from(vec![
                Span::styled("Size: ", Style::default().fg(app.theme.gray())),
                Span::styled("24.3 KB", Style::default().fg(app.theme.yellow())),
                Span::styled("  │  ", Style::default().fg(app.theme.gray())),
                Span::styled("Modified: ", Style::default().fg(app.theme.gray())),
                Span::styled("Today", Style::default().fg(app.theme.green())),
            ]),
        ];
        let vault_status = Paragraph::new(vault_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.green()))
                .title(" VAULT STATUS "),
        );
        f.render_widget(vault_status, status_layout[0]);

        let security_lines = vec![
            Line::from(vec![
                Span::styled("Encryption: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    "AES-256-GCM",
                    Style::default()
                        .fg(app.theme.purple())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("KDF: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    "Argon2id (64MB / 3 iter)",
                    Style::default().fg(app.theme.aqua()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Clipboard: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    if app.clipboard_countdown.is_some() {
                        "Active"
                    } else {
                        "Empty"
                    },
                    if app.clipboard_countdown.is_some() {
                        Style::default().fg(app.theme.orange())
                    } else {
                        Style::default().fg(app.theme.gray())
                    },
                ),
                Span::styled("  │  Timeout: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    if app.clipboard_timeout == 0 {
                        "Never".to_string()
                    } else {
                        format!("{}s", app.clipboard_timeout)
                    },
                    Style::default().fg(app.theme.yellow()),
                ),
            ]),
        ];
        let security_status = Paragraph::new(security_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.purple()))
                .title(" SECURITY METRICS "),
        );
        f.render_widget(security_status, status_layout[1]);

        let total = vault.e.len();
        let with_2fa = vault.e.iter().filter(|e| e.totp_secret.is_some()).count();
        let with_url = vault.e.iter().filter(|e| e.url.is_some()).count();
        let tags = app.all_tags.len();

        let stats_lines = vec![
            Line::from(vec![
                Span::styled("Total: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{total} "),
                    Style::default()
                        .fg(app.theme.yellow())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("passwords", Style::default().fg(app.theme.fg())),
                Span::styled("  │  ", Style::default().fg(app.theme.gray())),
                Span::styled("Tags: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{tags}"),
                    Style::default()
                        .fg(app.theme.purple())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("2FA Enabled: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{with_2fa} "),
                    Style::default()
                        .fg(app.theme.green())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        "({}%) ",
                        if total > 0 {
                            (with_2fa * 100) / total
                        } else {
                            0
                        }
                    ),
                    Style::default().fg(app.theme.gray()),
                ),
                Span::raw(create_bar(with_2fa, total, 10)),
            ]),
            Line::from(vec![
                Span::styled("With URL: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{with_url} "),
                    Style::default()
                        .fg(app.theme.blue())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        "({}%) ",
                        if total > 0 {
                            (with_url * 100) / total
                        } else {
                            0
                        }
                    ),
                    Style::default().fg(app.theme.gray()),
                ),
                Span::raw(create_bar(with_url, total, 10)),
            ]),
            Line::from(vec![
                Span::styled("Theme: ", Style::default().fg(app.theme.gray())),
                Span::styled(app.theme.name(), Style::default().fg(app.theme.aqua())),
                Span::styled("  │  Press ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    "T/8",
                    Style::default()
                        .fg(app.theme.orange())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to change", Style::default().fg(app.theme.gray())),
            ]),
        ];
        let stats = Paragraph::new(stats_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.yellow()))
                .title(" STATISTICS "),
        );
        f.render_widget(stats, main_layout[2]);
    }

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(main_layout[3]);

    let menu_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_layout[0]);

    let left_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.green()))
        .title(" PASSWORD MANAGEMENT ");

    let left_items = [
        (
            "1",
            "View All",
            "Browse vault",
            format!("{} entries", app.vault.as_ref().map_or(0, |v| v.e.len())),
        ),
        ("2", "Add New", "Create entry", "Quick: Ctrl+N".to_string()),
        (
            "3",
            "Search",
            "Find passwords",
            "Type to filter".to_string(),
        ),
    ];

    let left_list: Vec<ListItem> = left_items
        .iter()
        .enumerate()
        .map(|(i, (num, title, desc, _hint))| {
            let is_selected = app.selected_section == 0 && i == app.selected_menu;
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.fg())
            };
            let prefix = if is_selected { "▶ " } else { "  " };
            let lines = vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{num}] "), Style::default().fg(app.theme.orange())),
                    Span::styled(*title, style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(*desc, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ];
            ListItem::new(lines)
        })
        .collect();

    let left = List::new(left_list).block(left_block);
    f.render_widget(left, menu_layout[0]);

    let right_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.purple()))
        .title(" TOOLS ");

    let right_items = [
        (
            "4",
            "Filter Tags",
            "Sort by tags",
            format!("{} tags", app.all_tags.len()),
        ),
        (
            "5",
            "Generate",
            "Random password",
            "Default: 16 chars".to_string(),
        ),
        ("6", "Delete", "Remove entry", "⚠ Permanent".to_string()),
        ("7", "Exit", "Lock & quit", "Auto-save".to_string()),
    ];

    let right_list: Vec<ListItem> = right_items
        .iter()
        .enumerate()
        .map(|(i, (num, title, desc, _hint))| {
            let is_selected = app.selected_section == 1 && i == app.selected_menu - 3;
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.fg())
            };
            let prefix = if is_selected { "▶ " } else { "  " };
            let lines = vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{num}] "), Style::default().fg(app.theme.orange())),
                    Span::styled(*title, style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(*desc, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ];
            ListItem::new(lines)
        })
        .collect();

    let right = List::new(right_list).block(right_block);
    f.render_widget(right, menu_layout[1]);

    if let Some(ref _vault) = app.vault {
        let mut tag_lines = vec![];
        let max_count = app
            .all_tags
            .iter()
            .map(|(_, c)| c)
            .max()
            .copied()
            .unwrap_or(1);

        for (tag, count) in app.all_tags.iter().take(5) {
            tag_lines.push(Line::from(vec![
                Span::styled(
                    format!("{tag:12} "),
                    Style::default().fg(app.theme.purple()),
                ),
                Span::raw(create_bar(*count, max_count, 8)),
                Span::styled(
                    format!(" {count}"),
                    Style::default().fg(app.theme.yellow()),
                ),
            ]));
        }

        if app.all_tags.len() > 5 {
            tag_lines.push(Line::from(vec![Span::styled(
                format!("... {} more tags", app.all_tags.len() - 5),
                Style::default().fg(app.theme.gray()),
            )]));
        }

        let tag_dist = Paragraph::new(tag_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.orange()))
                .title(" TAG DISTRIBUTION "),
        );
        f.render_widget(tag_dist, content_layout[1]);
    }

    let footer_text = vec![Line::from(vec![
        Span::styled(
            "↑↓/←→",
            Style::default()
                .fg(app.theme.orange())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(": Navigate  ", Style::default().fg(app.theme.gray())),
        Span::styled(
            "Enter",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(": Select  ", Style::default().fg(app.theme.gray())),
        Span::styled(
            "Esc",
            Style::default()
                .fg(app.theme.purple())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(": Options  ", Style::default().fg(app.theme.gray())),
        Span::styled(
            "1-7",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(": Quick select", Style::default().fg(app.theme.gray())),
    ])];

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.gray())),
        );
    f.render_widget(footer, main_layout[4]);

    if !app.msg.is_empty() {
        let msg_area = Rect {
            x: size.width / 4,
            y: size.height - 5,
            width: size.width / 2,
            height: 3,
        };

        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };

        let msg = Paragraph::new(app.msg.as_str())
            .style(msg_style)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(msg_style),
            );
        f.render_widget(msg, msg_area);
    }
}

fn create_bar(value: usize, max: usize, width: usize) -> String {
    if max == 0 {
        return "░".repeat(width);
    }
    let filled = (value * width) / max;
    let mut bar = String::new();
    for i in 0..width {
        if i < filled {
            bar.push('▓');
        } else {
            bar.push('░');
        }
    }
    bar
}
