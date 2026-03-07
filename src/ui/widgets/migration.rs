use super::super::app::App;
use super::super::colors::ThemeColors;
use super::super::screens::MessageType;
use super::utility::centered_rect;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

#[allow(clippy::too_many_lines)]
pub fn draw_import_export_menu(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(70, 75, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.purple()))
        .title(Span::styled(
            " [ Import/Export Manager ] ",
            Style::default()
                .fg(app.theme.purple())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(2),  // title
            Constraint::Length(1),  // spacer
            Constraint::Length(10), // import
            Constraint::Length(1),  // spacer
            Constraint::Length(14), // export
            Constraint::Min(0),     // bottom
        ])
        .split(area);

    let title = Paragraph::new("Select an Import/Export operation")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let import_items = [("1", "Import from CSV", "Import passwords from CSV file"),
        ("2", "Import from JSON", "Import passwords from JSON file")];

    let import_lines: Vec<Line> = std::iter::once(Line::from(vec![Span::styled(
        "IMPORT:",
        Style::default()
            .fg(app.theme.green())
            .add_modifier(Modifier::BOLD),
    )]))
    .chain(std::iter::once(Line::from("")))
    .chain(
        import_items
            .iter()
            .enumerate()
            .map(|(i, (num, title, desc))| {
                let is_selected = app.import_export_menu_index == i;
                let prefix = if is_selected { "▶ " } else { "  " };
                let style = if is_selected {
                    Style::default()
                        .fg(app.theme.yellow())
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.fg())
                };

                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{num}] {title}"), style),
                    Span::styled(format!(" - {desc}"), Style::default().fg(app.theme.gray())),
                ])
            }),
    )
    .collect();

    let import_widget = Paragraph::new(import_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.green())),
    );
    f.render_widget(import_widget, chunks[2]);

    let export_items = [("3", "Export to CSV (All)", "Export all passwords to CSV"),
        ("4", "Export to CSV (Filtered)", "Export filtered passwords"),
        ("5", "Export to JSON", "Export to JSON format"),
        ("6", "Export Encrypted Vault", "Export encrypted backup")];

    let export_lines: Vec<Line> = std::iter::once(Line::from(vec![Span::styled(
        "EXPORT:",
        Style::default()
            .fg(app.theme.blue())
            .add_modifier(Modifier::BOLD),
    )]))
    .chain(std::iter::once(Line::from("")))
    .chain(
        export_items
            .iter()
            .enumerate()
            .map(|(i, (num, title, desc))| {
                let menu_idx = i + 2;
                let is_selected = app.import_export_menu_index == menu_idx;
                let prefix = if is_selected { "▶ " } else { "  " };
                let style = if is_selected {
                    Style::default()
                        .fg(app.theme.yellow())
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(app.theme.fg())
                };

                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(app.theme.yellow())),
                    Span::styled(format!("[{num}] {title}"), style),
                    Span::styled(format!(" - {desc}"), Style::default().fg(app.theme.gray())),
                ])
            }),
    )
    .collect();

    let export_widget = Paragraph::new(export_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.blue())),
    );
    f.render_widget(export_widget, chunks[4]);

    let help = Paragraph::new("[↑/↓] Navigate  [Enter] Select  [Esc] Back")
        .style(Style::default().fg(app.theme.gray()))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[5]);
}

pub fn draw_import_csv(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(70, 60, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.green()))
        .title(Span::styled(
            " [ Import from CSV ] ",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Import passwords from CSV file")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let file_path_text = if app.import_file_path.is_empty() {
        "File path: _".to_string()
    } else {
        format!("File path: {}", app.import_file_path)
    };
    let file_path = Paragraph::new(file_path_text)
        .style(
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.green())),
        );
    f.render_widget(file_path, chunks[2]);

    let help_lines = vec![
        Line::from(vec![
            Span::styled("Type file path", Style::default().fg(app.theme.gray())),
            Span::styled(" (e.g., ", Style::default().fg(app.theme.gray())),
            Span::styled(
                "/home/user/passwords.csv",
                Style::default().fg(app.theme.blue()),
            ),
            Span::styled(")", Style::default().fg(app.theme.gray())),
        ]),
        Line::from(vec![Span::styled(
            "[Enter] Preview  [Esc] Cancel",
            Style::default().fg(app.theme.yellow()),
        )]),
    ];
    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);

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
        f.render_widget(msg, chunks[5]);
    }
}

