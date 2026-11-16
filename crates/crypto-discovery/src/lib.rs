use guardian_core::{AssetInventory, GuardianContext, GuardianError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryResult {
    pub context: GuardianContext,
    pub assets: Vec<AssetInventory>,
    pub discovered_count: usize,
}

/// Receives observations produced by an authorized inventory process. This API
/// does not scan arbitrary hosts or execute network probes itself.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryRequest {
    pub environment: String,
    pub tenant_id: String,
    pub operator: String,
    pub assets: Vec<AssetInventory>,
}

pub fn build_discovery_result(request: DiscoveryRequest) -> Result<DiscoveryResult, GuardianError> {
    let context = GuardianContext {
        environment: request.environment,
        tenant_id: request.tenant_id,
        operator: request.operator,
    };
    guardian_core::validate_context(&context)?;

    if request.assets.is_empty() {
        return Err(GuardianError::ValidationError(
            "discovery request must include at least one observed asset".into(),
        ));
    }
    if request.assets.len() > 500 {
        return Err(GuardianError::ValidationError(
            "discovery request cannot contain more than 500 assets".into(),
        ));
    }
    for asset in &request.assets {
        if asset.asset_id.trim().is_empty()
            || asset.asset_type.trim().is_empty()
            || asset.algorithm.trim().is_empty()
            || asset.location.trim().is_empty()
        {
            return Err(GuardianError::ValidationError(
                "asset_id, asset_type, algorithm, and location must be non-empty".into(),
            ));
        }
    }

    let discovered_count = request.assets.len();
    Ok(DiscoveryResult {
        context,
        assets: request.assets,
        discovered_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_observed_inventory() {
        let result = build_discovery_result(DiscoveryRequest {
            environment: "stage".into(),
            tenant_id: "tenant-01".into(),
            operator: "scanner".into(),
            assets: vec![AssetInventory {
                asset_id: "asset-001".into(),
                asset_type: "tls".into(),
                algorithm: "RSA-2048".into(),
                location: "service.internal".into(),
            }],
        })
        .unwrap();

        assert_eq!(result.discovered_count, 1);
        assert_eq!(result.assets[0].algorithm, "RSA-2048");
    }

    #[test]
    fn rejects_empty_inventory_instead_of_fabricating_assets() {
        let result = build_discovery_result(DiscoveryRequest {
            environment: "stage".into(),
            tenant_id: "tenant-01".into(),
            operator: "scanner".into(),
            assets: Vec::new(),
        });
        assert!(result.is_err());
    }
}
