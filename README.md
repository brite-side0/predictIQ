# predictIQ

predictIQ is a predictive analytics platform that ingests operational and market data, runs configurable forecasting and scoring pipelines, and exposes results through a unified API and web dashboard. It is designed to be deployed as a set of independently scalable services, with a shared contract layer that keeps producers and consumers of predictions in sync. The project targets teams that need reproducible, auditable predictions without building their own modeling infrastructure from scratch.

## Architecture

At a high level, predictIQ is composed of stateless API and worker services that communicate over a message bus, backed by a relational store for metadata and an object store for model artifacts and datasets. The frontend is a single-page application that talks exclusively to the public API. Contracts (schemas, event definitions, and generated clients) live in a dedicated package so that services and the frontend evolve together.

For a deeper dive, see:

- [docs/architecture.md](docs/architecture.md) — component breakdown, service responsibilities, and deployment topology.
- [docs/data-flow.md](docs/data-flow.md) — how data moves from ingestion through training, inference, and delivery.

## Repository layout

```
.
├── services/         # Backend services (API, workers, schedulers)
├── contracts/        # Shared schemas, event definitions, generated clients
├── frontend/         # Web dashboard (single-page application)
├── infrastructure/   # IaC, deployment manifests, and environment configs
└── docs/             # Architecture, data flow, deployment, and runbooks
```

- **`services/`** — Independently deployable backend services. Each service owns its own entrypoint, configuration, and tests.
- **`contracts/`** — The source of truth for interfaces between services and the frontend. Changes here should be reviewed carefully, as they can affect multiple consumers.
- **`frontend/`** — The user-facing dashboard. Consumes the public API and generated clients from `contracts/`.
- **`infrastructure/`** — Infrastructure as code, deployment manifests, and environment-specific configuration.
- **`docs/`** — Long-form documentation, including architecture, data flow, and deployment guides.

## Local development

For setup instructions, prerequisites, and common workflows, see [CONTRIBUTING.md](CONTRIBUTING.md). It covers cloning the repository, installing dependencies, running services locally, and executing the test suites.

## Further reading

- [API_SPEC.md](API_SPEC.md) — Public API specification.
- [docs/deployment.md](docs/deployment.md) — Deployment guide and environment configuration.
- [SECURITY.md](SECURITY.md) — Security policy and vulnerability reporting.

## Handsoff notes

<!-- Preserve existing content below this line. -->