pub fn draw_import_json(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(70, 60, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.green()))
        .title(Span::styled(
            " [ Import from JSON ] ",
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Import passwords from JSON file")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let file_path_text = if app.import_file_path.is_empty() {
        "File path: _".to_string()
    } else {
        format!("File path: {}", app.import_file_path)
    };
    let file_path = Paragraph::new(file_path_text)
        .style(
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.green())),
        );
    f.render_widget(file_path, chunks[2]);

    let help_lines = vec![
        Line::from(vec![
            Span::styled("Type file path", Style::default().fg(app.theme.gray())),
            Span::styled(" (e.g., ", Style::default().fg(app.theme.gray())),
            Span::styled(
                "/home/user/passwords.json",
                Style::default().fg(app.theme.blue()),
            ),
            Span::styled(")", Style::default().fg(app.theme.gray())),
        ]),
        Line::from(vec![Span::styled(
            "[Enter] Preview  [Esc] Cancel",
            Style::default().fg(app.theme.yellow()),
        )]),
    ];
    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);

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
        f.render_widget(msg, chunks[5]);
    }
}

#[allow(clippy::too_many_lines)]
pub fn draw_import_preview(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(85, 85, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.purple()))
        .title(Span::styled(
            " [ Import Preview ] ",
            Style::default()
                .fg(app.theme.purple())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(6), // stats
            Constraint::Length(1), // spacer
            Constraint::Min(8),    // sample entries
            Constraint::Length(1), // spacer
            Constraint::Length(7), // duplicate handling
            Constraint::Length(2), // help
        ])
        .split(area);

    if let Some(ref preview) = app.import_preview {
        let stats_lines = vec![
            Line::from(vec![
                Span::styled(
                    "Total entries in file:  ",
                    Style::default().fg(app.theme.gray()),
                ),
                Span::styled(
                    format!("{}", preview.total_entries),
                    Style::default()
                        .fg(app.theme.yellow())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Valid entries:          ",
                    Style::default().fg(app.theme.gray()),
                ),
                Span::styled(
                    format!("{}", preview.valid_entries),
                    Style::default()
                        .fg(app.theme.green())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Empty/invalid entries:  ",
                    Style::default().fg(app.theme.gray()),
                ),
                Span::styled(
                    format!("{}", preview.empty_entries),
                    Style::default().fg(app.theme.red()),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Duplicates found:       ",
                    Style::default().fg(app.theme.gray()),
                ),
                Span::styled(
                    format!("{}", preview.duplicates),
                    Style::default()
                        .fg(app.theme.orange())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];
        let stats = Paragraph::new(stats_lines);
        f.render_widget(stats, chunks[0]);

        let sample_lines: Vec<Line> = std::iter::once(Line::from(vec![Span::styled(
            "Sample entries (first 5):",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]))
        .chain(std::iter::once(Line::from("")))
        .chain(
            preview
                .entries
                .iter()
                .take(5)
                .enumerate()
                .flat_map(|(idx, entry)| {
                    let dup_marker = if entry.is_duplicate {
                        " [DUPLICATE]"
                    } else {
                        ""
                    };
                    let totp_marker = if entry.has_totp { " [2FA]" } else { "" };

                    let mut lines = vec![
                        Line::from(vec![
                            Span::styled(
                                format!("{}. ", idx + 1),
                                Style::default().fg(app.theme.gray()),
                            ),
                            Span::styled(
                                &entry.name,
                                Style::default()
                                    .fg(app.theme.fg())
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(dup_marker, Style::default().fg(app.theme.orange())),
                            Span::styled(totp_marker, Style::default().fg(app.theme.green())),
                        ]),
                        Line::from(vec![
                            Span::styled("   Username: ", Style::default().fg(app.theme.gray())),
                            Span::styled(&entry.username, Style::default().fg(app.theme.blue())),
                        ]),
                    ];

                    if let Some(ref url) = entry.url {
                        lines.push(Line::from(vec![
                            Span::styled("   URL: ", Style::default().fg(app.theme.gray())),
                            Span::styled(url, Style::default().fg(app.theme.aqua())),
                        ]));
                    }

                    lines.push(Line::from(""));
                    lines
                }),
        )
        .chain(if preview.entries.len() > 5 {
            vec![Line::from(vec![Span::styled(
                format!("... and {} more entries", preview.entries.len() - 5),
                Style::default().fg(app.theme.gray()),
            )])]
        } else {
            vec![]
        })
        .collect();

        let sample = Paragraph::new(sample_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.purple())),
        );
        f.render_widget(sample, chunks[2]);

        let dup_options = [("Import all (create duplicates)", 0),
            ("Skip duplicates (keep existing)", 1),
            ("Merge/update duplicates", 2)];

        let dup_lines: Vec<Line> = std::iter::once(Line::from(vec![Span::styled(
            "Duplicate handling:",
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )]))
        .chain(std::iter::once(Line::from("")))
        .chain(dup_options.iter().map(|(title, idx)| {
            let is_selected = app.duplicate_handling == *idx;
            let marker = if is_selected { "●" } else { "○" };
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.green())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.gray())
            };

            Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(marker, style),
                Span::styled(format!(" {title}"), style),
            ])
        }))
        .collect();

        let dup_widget = Paragraph::new(dup_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.yellow())),
        );
        f.render_widget(dup_widget, chunks[4]);

        let help_lines = vec![
            Line::from(vec![Span::styled(
                "[↑/↓] Select duplicate handling",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                "[Enter] Import  [Esc] Cancel",
                Style::default().fg(app.theme.yellow()),
            )]),
        ];
        let help = Paragraph::new(help_lines).alignment(Alignment::Center);
        f.render_widget(help, chunks[5]);
    }
}

