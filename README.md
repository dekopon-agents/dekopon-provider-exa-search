# Exa Search provider for Dekopon

**Documentation and planning only. There is no installable provider, release, runtime implementation, or supported CLI in this repository yet.**

This public design reference covers Exa's synchronous search, URL contents, and code-context APIs. Research snapshot: **2026-09-07**. No authenticated API calls were made. Every example response is synthetic; prices, access and wire shapes require rechecking before implementation.

## Start here

- [Endpoint index and deferred families](docs/api/README.md)
- [Search](docs/api/search.md), [Contents](docs/api/contents.md), [Code Context](docs/api/context.md)
- [Adjacent Answer synthesis](docs/api/answer.md)
- [Transport, authentication and errors](docs/api/transport.md)
- [Shared output semantics](docs/api/outputs.md)
- [Provider implementation plan](docs/provider-plan.md)
- [Dated primary sources and material conflicts](docs/sources.md)
- [Machine-readable parameter coverage](docs/coverage.json)
- [Schema boundaries](schemas/README.md) and [synthetic fixtures](examples/README.md)
- [Offline validation](docs/validation.md)

The proposed provider uses typed per-operation inputs, broker-held Bearer credentials and bounded outputs. Upstream retrieval options are independent of output projection. Nothing here authorizes changes to Dekopon core, account configuration, publication, or API access.

## Documentation checks

```sh
python3 -m venv .venv
.venv/bin/python -m pip install -r requirements-dev.txt
.venv/bin/python scripts/validate_docs.py
actionlint
git diff --check
```

Validation is offline after dependency installation. CI needs no vendor connection, API key, Rust toolchain or secret store.
