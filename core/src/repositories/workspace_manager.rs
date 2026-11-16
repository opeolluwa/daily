
use crate::error::KernelError;

#[async_trait::async_trait]
pub trait TransferRecord {
    async fn transfer_record(
        &self,
        record_identifier: &str,
        previous_workspace_identifier: &str,
        target_workspace_identifier: &str,
    ) -> Result<(), KernelError>;
}

#[async_trait::async_trait]
pub trait DuplicateRecord {
    async fn duplicate_record(
        &self,
        record_identifier: &str,
        previous_workspace_identifier: &str,
        target_workspace_identifier: &str,
    ) -> Result<String, KernelError>;
}

#[async_trait::async_trait]
pub trait RecordExistInWorkspace {
    async fn record_exists_in_workspace(
        &self,
        record_identifier: &str,
        workspace_identifier: &str,
    ) -> Result<bool, KernelError>;
}
