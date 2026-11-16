-- Tenant-scoped inventory: all reads and writes include tenant_id.
CREATE TABLE IF NOT EXISTS cryptographic_assets (
    tenant_id TEXT NOT NULL,
    asset_id TEXT NOT NULL,
    asset_type TEXT NOT NULL,
    algorithm TEXT NOT NULL,
    location TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, asset_id)
);

CREATE INDEX IF NOT EXISTS idx_crypto_assets_tenant_algorithm
    ON cryptographic_assets (tenant_id, algorithm);