#[allow(clippy::too_many_lines)]
pub fn draw_export_csv(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(75, 75, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.blue()))
        .title(Span::styled(
            " [ Export to CSV ] ",
            Style::default()
                .fg(app.theme.blue())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(2), // title
            Constraint::Length(1), // spacer
            Constraint::Length(7), // filter options
            Constraint::Length(1), // spacer
            Constraint::Length(3), // filter val
            Constraint::Length(1), // spacer
            Constraint::Length(3), // output path
            Constraint::Length(1), // spacer
            Constraint::Length(3), // help
            Constraint::Min(0),    // bottom + msg
        ])
        .split(area);

    let title = Paragraph::new("Export passwords to CSV file")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let filter_options = [("All entries", 0),
        ("Filter by tag", 1),
        ("Filter by search", 2)];

    let filter_lines: Vec<Line> = std::iter::once(Line::from(vec![Span::styled(
        "Filter by:",
        Style::default()
            .fg(app.theme.yellow())
            .add_modifier(Modifier::BOLD),
    )]))
    .chain(std::iter::once(Line::from("")))
    .chain(filter_options.iter().map(|(title, idx)| {
        let is_selected = app.export_filter_type == *idx;
        let marker = if is_selected { "●" } else { "○" };
        let style = if is_selected {
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.gray())
        };

        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(marker, style),
            Span::styled(format!(" {title}"), style),
        ])
    }))
    .collect();

    let filter_widget = Paragraph::new(filter_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.yellow())),
    );
    f.render_widget(filter_widget, chunks[2]);

    if app.export_filter_type > 0 {
        let filter_label = if app.export_filter_type == 1 {
            "Tag name: "
        } else {
            "Search query: "
        };

        let filter_value_text = if app.export_filter_value.is_empty() {
            format!("{filter_label}_")
        } else {
            format!("{}{}", filter_label, app.export_filter_value)
        };

        let filter_value_style = if app.export_file_path.is_empty() {
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.gray())
        };

        let filter_value = Paragraph::new(filter_value_text)
            .style(filter_value_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(app.theme.green())),
            );
        f.render_widget(filter_value, chunks[4]);
    }
    let file_path_text = if app.export_file_path.is_empty() {
        "Output file: _".to_string()
    } else {
        format!("Output file: {}", app.export_file_path)
    };

    let file_path_style = if !app.export_file_path.is_empty()
        || (app.export_filter_type == 0)
        || (app.export_filter_type > 0 && !app.export_filter_value.is_empty())
    {
        Style::default()
            .fg(app.theme.green())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.theme.gray())
    };

    let file_path = Paragraph::new(file_path_text).style(file_path_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.blue())),
    );
    f.render_widget(file_path, chunks[6]);

    let help_lines = if app.export_filter_type == 0 {
        vec![
            Line::from(vec![Span::styled(
                "[↑/↓] Select filter type",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                "Type file path, then [Enter] to export",
                Style::default().fg(app.theme.yellow()),
            )]),
            Line::from(vec![Span::styled(
                "[Esc] Cancel",
                Style::default().fg(app.theme.gray()),
            )]),
        ]
    } else if app.export_filter_value.is_empty() {
        vec![
            Line::from(vec![Span::styled(
                "[↑/↓] Change filter type",
                Style::default().fg(app.theme.gray()),
            )]),
            Line::from(vec![Span::styled(
                if app.export_filter_type == 1 {
                    "Type TAG NAME, then file path, then [Enter]"
                } else {
                    "Type SEARCH QUERY, then file path, then [Enter]"
                },
                Style::default().fg(app.theme.yellow()),
            )]),
            Line::from(vec![Span::styled(
                "[Esc] Cancel",
                Style::default().fg(app.theme.gray()),
            )]),
        ]
    } else {
        vec![
            Line::from(vec![
                Span::styled("Filter: ", Style::default().fg(app.theme.gray())),
                Span::styled(
                    &app.export_filter_value,
                    Style::default().fg(app.theme.green()),
                ),
            ]),
            Line::from(vec![Span::styled(
                "Type file path, then [Enter] to export",
                Style::default().fg(app.theme.yellow()),
            )]),
            Line::from(vec![Span::styled(
                "[Backspace] to edit filter  [Esc] Cancel",
                Style::default().fg(app.theme.gray()),
            )]),
        ]
    };

    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    f.render_widget(help, chunks[8]);

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
        f.render_widget(msg, chunks[9]);
    }
}

