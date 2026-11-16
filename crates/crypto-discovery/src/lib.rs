use guardian_core::{AssetInventory, GuardianContext, GuardianError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryResult {
    pub context: GuardianContext,
    pub assets: Vec<AssetInventory>,
    pub discovered_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryRequest {
    pub environment: String,
    pub tenant_id: String,
    pub operator: String,
    pub scan_target: String,
}

pub fn build_discovery_result(request: DiscoveryRequest) -> Result<DiscoveryResult, GuardianError> {
    let context = GuardianContext {
        environment: request.environment,
        tenant_id: request.tenant_id,
        operator: request.operator,
    };

    guardian_core::validate_context(&context)?;

    let assets = vec![
        AssetInventory {
            asset_id: "asset-001".to_string(),
            asset_type: "tls".to_string(),
            algorithm: "RSA-2048".to_string(),
            location: request.scan_target,
        },
        AssetInventory {
            asset_id: "asset-002".to_string(),
            asset_type: "certificate".to_string(),
            algorithm: "ECDSA-P256".to_string(),
            location: request.scan_target,
        },
    ];

    Ok(DiscoveryResult {
        context,
        discovered_count: assets.len(),
        assets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_discovery_report_for_valid_request() {
        let request = DiscoveryRequest {
            environment: "stage".to_string(),
            tenant_id: "tenant-01".to_string(),
            operator: "scanner".to_string(),
            scan_target: "internal-gateway".to_string(),
        };

        let result = build_discovery_result(request).unwrap();
        assert_eq!(result.discovered_count, 2);
        assert_eq!(result.assets[0].algorithm, "RSA-2048");
    }
}
