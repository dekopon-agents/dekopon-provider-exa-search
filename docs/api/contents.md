# Contents

**POST `https://api.exa.ai/contents`** — retrieve already-known URLs/document IDs, without a discovery query. Source: [Contents OpenAPI](https://exa.ai/docs/reference/get-contents.md), [coding-agent guide](https://exa.ai/docs/reference/contents-api-guide-for-coding-agents.md), [freshness guide](https://exa.ai/docs/reference/livecrawling-contents.md).

- [Complete input table](contents-inputs.md), [output envelope table](contents-output.md), [nested result fields](result-fields.md)
- [Raw request](../../schemas/upstream/contents-request.schema.json), [raw response](../../schemas/upstream/contents-response.schema.json)
- [Minimal request](../../examples/contents-minimal.json), [feature-rich request](../../examples/contents-rich.json), [synthetic mixed-success response](../../examples/contents-response.json)
- [Proposed provider input](../../schemas/provider/contents-input.schema.json)

Minimal body (no guaranteed text requested):

```json
{"urls":["https://example.org/research/orchard"]}
```

For explicit content, add `"text": true` or `"highlights": true`. Provide **exactly one** of `ids` or `urls`, 1–100 strings, each 1–2048 characters. IDs may be URL strings but are temporary vendor document identifiers, not durable provider handles. Preserve their identity rather than replacing them with title-based keys. No GET encoding, pagination or streaming exists on this route. `text`, `highlights`, `summary`, `extras` and freshness fields are **top-level** here, unlike Search's `contents` nesting.

## Extraction controls (also Search `contents`)

- `text` accepts boolean or advanced options. `maxCharacters` is 1–10000 per page. This is a character control, not a byte ceiling. `includeHtmlTags` defaults false. `verbosity` defaults `compact` (main content); `standard` adds context; `full` requests the most rendered material, sometimes identical to standard. None promises all original HTML, scripts, assets, paywalled text, or the complete original webpage.
- `includeSections` / `excludeSections` take `header` (page masthead), `navigation` (menus), `banner` (banner material), `body` (main text), `sidebar` (side material), `footer` (page footer), `metadata` (classified metadata). Classification is best-effort and may be missing. Both lists may be sent; overlap precedence is unverified, so the provider plan rejects overlap. Set `maxAgeHours: 0` to apply rendering choices on newly fetched material; cached parser output may not reflect them.
- `highlights: true` asks for default selected excerpts; `false` disables. Object `query` steers relevance. Stable `maxCharacters` caps combined highlight characters **per URL**, 1–10000. Highlights are source excerpts selected by an LLM, unlike newly generated summaries; do not assume contiguous text or full coverage.
- `summary` object: `query` guides generated content; `schema` asks for structured extraction (guide names Draft 7). Guides permit boolean but OpenAPI permits only object/null; use object, including `{}`, until resolved. Response `summary` is documented as a **string**, including when a structure was requested; do not blindly treat it as a JSON object. Parsing a JSON-encoded summary needs explicit validation and must retain raw string on failure.
- `extras` has five independent integer counts, each 0–1000/default 0: `links` URL strings, `imageLinks` image URL strings, `richImageLinks` URL/alt records, `richLinks` URL/anchor records, `codeBlocks` text/source records. Extracted links are not automatically fetched by the provider.
- `subpages` 0–100/default 0 requests linked children per parent; actual count may be smaller due to service constraints. `subpageTarget` is one 1–100-character term or array of 0–100 such terms, influencing discovery rather than forcing exact paths. It is not unlimited recursion. The wire output schema lists only base metadata for children while guides describe full nested result shapes; see [conflicts](../sources.md). Preserve unknown child fields only in bounded raw mode.

## Dynamic Highlights research preview

The actual HTTP header is **`Exa-Beta: dynamic-highlights-2026-08-28`**. The OpenAPI annotation named `x-exa-beta-flag` stores that value; it is **not** an HTTP header named `x-exa-beta-flag`.

Both `highlights.dynamic` and `highlights.verbosity` require the flag when set. `dynamic: true` considers the whole result set and distributes **one shared context budget**, not a fixed per-page allocation. `verbosity: low|medium|high` selects progressively larger vendor-tuned token budgets; exact numbers can change. Without dynamic, verbosity is per-page. Response shape remains highlights arrays; token budgets do not bound JSON bytes or promise equally sized excerpts.

Conflicts: dynamic cannot combine with `maxCharacters`; verbosity cannot combine with `maxCharacters` or deprecated `numSentences`. OpenAPI explicitly rejects missing flag, even though guide prose highlights only dynamic=true. Initial plan uses explicit caller `betaDynamicHighlights: true` **and** operator permission before emitting the one fixed public header. No arbitrary headers API. See [beta fixture](../../examples/provider-search-beta.json).

`numSentences` is deprecated; reference describes an approximate 1333-character mapping while guides tell users to remove it. It is not an effective sentence-count control in the planned provider. `highlightsPerUrl` is deprecated and ignored. Both are inventory-visible and rejected, not silently dropped.

## Freshness, timeout, provenance

| `maxAgeHours` | Vendor behavior |
|---|---|
| omitted | Prefer cache; fetch if no cached material |
| -1 | Cache only, no live crawl |
| 0 | Request new fetch; supported route to freshly applied rendering options |
| 1–720 | Accept sufficiently young cache; otherwise fetch |

`livecrawlTimeout` is an integer **>0 and ≤90000 milliseconds**, vendor default 10000; it is not an overall transport deadline. Freshness guide says failed/time-out live fetch may fall back to cache; the contents guide says 0 never uses cache. Preserve `statuses[].source` and warn that requested freshness is **not evidence of observed freshness**. No documented `crawledAt` timestamp means exact crawl age is unknown. The provider itself makes no fallback request.

Deprecated `livecrawl` values were `never` (cache), `always` (fetch), `fallback` (cache then fetch), `preferred` (prefer fetch). Migration guidance maps never→-1, always→0, fallback→omit; preferred has no exact equivalent (small positive age is only approximate). The OpenAPI warns legacy livecrawl may follow server freshness policy, not newly parsed content. Never send it with `maxAgeHours`; planned provider rejects legacy livecrawl outright.

## Per-URL outcomes

Inspect `statuses` even on HTTP 200. Each record requires `id` and `status` (`success`/`error`); `source` optionally says `cached`/`crawled`. `error` is optional/null or `{tag, httpStatusCode}`, whose status code may be null. Match by ID, not parallel array position. Missing status/row is unknown, not success; a whole request can also fail. The raw ContentsResponse schema does not require `statuses` or even `results`; robust implementation must tolerate omissions while reporting incompleteness.

| Per-URL tag | Meaning |
|---|---|
| `CRAWL_NOT_FOUND` | Target not found (typically 404) |
| `CRAWL_TIMEOUT` | Target fetch timed out (504) |
| `CRAWL_LIVECRAWL_TIMEOUT` | Requested live-crawl deadline exhausted (504) |
| `SOURCE_NOT_AVAILABLE` | Forbidden/unavailable source (403) |
| `UNSUPPORTED_URL` | Unsupported URL scheme |
| `CRAWL_UNKNOWN_ERROR` | Other upstream crawl failure (500+) |

Preserve these and unknown future tags, including bounded diagnostics, without automatically retrying or crawling excluded links. [Transport errors](transport.md) are distinct from per-URL status.

## Enterprise compliance

The [targeted HIPAA page](https://exa.ai/docs/reference/security/hipaa.md) requires Enterprise enablement/BAA; unavailable teams get 403 `FEATURE_DISABLED`. Mode is top-level `compliance: hipaa` only on Search/Contents; other routes reject it. It is **operator-only**, excluded from model input schemas.

HIPAA supports cached text/highlights, **no summary and no live-fetch freshness**. Search requires explicit `instant` or `fast`; omitted/auto/deep types fail. Source says omit freshness fields, or set `maxAgeHours: -1` on Contents. The conservative planned Search HIPAA profile omits all contents freshness fields (do not generalize Contents -1 allowance); Contents profile may use -1. Operator profile rejects incompatible caller options rather than converting them silently. Vendor says these enabled requests include Zero Data Retention; this repository does not certify compliance or enable accounts.
