use async_trait::async_trait;
use guardian_core::{AssetInventory, GuardianContext};
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::{collections::HashMap, sync::Arc};

#[async_trait]
pub trait AssetRepository: Send + Sync {
    async fn upsert_many(
        &self,
        tenant_id: &str,
        assets: &[AssetInventory],
    ) -> Result<(), sqlx::Error>;

    async fn find_many(
        &self,
        tenant_id: &str,
        asset_ids: &[String],
    ) -> Result<Vec<AssetInventory>, sqlx::Error>;
}

#[derive(Clone)]
pub struct PgAssetRepository {
    pool: PgPool,
}

impl PgAssetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AssetRepository for PgAssetRepository {
    async fn upsert_many(
        &self,
        tenant_id: &str,
        assets: &[AssetInventory],
    ) -> Result<(), sqlx::Error> {
        if assets.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;
        for asset in assets {
            sqlx::query(
                r#"INSERT INTO cryptographic_assets
                   (tenant_id, asset_id, asset_type, algorithm, location)
                   VALUES ($1, $2, $3, $4, $5)
                   ON CONFLICT (tenant_id, asset_id) DO UPDATE SET
                     asset_type = EXCLUDED.asset_type,
                     algorithm = EXCLUDED.algorithm,
                     location = EXCLUDED.location,
                     updated_at = NOW()"#,
            )
            .bind(tenant_id)
            .bind(&asset.asset_id)
            .bind(&asset.asset_type)
            .bind(&asset.algorithm)
            .bind(&asset.location)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await
    }

    async fn find_many(
        &self,
        tenant_id: &str,
        asset_ids: &[String],
    ) -> Result<Vec<AssetInventory>, sqlx::Error> {
        if asset_ids.is_empty() {
            return Ok(Vec::new());
        }
        sqlx::query_as::<_, AssetRow>(
            r#"SELECT asset_id, asset_type, algorithm, location
               FROM cryptographic_assets
               WHERE tenant_id = $1 AND asset_id = ANY($2)"#,
        )
        .bind(tenant_id)
        .bind(asset_ids)
        .fetch_all(&self.pool)
        .await
        .map(|rows| rows.into_iter().map(AssetInventory::from).collect())
    }
}

#[derive(sqlx::FromRow)]
struct AssetRow {
    asset_id: String,
    asset_type: String,
    algorithm: String,
    location: String,
}

impl From<AssetRow> for AssetInventory {
    fn from(row: AssetRow) -> Self {
        Self {
            asset_id: row.asset_id,
            asset_type: row.asset_type,
            algorithm: row.algorithm,
            location: row.location,
        }
    }
}

/// Request-scoped batch loader: collect IDs and issue one tenant-scoped query,
/// avoiding one database round trip per asset.
#[derive(Clone)]
pub struct AssetBatchLoader {
    repository: Arc<dyn AssetRepository>,
}

impl AssetBatchLoader {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }

    pub async fn load_many(
        &self,
        context: &GuardianContext,
        asset_ids: &[String],
    ) -> Result<HashMap<String, AssetInventory>, sqlx::Error> {
        let rows = self
            .repository
            .find_many(&context.tenant_id, asset_ids)
            .await?;
        Ok(rows
            .into_iter()
            .map(|asset| (asset.asset_id.clone(), asset))
            .collect())
    }
}

// Keep the query builder type referenced here as a compile-time guard for the
// PostgreSQL repository's dynamic-query pattern used by future batch writes.
#[allow(dead_code)]
fn _postgres_query_builder() -> QueryBuilder<'static, Postgres> {
    QueryBuilder::new("SELECT 1")
}