pub fn draw_export_json(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(70, 60, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.blue()))
        .title(Span::styled(
            " [ Export to JSON ] ",
            Style::default()
                .fg(app.theme.blue())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Export passwords to JSON file")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let file_path_text = if app.export_file_path.is_empty() {
        "Output file: _".to_string()
    } else {
        format!("Output file: {}", app.export_file_path)
    };
    let file_path = Paragraph::new(file_path_text)
        .style(
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.green())),
        );
    f.render_widget(file_path, chunks[2]);

    let help_lines = vec![
        Line::from(vec![Span::styled(
            "Type output file path",
            Style::default().fg(app.theme.gray()),
        )]),
        Line::from(vec![Span::styled(
            "[Enter] Export  [Esc] Cancel",
            Style::default().fg(app.theme.yellow()),
        )]),
    ];
    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);

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
        f.render_widget(msg, chunks[5]);
    }
}

pub fn draw_export_vault(f: &mut Frame, size: Rect, app: &App) {
    let area = centered_rect(70, 60, size);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.purple()))
        .title(Span::styled(
            " [ Export Encrypted Vault ] ",
            Style::default()
                .fg(app.theme.purple())
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.bg0()));
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    let title = Paragraph::new("Export encrypted vault backup")
        .style(
            Style::default()
                .fg(app.theme.yellow())
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let file_path_text = if app.export_file_path.is_empty() {
        "Output file: _".to_string()
    } else {
        format!("Output file: {}", app.export_file_path)
    };
    let file_path = Paragraph::new(file_path_text)
        .style(
            Style::default()
                .fg(app.theme.green())
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(app.theme.green())),
        );
    f.render_widget(file_path, chunks[2]);

    let help_lines = vec![
        Line::from(vec![Span::styled(
            "Type output file path (e.g., /home/user/backup.vault)",
            Style::default().fg(app.theme.gray()),
        )]),
        Line::from(vec![Span::styled(
            "[Enter] Export  [Esc] Cancel",
            Style::default().fg(app.theme.yellow()),
        )]),
    ];
    let help = Paragraph::new(help_lines).alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);

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
        f.render_widget(msg, chunks[5]);
    }
}
