# PredictIQ Documentation

Welcome to the PredictIQ documentation! This directory contains comprehensive guides, references, and resources for developers, users, and contributors.

## 📚 Documentation Structure

```
docs/
├── README.md                    # This file
├── API_SPEC.md                  # API reference and integration guide
├── CONTRACT_ERRORS.md           # Smart contract error reference
├── DISTRIBUTED_TRACING.md       # Distributed tracing setup and usage
├── api-versioning.md            # API versioning policy and guidelines
├── architecture.md              # System architecture overview
├── data-flow.md                 # Data flow documentation
├── deployment.md                # Deployment guide
├── secrets.md                   # Secrets management guide
├── pr/                          # Pull request working notes (ephemeral)
└── runbooks/                    # Operational runbooks
```

## 🚀 Getting Started

### New to PredictIQ?

1. **[Project README](../README.md)** - Start here for project overview
2. **[API Specification](./API_SPEC.md)** - API reference and integration guide
3. **[Changelog](../CHANGELOG.md)** - Release history and notable changes

### Want to Contribute?

1. **[Contributing Guide](../CONTRIBUTING.md)** - Setup, branch naming, commit conventions, and PR process
2. **[API Specification](./API_SPEC.md)** - API reference and integration guide
3. **[Infrastructure README](../infrastructure/README.md)** - Infrastructure and deployment overview

## 📖 Documentation Categories

### Architecture & Design

- **[Architecture Overview](./architecture.md)** - System architecture and components
- **[Data Flow](./data-flow.md)** - Data flow through the system
- **[API Versioning](./api-versioning.md)** - API versioning policy and guidelines

### Dashboard Management

- **[Grafana Dashboard Provisioning](../performance/config/README.md#grafana-dashboard-provisioning)** - Version-controlled dashboard setup

### Distributed Tracing

- **[Distributed Tracing Guide](./DISTRIBUTED_TRACING.md)** - OpenTelemetry setup and trace propagation

### Gas Optimization

Learn how to optimize gas usage in PredictIQ smart contracts:

- **[Gas Benchmarks README](../contracts/predict-iq/.gas-benchmarks/README.md)** - Gas benchmark results and methodology

### Infrastructure

- **[Infrastructure README](../infrastructure/README.md)** - Terraform modules, deployment, and rollback
- **[Rollback Guide](../infrastructure/ROLLBACK.md)** - Emergency rollback procedures
- **[Deployment Guide](./deployment.md)** - Deployment procedures and configuration
- **[Secrets Management](./secrets.md)** - Secrets handling and configuration

### Operations / Runbooks

- **[Runbooks](./runbooks/)** - Operational runbooks for incident response, including:
  - [API Outage](./runbooks/api-outage.md)
  - [Redis Failure](./runbooks/redis-failure.md)
  - [Stellar RPC Unavailable](./runbooks/stellar-rpc-unavailable.md)
  - [Service Down](./runbooks/service-down.md)

### Performance

- **[SLO Guide](../performance/SLO_GUIDE.md)** - Service level objectives and error budgets

### API Service

- **[API Specification](./API_SPEC.md)** - API reference and integration guide
- **[Contract Errors](./CONTRACT_ERRORS.md)** - Smart contract error reference
- **[Database Schema](../services/api/DATABASE.md)** - PostgreSQL schema and migration guide
- **[Graceful Shutdown](../services/api/GRACEFUL_SHUTDOWN.md)** - Shutdown behaviour and configuration
- **[Tracing](../services/api/TRACING.md)** - API service tracing configuration

### Pull Request Working Notes

- **[docs/pr/](./pr/)** and **[frontend/docs/pr/](../frontend/docs/pr/)** contain PR-specific working notes (design sketches, review context, temporary investigation notes).
- **Purpose:** These are ephemeral working notes, not permanent documentation. They exist to support an in-flight or recently merged PR.
- **Retention policy:** Working notes are retained only while the associated PR is open or recently merged, and are removed once their content has been folded into permanent docs (or is no longer relevant). Do not link to them as stable references.

## 🔍 Finding Documentation

### By Role

**Developers:**
- [API Specification](./API_SPEC.md)
- [Architecture Overview](./architecture.md)
- [Database Schema](../services/api/DATABASE.md)
- [Gas Benchmarks](../contracts/predict-iq/.gas-benchmarks/README.md)
- [Distributed Tracing](./DISTRIBUTED_TRACING.md)

**Operators / DevOps:**
- [Infrastructure README](../infrastructure/README.md)
- [Rollback Guide](../infrastructure/ROLLBACK.md)
- [Deployment Guide](./deployment.md)
- [Runbooks](./runbooks/)
- [SLO Guide](../performance/SLO_GUIDE.md)

**Users:**
- [Project README](../README.md)
- [API Specification](./API_SPEC.md)

### By Topic

**Smart Contracts:**
- [Gas Benchmarks](../contracts/predict-iq/.gas-benchmarks/README.md)
- [Contract Errors](./CONTRACT_ERRORS.md)

**Observability:**
- [Distributed Tracing](./DISTRIBUTED_TRACING.md)
- [API Tracing](../services/api/TRACING.md)

**Integration:**
- [API Specification](./API_SPEC.md)
- [API Versioning](./api-versioning.md)
- [Database Schema](../services/api/DATABASE.md)

**Incident Response:**
- [Runbooks](./runbooks/)
- [Rollback Guide](../infrastructure/ROLLBACK.md)

## 🤝 Contributing to Documentation

Found an error or want to improve the documentation?

1. Documentation follows the same PR process as code
2. Use clear, concise language
3. Include code examples where applicable
4. Keep documentation up to date with code changes
5. Verify all links resolve before submitting

### Documentation Standards

- Use Markdown format
- Include table of contents for long documents
- Add code examples with syntax highlighting
- Link to related documentation
- Keep line length reasonable (80-100 characters)
- Use proper heading hierarchy

## 📝 Documentation Checklist

When creating or updating documentation:

- [ ] Clear and concise writing
- [ ] Code examples tested and working
- [ ] Links verified
- [ ] Spelling and grammar checked
- [ ] Follows project style
- [ ] Includes table of contents (if long)
- [ ] Cross-references added
- [ ] Updated in CHANGELOG (if significant)

## 🔗 External Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Pyth Network](https://pyth.network/)

## 📮 Feedback

Have suggestions for improving our documentation?

- Open an issue on [GitHub](https://github.com/solutions-plug/predictIQ/issues)
- Submit a PR with improvements

---

**Last Updated:** 2026-04-26  
**Maintained By:** PredictIQ Team
