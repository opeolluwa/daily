use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppContext {
    #[serde(default)]
    pub notes: Vec<ContextItem>,
    #[serde(default)]
    pub tasks: Vec<ContextItem>,
    #[serde(default)]
    pub bookmarks: Vec<ContextItem>,
    #[serde(default)]
    pub reminders: Vec<ContextItem>,
    #[serde(default)]
    pub extra: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextItem {
    pub title: String,
    pub summary: String,
    pub identifier: Option<String>,
}

impl AppContext {
    pub fn to_system_prompt_section(&self) -> String {
        let mut parts = Vec::new();

        if !self.notes.is_empty() {
            parts.push("## User's Notes".to_string());
            for item in &self.notes {
                parts.push(format!("- **{}**: {}", item.title, item.summary));
            }
        }

        if !self.tasks.is_empty() {
            parts.push("## User's Tasks".to_string());
            for item in &self.tasks {
                parts.push(format!("- **{}**: {}", item.title, item.summary));
            }
        }

        if !self.bookmarks.is_empty() {
            parts.push("## User's Bookmarks".to_string());
            for item in &self.bookmarks {
                parts.push(format!("- **{}**: {}", item.title, item.summary));
            }
        }

        if !self.reminders.is_empty() {
            parts.push("## User's Reminders".to_string());
            for item in &self.reminders {
                parts.push(format!("- **{}**: {}", item.title, item.summary));
            }
        }

        if let Some(extra) = &self.extra {
            parts.push(format!("## Additional Context\n{extra}"));
        }

        parts.join("\n\n")
    }
}
