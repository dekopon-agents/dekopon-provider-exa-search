# Proposed Exa provider implementation plan

**Design only; no implementation, installable release, Cargo scaffold, WIT change or core change is included.** All defaults/limits in this document are proposals for later review, not existing behavior.

## Compatibility baseline and invariants

Public Dekopon baseline: commit [`542430ed349909eb7cef4269e7e52037e7126efe`](https://github.com/dekopon-agents/dekopon/tree/542430ed349909eb7cef4269e7e52037e7126efe), workspace 0.12.0, provider WIT 0.3.0, HTTP WIT 1.0.0. [HTTP request binding](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/crates/dekopon-provider-http/src/lib.rs#L65-L129) supports `Request::new(method, uri)`, public headers, byte body and `send(Request)` for ordinary JSON **POST as well as GET**, returning buffered responses. No new transport architecture is necessary for synchronous Exa retrieval.

[Broker credential sinks](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/crates/dekopon-brokerd/src/credentials.rs#L47-L67) and [HTTP host authorization](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/crates/dekopon-http-host/src/lib.rs#L810-L842) already support Authorization Bearer. Exa documents this scheme. Credentials stay in the broker, never guest/model/env, logs or examples. No proxy, arbitrary secret-header workaround or core patch is part of this plan. Read public [development](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/docs/development.md) and [secrets](https://github.com/dekopon-agents/dekopon/blob/542430ed349909eb7cef4269e7e52037e7126efe/docs/secrets.md) guidance before implementing.

`Provider::invoke` consumes `serde_json::Value`; future guest code must parse/validate typed input itself. Provider manifest schema is model-facing metadata, **not general host JSON Schema enforcement**. Optional provider-cli/run-command allows idiomatic operation subcommands; there is no implemented CLI here. No imaginary streaming/blob/time/secret/handle API is assumed.

## Capabilities and namespaces

| Capability / planned CLI subcommand | Native route | Intended use |
|---|---|---|
| `search` | POST /search | Source discovery, optional extraction and synchronous synthesis |
| `contents` | POST /contents | Known-URL/ID retrieval and per-URL outcomes |
| `context` | POST /context | Assembled code/documentation context |

The operation chooses a fixed host/path/method. CLI native options use one unambiguous spelling per documented JSON field (a typed parser can map idiomatic kebab-case switches to camelCase); repeated flags produce arrays, flags parse booleans, structured `outputSchema`/summary schema use validated JSON input. Do not create a generic URL/body/headers tool. No Answer command initially.

Each input has:

- `native`: the typed upstream request, preserving current stable native controls and bounds. This namespace is not an arbitrary JSON pass-through: closed top-level request validation, unknown-field rejection and explicit nested policy checks are required. Free-form maps only exist where vendor schema controls intentionally accept JSON Schema/JSON values.
- `output`: independent `mode`, `maxResults`, `maxBytes`, `maxCharactersPerResult` projection settings. These never become Exa body fields and never silently overwrite a native extraction choice.
- `betaDynamicHighlights`: caller opt-in for Search/Contents only. Requires operator permission and valid conflict checks; emits only fixed `Exa-Beta: dynamic-highlights-2026-08-28`. No key material or user-specified headers.

The [coverage inventory](coverage.json) gives each documented field a supported/beta/deprecated/operator-only/deferred disposition, with schema pointers and documentation. “Supported” means **planned**, not implemented or live-tested. Optional native nulls are accepted where sourced but canonicalized to omission before dispatch, with a warning when supplied; explicit false remains false. Defaults are applied after normalization. Deprecated inputs fail with migration guidance rather than being accepted as effective or silently dropped. Unknown category hints are explicitly deferred pending the enum/prose conflict; the six listed categories remain available.

## Native defaults versus provider/operator limits

| Dimension | Vendor contract | Proposed provider default | Proposed operator ceiling / policy |
|---|---|---|---|
| Search type | auto | Leave absent: vendor auto | Permit all six types; no silent mode fallback |
| Search result count | 10, structural 1–100; entitlement conflict above 25 | Leave absent: 10 | 25 until operator confirms entitlement; allow up to sourced 100 after confirmation |
| Contents input list | 1–100 IDs or URLs, exactly one list | Caller supplies list | 25 URLs/IDs initially, configurable to sourced 100 |
| Context tokens | dynamic or 50–100000 | If omitted, set 5000 explicitly and report applied native default | 10000 initially, configurable up to sourced 100000; dynamic allowed only with operator opt-in because budget is vendor-selected |
| Text/highlight characters | 1–10000 when specified | Do not override explicit native values; omitted extraction untouched | Enforce sourced 10000; operator may lower, reject explicit excess |
| Subpages | 0–100 per parent, best-effort | Leave absent: 0 | 2 per parent initially; configurable to sourced 100 within HTTP limits |
| Extras counts | Each 0–1000 | Leave absent: 0 | 10 per artifact kind initially; configurable to sourced 1000 |
| Output mode | Not an upstream knob | brief for Search/Contents, relevant for Context | raw and other supported modes remain selectable |
| Output result count | Not an upstream knob | 10 | 100, constrained independently by bytes |
| Output chars per result | Not an upstream knob | 2000 | 10000 (applies to projected text/excerpts, not raw mode) |
| Exact serialized provider output | Not a character/token limit | 32768 UTF-8 bytes | 262144 bytes; caller may lower to ≥1024 |
| Native HTTP response body | Buffered by binding | Operator configures 2 MiB ceiling before dispatch | Hard host-enforced ceiling; abort rather than buffer arbitrarily |
| Native request body | JSON includes user schemas | Serialize and check ≤65536 bytes | 65536 bytes initially, reject excess |
| Native HTTP deadline | Not livecrawlTimeout | Operator sets 60 seconds | Includes queue/network/body; reject unsupported host configuration rather than claim guest enforcement |

These ceilings are **independent** and proposed, not public entitlement claims. Lower operator limits must return clear validation errors for explicit excess, not silently delete native knobs. An omitted native option may retain vendor defaults only if policy permits them. Record actually applied defaults and the requested type; never infer type from deprecated `resolvedSearchType`. Native host response/deadline configuration must be validated against real documented configuration before runtime work is accepted; this plan deliberately does not invent configuration key names or guest setters. If the available native ceiling cannot be established, stop the implementation gate rather than substituting post-buffer truncation.

No automatic content is enabled merely because `output.mode` is relevant/page-text. If corresponding native text/highlights were not requested or absent, return links/available fields and a missing-content warning, not a hidden second request. Context supports relevant/page-text/raw only. All native options remain discoverable even when operator policy rejects a specific value.

## Output modes and deterministic size handling

Envelope [subset schema](../schemas/provider/output.schema.json): `operation`, `mode`, `data`, `meta`. Metadata includes observed requestId, optional estimated costDollars, upstream/returned counts when known, `truncated`, `truncationReasons` and warnings. Missing request IDs/costs/counts remain absent/null, never invented. Exact output cap includes metadata, JSON syntax, escaped strings and UTF-8 encoding, not just data text.

| Mode | Data retained |
|---|---|
| links | Ordered title/URL/id rows; no content payload; retain available provenance and per-URL failure summary |
| brief | Links plus publication/author provenance and already-requested page summary, bounded by projection character cap; no hidden generation |
| relevant | Links plus already-requested highlights (and their supplied scores), or assembled Context string; retain synthesis and grounding as an indivisible unit when present and fitting |
| page-text | Links plus already-requested rendered text (or Context string), subject to character/byte truncation; never “full original webpage” |
| raw | Parsed upstream object with unknown fields preserved, bounded by native and exact provider ceilings; no promise byte-for-byte original HTTP formatting |

Projected modes first cap selected result count and content characters at Unicode character boundaries. Preserve URL/title identity pairs. Then serialize the **whole envelope** to measure actual bytes. If oversized, remove lowest-priority trailing rows or trailing complete text segments, updating truncation metadata and reserializing until it fits. Drop generated synthesis **with its grounding together**, never leave uncited content or detached citations. Per-URL failures and costs have priority over successful content; if diagnostics themselves cannot fit, return a small explicit size-limit error rather than silently hiding failures. Do not slice serialized JSON, URLs, escape sequences or UTF-8 bytes into invalid output.

Raw mode preserves unknown upstream fields on retained rows. It may remove only complete trailing top-level `results` rows while marking truncation; it does not secretly clip arbitrary nested values. Preserve the full `statuses`, costs, requestId and synthesis/grounding if present. If the remaining raw envelope (including a single large row, entity object, Context string, synthesis or failure list) cannot fit, return an explicit `output_limit` error with bounded identity metadata—never claim full raw access. Users can lower native retrieval controls or choose a projected mode on a subsequent explicit call. No blob stores, continuation handles or offloaded artifacts.

## Validation and safety before dispatch

Future guest validation must enforce types/ranges, closed request fields, required query/list constraints; deep-only additionalQueries; category forbidden filters; ISO country validity and date ordering; disjoint section/domain inclusion/exclusion; generated-schema complexity (Search guide depth 2/properties 10); beta header gate/conflicts; deprecated rejection; and all operator ceilings. Unsupported combinations must return a local error **before** an API request. Offline schemas cover only the subset listed in [validation](validation.md); they are not proof these runtime checks exist.

HIPAA remains a separately operator-selected profile after vendor enablement. The model cannot set `native.compliance`; eligible Search uses explicit instant/fast and omits freshness; eligible Contents uses cache-only, no summaries, and may set -1. Reject incompatible native values; do not turn regular requests into regulated processing silently. No account changes or compliance certification occur here.

URLs/snippets/entity data are untrusted; do not execute code or instructions, render unsanitized HTML, or follow links automatically. A read-only retrieval using POST can spend credits and trigger a vendor crawl but is not intrinsically a persistent external write. Classify effects from endpoint semantics, not HTTP method alone. Never log broker secrets, auth headers, or unrestricted bodies. Error output is bounded and preserves unknown status/tags without retries.

## Independently verifiable future milestones (not authorized runtime work)

1. **Contract review** — owner: provider maintainer; dependency: these docs. Verify sources/coverage, resolve access/shape conflicts needed for selected features, agree operator ceilings. Stop if missing facts are necessary to claim support.
2. **Typed adapter** — owner: one provider writer; dependency: contract approval. Implement only fixed operations and guest-side validation using existing WIT. Offline native-fixture tests must prove no request for invalid inputs and correct JSON/headers/projection. Fresh review before next milestone; no core changes.
3. **Real boundary acceptance** — owner: maintainer with separately authorized test credentials; dependency: adapter review. Validate actual broker Bearer injection, destination allowlist, native byte/deadline enforcement, buffered response behavior, UTF-8 exact output caps, mixed Contents failure and missing metadata. Component mocks alone cannot establish these contracts. No permission for such calls is granted by this repository.
4. **Release decision** — owner: maintainers; dependency: exact-head checks and fresh review. Recheck vendor terms/access/prices and schema drift; then separately authorize packaging/publication. No builds, release workflow or install instructions now.

No broker-wide accounting, handle system, cache subsystem or streaming mechanism is required as a speculative prerequisite. Missing validation and unresolved material review findings block advancement.
