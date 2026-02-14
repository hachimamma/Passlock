use super::screens::{InputField, MessageType, Screen};
use crate::crypto;
use crate::models::{Entry, PasswordHistory, Vault};
use crate::storage;
use std::collections::HashMap;

pub struct App {
    pub screen: Screen,
    pub vault: Option<Vault>,
    pub master_pwd: String,
    pub selected_menu: usize,
    pub selected_section: usize,
    pub selected_entry: usize,
    pub input_field: InputField,
    pub input_buffer: String,
    pub input_buffer2: String,
    pub msg: String,
    pub msg_type: MessageType,
    pub entry_disp: Vec<Entry>,
    pub search_query: String,
    pub gen_pwd: String,
    #[allow(dead_code)]
    pub scroll_offset: usize,
    pub n_entry_name: String,
    pub n_entry_user: String,
    pub n_entry_pass: String,
    pub n_entry_url: String,
    pub n_entry_notes: String,
    pub n_entry_tags: Vec<String>,
    pub tag_input: String,
    pub add_fi: usize,
    pub all_tags: Vec<(String, usize)>,
    pub select_tf: usize,
    pub active_tf: Option<String>,
    pub edit_eid: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::VaultCheck,
            vault: None,
            master_pwd: String::new(),
            selected_menu: 0,
            selected_section: 0,
            selected_entry: 0,
            input_field: InputField::None,
            input_buffer: String::new(),
            input_buffer2: String::new(),
            msg: String::new(),
            msg_type: MessageType::None,
            entry_disp: Vec::new(),
            search_query: String::new(),
            gen_pwd: String::new(),
            scroll_offset: 0,
            n_entry_name: String::new(),
            n_entry_user: String::new(),
            n_entry_pass: String::new(),
            n_entry_url: String::new(),
            n_entry_notes: String::new(),
            n_entry_tags: Vec::new(),
            tag_input: String::new(),
            add_fi: 0,
            all_tags: Vec::new(),
            select_tf: 0,
            active_tf: None,
            edit_eid: String::new(),
            should_quit: false,
        }
    }

    pub fn check_vault(&mut self) {
        if storage::vt_exi() {
            self.screen = Screen::UnlockVault;
            self.input_field = InputField::Password;
        } else {
            self.screen = Screen::CreateVault;
            self.input_field = InputField::Password;
            self.set_msg(
                "No vault found. Create one to get started!",
                MessageType::Info,
            );
        }
    }

    pub fn create_vault(&mut self) {
        if self.input_buffer.len() < 4 {
            self.set_msg("Password too short (min 4 chars)", MessageType::Error);
            return;
        }
        if self.input_buffer != self.input_buffer2 {
            self.set_msg("Passwords don't match!", MessageType::Error);
            return;
        }
        let salt = crypto::gen_salt();
        let vault = Vault::new(salt);
        match storage::svv(&vault, &self.input_buffer) {
            Ok(()) => {
                self.master_pwd = self.input_buffer.clone();
                self.vault = Some(vault);
                self.screen = Screen::MainMenu;
                self.input_buffer.clear();
                self.input_buffer2.clear();
                self.input_field = InputField::None;
                self.set_msg("Vault created successfully!", MessageType::Success);
            }
            Err(e) => {
                self.set_msg(&format!("Failed to create vault: {e}"), MessageType::Error);
            }
        }
    }

    pub fn unlock_vault(&mut self) {
        match storage::ld_vt(&self.input_buffer) {
            Ok(vault) => {
                self.master_pwd = self.input_buffer.clone();
                self.vault = Some(vault);
                self.screen = Screen::MainMenu;
                self.input_buffer.clear();
                self.input_field = InputField::None;
                self.set_msg("Vault unlocked!", MessageType::Success);
                self.load_at();
                if let Some(ref vault) = self.vault {
                    self.entry_disp = vault.e.clone();
                }
            }
            Err(_) => {
                self.set_msg("Wrong password!", MessageType::Error);
            }
        }
    }

    pub fn add_entry(&mut self) {
        if self.n_entry_name.is_empty()
            || self.n_entry_user.is_empty()
            || self.n_entry_pass.is_empty()
        {
            self.set_msg(
                "Name, Username, and Password are required!",
                MessageType::Error,
            );
            return;
        }
        let now = crate::get_timestamp();
        let entry = Entry {
            id: crate::generate_uuid(),
            n: self.n_entry_name.clone(),
            u: self.n_entry_user.clone(),
            p: self.n_entry_pass.clone(),
            url: if self.n_entry_url.is_empty() {
                None
            } else {
                Some(self.n_entry_url.clone())
            },
            nt: if self.n_entry_notes.is_empty() {
                None
            } else {
                Some(self.n_entry_notes.clone())
            },
            t: now,
            tags: self.n_entry_tags.clone(),
            history: Vec::new(),
            last_modified: now,
        };
        if let Some(ref mut vault) = self.vault {
            vault.e.push(entry);
            if let Err(e) = storage::svv(vault, &self.master_pwd) {
                self.set_msg(&format!("Failed to save: {e}"), MessageType::Error);
            } else {
                self.set_msg("Password added successfully!", MessageType::Success);
                self.ca_form();
                self.screen = Screen::MainMenu;
                self.load_at();
                if let Some(ref vault) = self.vault {
                    self.entry_disp = vault.e.clone();
                }
            }
        }
    }

    pub fn edit_entry(&mut self) {
        if self.n_entry_name.is_empty()
            || self.n_entry_user.is_empty()
            || self.n_entry_pass.is_empty()
        {
            self.set_msg(
                "Name, Username, and Password are required!",
                MessageType::Error,
            );
            return;
        }
        if let Some(ref mut vault) = self.vault {
            if let Some(entry) = vault.e.iter_mut().find(|e| e.id == self.edit_eid) {
                let now = crate::get_timestamp();
                if entry.p != self.n_entry_pass {
                    if entry.history.len() >= 5 {
                        entry.history.remove(0);
                    }
                    entry.history.push(PasswordHistory {
                        password: entry.p.clone(),
                        changed_at: now,
                    });
                }
                entry.n.clone_from(&self.n_entry_name);
                entry.u.clone_from(&self.n_entry_user);
                entry.p.clone_from(&self.n_entry_pass);
                entry.url = if self.n_entry_url.is_empty() {
                    None
                } else {
                    Some(self.n_entry_url.clone())
                };
                entry.nt = if self.n_entry_notes.is_empty() {
                    None
                } else {
                    Some(self.n_entry_notes.clone())
                };
                entry.tags.clone_from(&self.n_entry_tags);
                entry.last_modified = now;

                if let Err(e) = storage::svv(vault, &self.master_pwd) {
                    self.set_msg(&format!("Failed to save: {e}"), MessageType::Error);
                } else {
                    self.set_msg("Entry updated successfully!", MessageType::Success);
                    self.ca_form();
                    self.screen = Screen::MainMenu;
                    self.load_at();
                    if let Some(ref vault) = self.vault {
                        self.entry_disp = vault.e.clone();
                    }
                }
            }
        }
    }

    pub fn load_efe(&mut self, entry_id: &str) {
        if let Some(ref vault) = self.vault {
            if let Some(entry) = vault.e.iter().find(|e| e.id == entry_id) {
                self.edit_eid = entry.id.clone();
                self.n_entry_name = entry.n.clone();
                self.n_entry_user = entry.u.clone();
                self.n_entry_pass = entry.p.clone();
                self.n_entry_url = entry.url.clone().unwrap_or_default();
                self.n_entry_notes = entry.nt.clone().unwrap_or_default();
                self.n_entry_tags = entry.tags.clone();
                self.add_fi = 0;
                self.screen = Screen::EditPassword;
            }
        }
    }

    pub fn delete_entry(&mut self, index: usize) {
        if let Some(ref mut vault) = self.vault {
            if index < vault.e.len() {
                let removed = vault.e.remove(index);
                if let Err(e) = storage::svv(vault, &self.master_pwd) {
                    self.set_msg(&format!("Failed to save: {e}"), MessageType::Error);
                } else {
                    self.set_msg(&format!("Deleted '{}'", removed.n), MessageType::Success);
                    self.screen = Screen::MainMenu;
                    self.load_at();
                    if let Some(ref vault) = self.vault {
                        self.entry_disp = vault.e.clone();
                    }
                }
            } else {
                self.set_msg("Invalid entry number!", MessageType::Error);
            }
        }
    }

    pub fn search_entries(&mut self) {
        if let Some(ref vault) = self.vault {
            let query = self.search_query.to_lowercase();
            if query.is_empty() {
                self.entry_disp = vault.e.clone();
            } else {
                self.entry_disp = vault
                    .e
                    .iter()
                    .filter(|e| {
                        e.n.to_lowercase().contains(&query)
                            || e.u.to_lowercase().contains(&query)
                            || e.url
                                .as_ref()
                                .is_some_and(|u| u.to_lowercase().contains(&query))
                            || e.tags.iter().any(|t| t.to_lowercase().contains(&query))
                    })
                    .cloned()
                    .collect();
            }
        }
    }

    pub fn gen_pwd(&mut self) {
        let len = self
            .input_buffer
            .parse::<usize>()
            .unwrap_or(16)
            .clamp(4, 64);
        self.gen_pwd = crypto::gen_pwd(len);
    }

    pub fn set_msg(&mut self, msg: &str, msg_type: MessageType) {
        self.msg = msg.to_string();
        self.msg_type = msg_type;
    }

    pub fn ca_form(&mut self) {
        self.n_entry_name.clear();
        self.n_entry_user.clear();
        self.n_entry_pass.clear();
        self.n_entry_url.clear();
        self.n_entry_notes.clear();
        self.n_entry_tags.clear();
        self.tag_input.clear();
        self.add_fi = 0;
        self.edit_eid.clear();
    }

    pub fn add_tag(&mut self) {
        let tag = self.tag_input.trim().to_lowercase();
        if !tag.is_empty() && !self.n_entry_tags.contains(&tag) {
            self.n_entry_tags.push(tag);
            self.tag_input.clear();
        }
    }

    pub fn remove_tag(&mut self, index: usize) {
        if index < self.n_entry_tags.len() {
            self.n_entry_tags.remove(index);
        }
    }

    pub fn load_at(&mut self) {
        if let Some(ref vault) = self.vault {
            let mut tag_map: HashMap<String, usize> = HashMap::new();
            for entry in &vault.e {
                for tag in &entry.tags {
                    *tag_map.entry(tag.clone()).or_insert(0) += 1;
                }
            }
            self.all_tags = tag_map.into_iter().collect();
            self.all_tags.sort_by(|a, b| b.1.cmp(&a.1));
        }
    }

    pub fn filter_bt(&mut self, tag: Option<String>) {
        self.active_tf.clone_from(&tag);
        if let Some(ref vault) = self.vault {
            if let Some(filter_tag) = tag {
                self.entry_disp = vault
                    .e
                    .iter()
                    .filter(|e| e.tags.contains(&filter_tag))
                    .cloned()
                    .collect();
            } else {
                self.entry_disp = vault.e.clone();
            }
        }
    }

    pub fn get_ta(timestamp: u64) -> String {
        let now = crate::get_timestamp();
        let diff = now.saturating_sub(timestamp);
        let days = diff / 86400;
        if days == 0 {
            "Today".to_string()
        } else if days == 1 {
            "1 day ago".to_string()
        } else if days < 30 {
            format!("{days} days ago")
        } else if days < 365 {
            let months = days / 30;
            if months == 1 {
                "1 month ago".to_string()
            } else {
                format!("{months} months ago")
            }
        } else {
            let years = days / 365;
            if years == 1 {
                "1 year ago".to_string()
            } else {
                format!("{years} years ago")
            }
        }
    }
}
