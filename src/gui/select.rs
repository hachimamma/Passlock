use crate::models::Entry;
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SelectDialog {
    entries: Vec<Entry>,
    selected_index: Option<usize>,
    result: Arc<Mutex<Option<Entry>>>,
    should_close: bool,
}

impl SelectDialog {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            selected_index: None,
            result: Arc::new(Mutex::new(None)),
            should_close: false,
        }
    }

    fn select_entry(&mut self, index: usize) {
        if index < self.entries.len() {
            if let Ok(mut result) = self.result.try_lock() {
                *result = Some(self.entries[index].clone());
            }
            self.should_close = true;
        }
    }
}

impl eframe::App for SelectDialog {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔐 Select Password to Auto-Fill");
            ui.add_space(10.0);
            
            ui.separator();
            ui.add_space(5.0);

            if self.entries.is_empty() {
                ui.label("❌ No matching passwords found");
                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.should_close = true;
                }
                return;
            }

            ui.label(format!("Found {} matching password(s):", self.entries.len()));
            ui.add_space(10.0);

            // Scrollable list of entries
            let mut selected_idx = None;
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, entry) in self.entries.iter().enumerate() {
                    let is_selected = self.selected_index == Some(i);

                    ui.horizontal(|ui| {
                        let button_text = if is_selected {
                            format!("▶ {} ({})", entry.n, entry.u)
                        } else {
                            format!("  {} ({})", entry.n, entry.u)
                        };

                        if ui.button(&button_text).clicked() {
                            selected_idx = Some(i);
                        }

                        // Show URL if available
                        if let Some(ref url) = entry.url {
                            ui.label(format!("🌍 {}", url));
                        }

                        // Show tags if available
                        if !entry.tags.is_empty() {
                            ui.label(format!("🏷️  {}", entry.tags.join(", ")));
                        }
                    });
                    ui.add_space(5.0);
                }
            });

            // Handle selection after the borrow ends
            if let Some(idx) = selected_idx {
                self.selected_index = Some(idx);
                self.select_entry(idx);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Cancel button
            if ui.button("❌ Cancel").clicked() {
                self.should_close = true;
            }
        });
    }
}

pub fn show_select_dialog(entries: Vec<Entry>) -> Option<Entry> {
    let dialog = SelectDialog::new(entries);
    let result = Arc::clone(&dialog.result);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_resizable(true)
            .with_title("PassLock - Select Password"),
        ..Default::default()
    };

    // Run the dialog
    let _ = eframe::run_native(
        "PassLock Select",
        native_options,
        Box::new(|_cc| Ok(Box::new(dialog))),
    );

    // Retrieve result
    result.try_lock().ok().and_then(|r| r.clone())
}