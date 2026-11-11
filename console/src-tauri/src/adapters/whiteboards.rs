use kernel::adapters::meta::RequestMeta;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWhiteboard {
    pub title: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub workspace_identifier: Option<Uuid>,
    pub meta: Option<RequestMeta>,
}

impl From<CreateWhiteboard> for kernel::adapters::whiteboards::CreateWhiteboard {
    fn from(whiteboard: CreateWhiteboard) -> Self {
        Self {
            title: whiteboard.title,
            payload: whiteboard.payload,
            workspace_identifier: whiteboard.workspace_identifier,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWhiteboard {
    pub title: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub meta: Option<RequestMeta>,
}

impl From<UpdateWhiteboard> for kernel::adapters::whiteboards::UpdateWhiteboard {
    fn from(whiteboard: UpdateWhiteboard) -> Self {
        Self {
            title: whiteboard.title,
            payload: whiteboard.payload,
        }
    }
}
