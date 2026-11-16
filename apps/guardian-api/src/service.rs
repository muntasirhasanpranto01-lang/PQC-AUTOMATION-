use crate::repository::{AssetBatchLoader, AssetRepository, PgAssetRepository};
use crypto_discovery::DiscoveryResult;
use guardian_core::GuardianContext;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};

pub struct GuardianService {
    repository: Arc<dyn AssetRepository>,
    loader: AssetBatchLoader,
    pool: PgPool,
}

impl GuardianService {
    pub fn new(pool: PgPool) -> Self {
        let repository: Arc<dyn AssetRepository> = Arc::new(PgAssetRepository::new(pool.clone()));
        let loader = AssetBatchLoader::new(Arc::clone(&repository));
        Self { repository, loader, pool }
    }

    pub async fn health(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn record_discovery(&self, result: &DiscoveryResult) -> Result<(), sqlx::Error> {
        self.repository
            .upsert_many(&result.context.tenant_id, &result.assets)
            .await
    }

    pub async fn load_assets(
        &self,
        context: &GuardianContext,
        asset_ids: &[String],
    ) -> Result<HashMap<String, guardian_core::AssetInventory>, sqlx::Error> {
        self.loader.load_many(context, asset_ids).await
    }
}
