# PQC Guardian API

Rust/Axum HTTP API for submitting authorized cryptographic inventory observations, assessing quantum-related risk, looking up supported PQC standards metadata, and generating migration plans. This is an MVP service: discovery accepts observations from an authorized scanner; it does not probe arbitrary networks. Risk and migration results are planning aids, not cryptographic validation or authorization to change production systems.

## Routes

| Method | Path | Purpose |
| --- | --- | --- |
| GET | `/health` | Checks API and PostgreSQL connectivity |
| POST | `/api/v1/discovery/inventory` | Validates and stores up to 500 observed assets for a tenant |
| POST | `/api/v1/risk/assess` | Returns a risk assessment for a submitted asset/algorithm |
| POST | `/api/v1/intelligence/algorithms` | Returns PQC algorithm metadata for a guardian context |
| POST | `/api/v1/migrations/plan` | Generates a proposed migration plan; does not execute it |

### Discovery example

```json
{
  "environment": "staging",
  "tenant_id": "tenant-01",
  "operator": "inventory-service",
  "assets": [
    {
      "asset_id": "edge-001",
      "asset_type": "tls",
      "algorithm": "RSA-2048",
      "location": "gateway.internal"
    }
  ]
}
```

### Risk example

```json
{
  "asset_id": "edge-001",
  "algorithm": "RSA-2048",
  "environment": "staging",
  "tenant_id": "tenant-01",
  "operator": "risk-service"
}
```

### Intelligence example

```json
{
  "environment": "staging",
  "tenant_id": "tenant-01",
  "operator": "pqc-planner"
}
```

### Migration plan example

```json
{
  "asset_id": "edge-001",
  "current_algorithm": "RSA-2048",
  "environment": "staging",
  "tenant_id": "tenant-01",
  "operator": "migration-planner"
}
```

All endpoints currently accept JSON bodies without authentication. Deploy only behind an authenticated gateway or add the planned identity/RBAC middleware before exposing this service to untrusted clients. Never send private keys or secret material to these endpoints.

## Run locally

Requirements: Rust stable, Docker Compose, and SQLx CLI (`cargo install sqlx-cli --no-default-features --features rustls,postgres`).

```bash
cp .env.example .env
 docker compose up --build
```

The API runs at `http://localhost:8080`; Compose starts PostgreSQL and the API. The service runs its embedded SQLx migrations during startup. To run without Compose:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/pqc_guardian
cargo run -p guardian-api
```

Run checks:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo check --workspace
```

Optional manual migration management:

```bash
sqlx migrate run --source apps/guardian-api/migrations
```

The SQLx migrator is embedded in the binary, so production startup applies pending migrations. For environments where migrations are managed separately, run the SQLx CLI command in the deployment pipeline and use a separate runtime role with only the permissions the service needs.
