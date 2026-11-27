use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskCategory {
    pub name: String,
    pub color: String,
}

impl From<CreateTaskCategory> for kernel::adapters::task_category::CreateTaskCategory {
    fn from(t: CreateTaskCategory) -> Self {
        Self {
            name: t.name,
            color: t.color,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskCategory {
    pub name: Option<String>,
    pub color: Option<String>,
}

impl From<UpdateTaskCategory> for kernel::adapters::task_category::UpdateTaskCategory {
    fn from(t: UpdateTaskCategory) -> Self {
        Self {
            name: t.name,
            color: t.color,
        }
    }
}
