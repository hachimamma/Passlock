use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PasswordHistory {
    pub password: String,
    pub changed_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub id: String,
    pub n: String,
    pub u: String,
    pub p: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nt: Option<String>,
    pub t: u64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<PasswordHistory>,
    #[serde(default)]
    pub last_modified: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vault {
    pub e: Vec<Entry>,
    pub s: String,
}

impl Vault {
    pub fn new(salt: String) -> Self {
        Self {
            e: Vec::new(),
            s: salt,
        }
    }
}
