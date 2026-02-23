use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use crate::vault_ffi;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
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
        .margin(2)
        .constraints([
            Constraint::Length(5),  // title
            Constraint::Length(12), // space
            Constraint::Min(20),    // menu
        ])
        .split(size);

    let title_lines = vec![
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
    ];

    let title = Paragraph::new(title_lines).alignment(Alignment::Center);
    f.render_widget(title, main_layout[0]);

    let all_items = [
        ("1", "View Passwords", "Browse all entries"),
        ("2", "Add Password", "Create new entry"),
        ("3", "Search", "Find passwords"),
        ("4", "Filter by Tags", "Sort entries"),
        ("5", "Generate Password", "Random secure"),
        ("6", "Delete Entry", "Remove password"),
        ("7", "Exit", "Lock vault"),
    ];

    let menu_list: Vec<ListItem> = all_items
        .iter()
        .enumerate()
        .map(|(i, (num, title, desc))| {
            let is_selected = i == app.selected_menu;

            let title_style = if is_selected {
                Style::default()
                    .fg(app.theme.yellow())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(app.theme.fg())
                    .add_modifier(Modifier::BOLD)
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(
                        format!("[{num}] "),
                        Style::default()
                            .fg(app.theme.orange())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(*title, title_style),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(*desc, Style::default().fg(app.theme.gray())),
                ]),
                Line::from(""),
            ])
        })
        .collect();

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.green()))
        .title(Span::styled(
            " [ Menu ] ",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ));

    let menu = List::new(menu_list).block(menu_block);
    f.render_widget(menu, main_layout[2]);

    if let Some(ref vault) = app.vault {
        let total = vault.e.len();
        let with_2fa = vault.e.iter().filter(|e| e.totp_secret.is_some()).count();
        let tags = app.all_tags.len();

        let cipher = vault_ffi::get_cipher();

        let info_area = Rect {
            x: 4,
            y: main_layout[0].y + main_layout[0].height + 1,
            width: 52,
            height: 10,
        };

        let info_lines = vec![
            Line::from(vec![Span::styled(
                "System Info",
                Style::default()
                    .fg(app.theme.aqua())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Version     ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    "v2.3.5",
                    Style::default()
                        .fg(app.theme.yellow())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Encryption  ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    cipher,
                    Style::default()
                        .fg(app.theme.purple())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Passwords   ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{total}"),
                    Style::default()
                        .fg(app.theme.green())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("2FA Enabled ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{with_2fa}"),
                    Style::default()
                        .fg(app.theme.orange())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Tags        ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    format!("{tags}"),
                    Style::default()
                        .fg(app.theme.blue())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Theme       ", Style::default().fg(app.theme.gray())),
                Span::styled(app.theme.name(), Style::default().fg(app.theme.aqua())),
            ]),
        ];

        let info_box = Paragraph::new(info_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.aqua())),
        );
        f.render_widget(info_box, info_area);
    }

    if !app.msg.is_empty() {
        let status_area = Rect {
            x: size.width.saturating_sub(22),
            y: main_layout[0].y + main_layout[0].height + 1,
            width: 20,
            height: 3,
        };

        let msg_style = match app.msg_type {
            MessageType::Success => Style::default().fg(app.theme.green()),
            MessageType::Error => Style::default().fg(app.theme.red()),
            MessageType::Info => Style::default().fg(app.theme.blue()),
            MessageType::None => Style::default().fg(app.theme.fg()),
        };

        let status_msg = Paragraph::new(app.msg.as_str())
            .style(msg_style.add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(msg_style),
            );
        f.render_widget(status_msg, status_area);
    }
}
