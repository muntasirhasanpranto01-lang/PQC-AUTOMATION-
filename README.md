# PQC Guardian AI

PQC Guardian AI is an enterprise-grade Rust-first platform for discovering cryptographic assets, identifying quantum-vulnerable deployments, and automating migration planning for post-quantum cryptography.

## Repository status

This repository is initialized with the first production-oriented workspace scaffold for Phase 1 of the PQC Guardian AI roadmap.

## Initial workspace layout

- `crates/guardian-core` – shared domain models and orchestration primitives.
- `crates/crypto-discovery` – discovery and inventory logic.
- `crates/quantum-risk` – risk scoring and classification.
- `crates/pqc-intelligence` – PQC algorithm intelligence and standards metadata.
- `crates/migration-architect` – migration planning and recommendation artifacts.

## Rust workspace

The repository is configured as a Cargo workspace and is designed to support the first five MVP modules from the project architecture.

## Next steps

1. Add API boundaries and service contracts.
2. Add database models and persistence adapters.
3. Integrate PQC algorithm registry and security policies.
4. Expand to dashboard and orchestration services.

## Environment setup

Copy `.env.example` to `.env` and fill in environment-specific settings before running services.
