use guardian_core::{GuardianContext, GuardianError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiskAssessment {
    pub context: GuardianContext,
    pub asset_id: String,
    pub algorithm: String,
    pub risk_score: u32,
    pub assessment: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiskRequest {
    pub asset_id: String,
    pub algorithm: String,
    pub environment: String,
    pub tenant_id: String,
    pub operator: String,
}

pub fn assess_risk(request: RiskRequest) -> Result<RiskAssessment, GuardianError> {
    let context = GuardianContext {
        environment: request.environment,
        tenant_id: request.tenant_id,
        operator: request.operator,
    };

    guardian_core::validate_context(&context)?;

    let risk_score = match request.algorithm.as_str() {
        "RSA-2048" => 85,
        "ECDSA-P256" => 72,
        _ => 45,
    };

    let assessment = if risk_score >= 80 {
        "Critical quantum exposure risk".to_string()
    } else {
        "Moderate quantum exposure risk".to_string()
    };

    let recommendation = match request.algorithm.as_str() {
        "RSA-2048" => "Prioritize ML-KEM migration with hybrid TLS transition".to_string(),
        "ECDSA-P256" => "Plan ML-DSA migration and certificate replacement".to_string(),
        _ => "Continue inventory review and evaluate post-quantum readiness".to_string(),
    };

    Ok(RiskAssessment {
        context,
        asset_id: request.asset_id,
        algorithm: request.algorithm,
        risk_score,
        assessment,
        recommendation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates_rsa_as_critical() {
        let request = RiskRequest {
            asset_id: "asset-001".to_string(),
            algorithm: "RSA-2048".to_string(),
            environment: "prod".to_string(),
            tenant_id: "tenant-02".to_string(),
            operator: "risk-admin".to_string(),
        };

        let result = assess_risk(request).unwrap();
        assert_eq!(result.risk_score, 85);
        assert_eq!(result.assessment, "Critical quantum exposure risk");
    }
}
