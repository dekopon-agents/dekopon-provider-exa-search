# Search

**POST `https://api.exa.ai/search`** — discover sources; request page extraction in the same call. Existing Bearer authentication works; see [transport](transport.md). Primary references: [Search OpenAPI](https://exa.ai/docs/reference/search.md), [coding-agent guide](https://exa.ai/docs/reference/search-api-guide-for-coding-agents.md), [best practices](https://exa.ai/docs/reference/search-best-practices.md).

- [Complete input table](search-inputs.md), [envelope output table](search-output.md), [nested result table](result-fields.md)
- [Raw request schema](../../schemas/upstream/search-request.schema.json), [raw response schema](../../schemas/upstream/search-response.schema.json)
- [Minimal request](../../examples/search-minimal.json), [feature-rich request](../../examples/search-rich.json), [synthetic response](../../examples/search-response.json)
- [Proposed input](../../schemas/provider/search-input.schema.json), [beta example](../../examples/provider-search-beta.json)

Minimal JSON body:

```json
{"query":"synthetic orchard irrigation research"}
```

## Mode, query, ranking and category

| `type` | Meaning | Dated indicative latency, not deadline |
|---|---|---|
| `auto` (vendor default) | Balanced quality and speed; normal starting point | ~1 second in coding guide |
| `fast` | Lower-latency quality search for interactive use | ~450 ms |
| `instant` | Minimum latency, less search depth, chat/voice/autocomplete | ~250 ms |
| `deep-lite` | Lightweight research and synthesized results | ~4 seconds |
| `deep` | Comprehensive multi-step retrieval and synthesis | ~4–15 seconds |
| `deep-reasoning` | Stronger reasoning for complex decisions | ~12–40 seconds |

These are retrieval modes, not models whose scores are interchangeable. `neural` and `keyword` are **not current request enum values** even though legacy response/cost fields mention them. There is no documented ranking-weight or sort parameter. Write a descriptive `query`; `additionalQueries` adds 1–10 variations only with a deep-family type. `systemPrompt` guides synthesis and deep search planning; it is not an exact domain/date filter.

| `category` | Native intent and differences |
|---|---|
| `company` | Organization pages and structured `type: company` entities; industry, funding, headcount and geography can be expressed in query, not invented REST numeric filters |
| `people` | Professional profiles and `type: person` entities; role, skill, seniority, employer and location go in natural language |
| `publication` | Scholarly papers, preprints and journals; `type: publication` metadata includes authors, citations, DOI and venue-related research metadata |
| `news` | Reporting; combine publication-date bounds with suitable freshness, which controls fetched content rather than news ranking |
| `personal site` | Individual-owned/personal websites |
| `financial report` | Financial reporting and filings |

People/company vertical pages describe weekly refreshed indexes; that is not a per-row freshness SLA. Both reject `startPublishedDate`, `endPublishedDate`, and `excludeDomains` with 400. The schema does **not** forbid `includeDomains` for these categories; other category/type entitlement combinations are unverified, not fabricated restrictions. `userLocation` is an ISO country hint, not a person's residence constraint. OpenAPI prose says other category strings become hints, but its enum rejects them: proposed strict input initially keeps the six enumerated values; other hints explicitly await upstream clarification.

The news guide's sample omits `category`, whereas the reference documents `news`; omission remains generic search. The code vertical recommends `type: fast` plus highlights on `/search`, without a `code` category. Choose [/context](context.md) instead when assembled snippets rather than separate source rows are wanted.

## Constraints and migration traps

- `numResults`: vendor default 10, schema range 1–100; pricing advertises enterprise access above 25. Proposed operator entitlement ceiling starts at 25, not a claim that the REST maximum is 25. No pagination field or continuation token is documented. Do not silently clamp an explicit request.
- Domains: up to 1200 entries each, supporting hostname, path-prefix and wildcard subdomain matching. Use JSON string arrays, not `includeUrls`/`excludeUrls` or redundant `site:` syntax. Include/exclude overlap precedence is undocumented; initial provider rejects overlap rather than claiming a vendor rule.
- Dates: ISO 8601 publication boundaries concern estimated publication timestamps. Provider should reject start after end; this is application validation, not encoded by JSON Schema. `startCrawlDate`/`endCrawlDate` are deprecated and **ignored**; they are not freshness options.
- `includeText`, `excludeText`, arbitrary ranking knobs, and standalone `text`/`summary` are absent from the current REST inventory. No claim of their support. `useAutoprompt` is guide-documented deprecated/no-op, absent from OpenAPI; reject it. SDK snake_case is not wire spelling.
- Content extraction fields must be under `contents`. [Contents controls](contents.md) apply identically there, including nested text/extras/summary and beta conflicts.
- Top-level and nested legacy `context` are deprecated; these are not the separate code-context operation.
- `compliance: hipaa` is operator-only enterprise mode. See [eligibility](contents.md#enterprise-compliance); new model inputs cannot enable it.

## Generated output, not extracted page text

`outputSchema` accepts `type: text` with optional description, or `type: object` with description, properties, required and additionalProperties. User-defined schema maps are open JSON; their supported semantic subset is narrower than their structural shape. The coding guide documents **nesting depth 2, total properties 10**. Count before dispatch in the future provider; offline schema fixtures do not enforce recursive schema-complexity or generation quality. Do not ask for citation fields in this schema: Exa returns `output.grounding` separately.

Current OpenAPI says synthesis works with **every search type**, adding about two seconds. A guide response example still labels it “deep search only”; do not copy that restriction. `output.content` is string/object; grounding associates field paths with URL/title citations and low/medium/high model confidence. These are generated claims, not verbatim source excerpts or calibrated probabilities. Preserve citations with their fields during projection.

`stream: true` only selects SSE when `outputSchema` is present according to OpenAPI; the coding guide overgeneralizes streaming. SSE uses chat-completion-shaped chunks (`choices[].delta.content`, terminal finish data). The [structural definitions](../../schemas/upstream/documented-shapes.json) retain `SearchStreamChunk` as documentation, but initial provider rejects `true` unconditionally. False/null is omitted on dispatch. Buffered HTTP is not streaming.

[Output semantics](outputs.md) cover optional fields, costs and non-comparable scores; [transport](transport.md) covers request-level failures. A missing content field on a search result is not proof of a successful full extraction, and `/contents`-style per-URL status tags are not documented for search.
