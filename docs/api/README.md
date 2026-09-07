# Endpoint index

Base URL: `https://api.exa.ai`. JSON POST bodies, not GET query parameters. See [transport](transport.md).

| Family | Method and exact URL | Choose when | Initial provider plan |
|---|---|---|---|
| [Search](search.md) | POST `https://api.exa.ai/search` | Discover ranked sources and optionally retrieve content or synthesis | `search` capability / subcommand |
| [Contents](contents.md) | POST `https://api.exa.ai/contents` | Retrieve known document IDs or URLs without a new query | `contents` capability / subcommand |
| [Code Context](context.md) | POST `https://api.exa.ai/context` | Obtain assembled code snippets and documentation context | `context` capability / subcommand |
| [Answer](answer.md) | POST `https://api.exa.ai/answer` | Generate an answer grounded in search citations | Adjacent synthesis, deferred |
| Find Similar | **No current endpoint verified** | Similarity-by-URL discovery | Absent from fetched index; targeted reference returned 404. Do not resurrect `/findSimilar` from remembered SDK methods. |

## Excluded / deferred families

Presence below is discovery from the [official index](https://exa.ai/docs/llms.txt), not endpoint-level research or implementation permission.

| Family | Why not ordinary synchronous retrieval |
|---|---|
| Agent / Connect | Asynchronous multi-step runs, cancellation/deletion, enrichment and third-party data access; distinct lifecycle, spend and effects |
| Research | Legacy/research-workflow family not independently verified as a current standalone API in this snapshot; do not alias a remembered Research API to Agent or synchronous `type: deep` |
| Websets | Persistent collections, searches, imports, enrichment, items, webhooks, events and scheduled monitors; stateful workflows |
| Monitors (standalone and Websets) | Recurring searches, schedules, webhook delivery, run history and lifecycle changes |
| Batch | Async submission, retrieval, cancellation and deletion; polling/lifecycle not this provider's one-call contract |
| Team / API-key management and billing | Account administration, permissions, credentials, budgets and usage; never a model retrieval capability |
| Answer streaming / Search streaming | SSE is documented, but current HTTP guest responses buffer; initial plan rejects streaming, not simulated token delivery |
| x402 / MPP / payment integrations | Alternative payment protocols and financial authorization, outside broker Bearer retrieval |
| SDK, MCP, OpenAI-compatibility surfaces | Convenience/alternate integrations are not evidence that arbitrary SDK fields are accepted by these REST routes |

`deep-lite`, `deep` and `deep-reasoning` remain **in-scope Search types** despite involving vendor-side reasoning. A POST search consumes billable compute and may trigger a crawl, but does not create the persistent user-managed workflows listed above. No automatic pagination, retry, fallback call, polling or subsequent content request is planned.
