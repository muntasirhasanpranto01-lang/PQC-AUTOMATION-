use guardian_core::{GuardianContext, GuardianError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlgorithmProfile {
    pub name: String,
    pub standard: String,
    pub status: String,
    pub recommended_use: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntelligenceContext {
    pub context: GuardianContext,
    pub algorithm_profiles: Vec<AlgorithmProfile>,
}

pub fn build_intelligence_report(context: GuardianContext) -> Result<IntelligenceContext, GuardianError> {
    guardian_core::validate_context(&context)?;

    let algorithm_profiles = vec![
        AlgorithmProfile {
            name: "ML-KEM".to_string(),
            standard: "FIPS 203".to_string(),
            status: "finalized".to_string(),
            recommended_use: "Key encapsulation for hybrid TLS and SSH".to_string(),
        },
        AlgorithmProfile {
            name: "ML-DSA".to_string(),
            standard: "FIPS 204".to_string(),
            status: "finalized".to_string(),
            recommended_use: "Digital signatures and certificate replacement".to_string(),
        },
        AlgorithmProfile {
            name: "SLH-DSA".to_string(),
            standard: "FIPS 205".to_string(),
            status: "finalized".to_string(),
            recommended_use: "Signature-only deployment in conservative ecosystems".to_string(),
        },
    ];

    Ok(IntelligenceContext {
        context,
        algorithm_profiles,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_finalized_algorithms() {
        let context = GuardianContext {
            environment: "prod".to_string(),
            tenant_id: "tenant-03".to_string(),
            operator: "pqc-expert".to_string(),
        };

        let report = build_intelligence_report(context).unwrap();
        assert_eq!(report.algorithm_profiles.len(), 3);
        assert_eq!(report.algorithm_profiles[0].name, "ML-KEM");
    }
}
