use guardian_core::{GuardianContext, GuardianError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationPlan {
    pub context: GuardianContext,
    pub asset_id: String,
    pub current_algorithm: String,
    pub target_algorithm: String,
    pub phase: String,
    pub rollback_plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationRequest {
    pub asset_id: String,
    pub current_algorithm: String,
    pub environment: String,
    pub tenant_id: String,
    pub operator: String,
}

pub fn propose_migration(request: MigrationRequest) -> Result<MigrationPlan, GuardianError> {
    let context = GuardianContext {
        environment: request.environment,
        tenant_id: request.tenant_id,
        operator: request.operator,
    };

    guardian_core::validate_context(&context)?;

    let (target_algorithm, phase, rollback) = match request.current_algorithm.as_str() {
        "RSA-2048" => (
            "ML-KEM + X25519 hybrid".to_string(),
            "Phase 1: hybrid key exchange".to_string(),
            "Restore legacy RSA certificate if interoperability fails".to_string(),
        ),
        "ECDSA-P256" => (
            "ML-DSA + ECDSA dual-signature".to_string(),
            "Phase 2: certificate and signing migration".to_string(),
            "Revert to ECDSA certificates while maintaining service continuity".to_string(),
        ),
        _ => (
            "Review policy and select standards-aligned PQC algorithm".to_string(),
            "Phase 0: inventory and standards validation".to_string(),
            "Retain current configuration until compliance review is approved".to_string(),
        ),
    };

    Ok(MigrationPlan {
        context,
        asset_id: request.asset_id,
        current_algorithm: request.current_algorithm,
        target_algorithm,
        phase,
        rollback_plan: rollback,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_rsa_migration_plan() {
        let request = MigrationRequest {
            asset_id: "asset-001".to_string(),
            current_algorithm: "RSA-2048".to_string(),
            environment: "prod".to_string(),
            tenant_id: "tenant-04".to_string(),
            operator: "migration-owner".to_string(),
        };

        let plan = propose_migration(request).unwrap();
        assert_eq!(plan.target_algorithm, "ML-KEM + X25519 hybrid");
        assert!(plan.rollback_plan.contains("legacy RSA"));
    }
}
