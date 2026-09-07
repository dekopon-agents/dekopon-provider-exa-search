# Transport, authentication and errors

Sources: [Search OpenAPI](https://exa.ai/docs/reference/search.md), [Contents OpenAPI](https://exa.ai/docs/reference/get-contents.md), [Answer OpenAPI](https://exa.ai/docs/reference/answer.md), [Context examples](https://exa.ai/docs/reference/context.md), [error guide](https://exa.ai/docs/reference/error-codes.md).

## Exact wire contract

All four indexed operations use **POST JSON** to fixed paths on `https://api.exa.ai`. There are no documented GET query/path parameters in scope. JSON booleans are `true`/`false`, arrays are JSON arrays, numeric controls are numbers, and explicit `null` is only structurally accepted where shown in the tables. Do not percent-encode JSON body strings, flatten arrays into comma-separated strings, send Python snake_case, or copy SDK-only convenience options. A JSON serializer correctly escapes quotes, newlines and Unicode. Native HTTP library URI construction is only for the fixed base/path; a retrieved URL remains a JSON string sent to Exa, not a new guest HTTP destination.

| Request header | Required / meaning | Plan |
|---|---|---|
| `Content-Type: application/json` | JSON POST body media type | Fixed guest header, no arbitrary value |
| `Authorization: Bearer <key>` | Documented alternative to x-api-key | **Existing broker Bearer sink**; no key material in guest/model/environment/examples |
| `x-api-key: <key>` | Alternative API-key scheme, not mandatory | Not selected; no arbitrary secret-header mechanism needed |
| `Exa-Beta: dynamic-highlights-2026-08-28` | Required when setting Dynamic Highlights or highlight verbosity on Search/Contents | Fixed non-secret header, explicit feature gate only |

The embedded OpenAPI declares two security objects (`apiKey: []` and `bearer: []`), meaning **OR**, not AND. `apiKey` is `type: apiKey`, `in: header`, name `x-api-key`; `bearer` is `type: http`, scheme `bearer` (no bearerFormat is declared). The existing Authorization Bearer credential sink satisfies Exa authentication. The literal notation `<key>` above documents syntax only; no copy-paste shell credential environment pattern is needed here. Auth is omitted from JSON fixtures and excluded from non-auth parameter coverage counts.

The beta **annotation** `x-exa-beta-flag` in schemas is not the header name. [Header fixtures](../../examples/headers-beta.json) contain only public fixed metadata. No caller-provided headers, arbitrary hostnames or transport settings are exposed to the model. No x402/MPP payment fallback.

## Response headers and envelope

Search/Contents/Answer OpenAPI document `x-request-id` (matches body requestId when present), `x-exa-queued` (string `"true"`/`"false"`, whether queued), and `x-exa-queue-ms` (string milliseconds). Preserve bounded diagnostics; no claim of these headers on Context absent evidence. A missing header does not imply no queueing. Native response ceilings and total deadlines must cover queue time, body buffering, retrieval and synthesis.

Accept expected JSON only for initial provider calls; reject HTML/WAF/error pages as bounded transport diagnostics. Unexpected `text/event-stream` is an unsupported media-type result, not parseable JSON and not permission to buffer an unbounded stream. Response-header diagnostics must never echo auth. There is no documented arbitrary client request-ID header.

## Failure handling

| HTTP status | Interpretation / caller action (no automatic retry) |
|---|---|
| 400 | Bad shape/conflicting fields, URL/count limits, unsupported category filters; correct input |
| 401 | Missing/invalid authorization; operator resolves credentials |
| 402 | Exhausted credit or key/team budget; operator resolves billing; do not initiate payment |
| 403 | Disabled feature, permissions, robots/policy/moderation, or regional/WAF block |
| 404 | Unavailable resource/URL; route-specific behavior may differ |
| 422 | Well-formed but unprocessable request/content |
| 429 | Rate limit; return bounded message, caller/operator decides future action |
| 500/502/503 | Service/upstream failure; preserve diagnostics, no automatic repeat charge |
| 501 | Answer unable to synthesize given available evidence |

The error guide also lists 409 for stateful duplicate resources; not an ordinary Search/Contents success condition or planned workflow. Reference endpoint OpenAPI response lists are narrower than the generic guide, so this table is conservative handling, not proof every endpoint emits every status.

Typical error envelope fields are `requestId`, `error` and `tag`, but 429 examples can contain **only `error`**. Preserve unknown tags; do not require the full canonical envelope to report failure. Relevant request tags: `INVALID_REQUEST_BODY`, `INVALID_REQUEST`, `INVALID_URLS`, `INVALID_NUM_RESULTS` (highlights result ceiling), `NUM_RESULTS_EXCEEDED` (plan), `INVALID_FLAGS`, `INVALID_JSON_SCHEMA`, `NO_CONTENT_FOUND`. Auth/spend tags: `INVALID_API_KEY`, `NO_MORE_CREDITS`, `API_KEY_BUDGET_EXCEEDED`, `TEAM_BUDGET_EXCEEDED`, `ACCESS_DENIED`, `FEATURE_DISABLED`. Content/safety tags: `ROBOTS_FILTER_FAILED`, `CONTENT_FILTER_ERROR`, `PROHIBITED_CONTENT`; processing: `FETCH_DOCUMENT_ERROR`, `UNABLE_TO_GENERATE_RESPONSE`, `DEFAULT_ERROR`, `INTERNAL_ERROR`.

A 200 Contents response can contain individual failures; [inspect statuses](contents.md#per-url-outcomes). Search lacks the Contents per-URL status contract. Never convert missing content, missing costs, parser failure or an empty response into success. No retry, auto-pagination, alternative endpoint, live-fetch fallback call, or follow-up URL fetch is performed by the planned provider. Vendor-internal crawl fallback remains a separately documented upstream behavior.
