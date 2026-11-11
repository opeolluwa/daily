use kernel::adapters::meta::RequestMeta;
use kernel::entities::sea_orm_active_enums::Priority as TaskPriority;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub priority: String,
    pub meta: Option<RequestMeta>,
}

impl From<CreateTask> for kernel::adapters::tasks::CreateTask {
    fn from(t: CreateTask) -> Self {
        use chrono::NaiveDate;

        let due_date = t
            .due_date
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        let priority = match t.priority.as_str() {
            "high" => TaskPriority::High,
            "low" => TaskPriority::Low,
            _ => TaskPriority::Medium,
        };

        Self {
            title: t.title,
            description: t.description,
            due_date,
            priority,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

impl From<UpdateTask> for kernel::adapters::tasks::UpdateTask {
    fn from(t: UpdateTask) -> Self {
        Self {
            title: t.title,
            description: t.description,
            status: t.status,
        }
    }
}
