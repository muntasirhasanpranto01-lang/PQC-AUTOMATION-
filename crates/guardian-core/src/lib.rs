use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuardianContext {
    pub environment: String,
    pub tenant_id: String,
    pub operator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetInventory {
    pub asset_id: String,
    pub asset_type: String,
    pub algorithm: String,
    pub location: String,
}

#[derive(Debug, Error)]
pub enum GuardianError {
    #[error("unsupported asset type: {0}")]
    UnsupportedAssetType(String),
    #[error("validation failed: {0}")]
    ValidationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentRegistration {
    pub name: String,
    pub role: String,
    pub status: String,
}

pub fn validate_context(context: &GuardianContext) -> Result<(), GuardianError> {
    if context.environment.trim().is_empty() {
        return Err(GuardianError::ValidationError(
            "guardian context environment cannot be empty".to_string(),
        ));
    }

    if context.tenant_id.trim().is_empty() {
        return Err(GuardianError::ValidationError(
            "guardian context tenant_id cannot be empty".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_expected_context() {
        let context = GuardianContext {
            environment: "prod".to_string(),
            tenant_id: "tenant-01".to_string(),
            operator: "ops-admin".to_string(),
        };

        assert!(validate_context(&context).is_ok());
    }

    #[test]
    fn rejects_empty_environment() {
        let context = GuardianContext {
            environment: String::new(),
            tenant_id: "tenant-01".to_string(),
            operator: "ops-admin".to_string(),
        };

        assert!(validate_context(&context).is_err());
    }
}
