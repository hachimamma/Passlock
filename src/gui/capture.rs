use crate::models::Entry;
use crate::daemon::window::WindowContext;
use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CaptureDialog {
    name: String,
    username: String,
    password: String,
    url: String,
    tags: String,
    notes: String,
    result: Arc<Mutex<Option<Entry>>>,
    should_close: bool,
}

impl CaptureDialog {
    pub fn new(context: WindowContext) -> Self {
        Self {
            name: context.suggested_name,
            username: String::new(),
            password: String::new(),
            url: context.suggested_url.unwrap_or_default(),
            tags: String::new(),
            notes: String::new(),
            result: Arc::new(Mutex::new(None)),
            should_close: false,
        }
    }

    fn save_entry(&mut self) {
        if self.name.is_empty() || self.username.is_empty() || self.password.is_empty() {
            return;
        }

        let now = crate::get_timestamp();
        let entry = Entry {
            id: crate::generate_uuid(),
            n: self.name.clone(),
            u: self.username.clone(),
            p: self.password.clone(),
            url: if self.url.is_empty() { None } else { Some(self.url.clone()) },
            nt: if self.notes.is_empty() { None } else { Some(self.notes.clone()) },
            t: now,
            tags: self.tags.split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect(),
            history: Vec::new(),
            last_modified: now,
        };

        if let Ok(mut result) = self.result.try_lock() {
            *result = Some(entry);
        }

        self.should_close = true;
    }
}

impl eframe::App for CaptureDialog {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.should_close {
            frame.close();
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Save Password to PassLock");
            ui.add_space(10.0);
            
            ui.separator();
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.add_space(10.0);
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.add_space(10.0);
                ui.text_edit_singleline(&mut self.username);
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.add_space(10.0);
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.add_space(10.0);
                ui.text_edit_singleline(&mut self.url);
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Tags:");
                ui.add_space(10.0);
                ui.text_edit_singleline(&mut self.tags);
            });
            ui.label("   (comma-separated)");
            ui.add_space(5.0);

            ui.label("Notes:");
            ui.text_edit_multiline(&mut self.notes);
            ui.add_space(10.0);

            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.save_entry();
                }
                ui.add_space(10.0);
                if ui.button("Cancel").clicked() {
                    self.should_close = true;
                }
            });
        });
    }
}

pub fn show_capture_dialog(context: WindowContext) -> Option<Entry> {
    let dialog = CaptureDialog::new(context);
    let result = Arc::clone(&dialog.result);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 450.0])
            .with_resizable(false)
            .with_title("PassLock - Capture Password"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "PassLock Capture",
        native_options,
        Box::new(|_cc| Ok(Box::new(dialog))),
    );

    result.try_lock().ok().and_then(|r| r.clone())
}