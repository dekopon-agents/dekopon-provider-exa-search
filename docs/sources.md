# Sources, access and material conflicts

Research date: **2026-09-07 UTC**. Official public pages only. No authenticated search/retrieval requests, account inspection or live API response collection. All repository examples are synthetic, not captured results.

## Bounded collection

Fetched the official [llms.txt index](https://exa.ai/docs/llms.txt) first, then **18 substantive page attempts** (17 HTTP 200, one HTTP 404). Used **one additional allowed targeted missing-fact page**, HIPAA, to resolve the newly discovered compliance parameter restrictions (HTTP 200). Total: **19 substantive attempts, 18 HTTP 200, 1 HTTP 404**, plus the index. Each request bounded to 30 seconds and 2 MiB; no crawler, retries or bypass. Untracked source scratch is ignored and not part of this repository. HTTP 200 is access evidence, not proof a page's substantive content was successfully interpreted.

| Official URL | Outcome | Use |
|---|---|---|
| https://exa.ai/docs/reference/search.md | HTTP 200 | Search parameter/response/security OpenAPI |
| https://exa.ai/docs/reference/get-contents.md | HTTP 200 | Contents parameter/response/security OpenAPI |
| https://exa.ai/docs/reference/context.md | HTTP 200 | Exact Context method/path, parameters and response examples |
| https://exa.ai/docs/reference/answer.md | HTTP 200 | Adjacent Answer schema, security and synthesis |
| https://exa.ai/docs/reference/search-api-guide-for-coding-agents.md | HTTP 200 | Search controls, restrictions, schema limits, beta, migration |
| https://exa.ai/docs/reference/contents-api-guide-for-coding-agents.md | HTTP 200 | Contents control nuances, beta, failures |
| https://exa.ai/docs/reference/verticals/people.md | HTTP 200 | People metadata/query differences |
| https://exa.ai/docs/reference/verticals/company.md | HTTP 200 | Company metadata/query differences |
| https://exa.ai/docs/reference/verticals/news.md | HTTP 200 | News/generic example difference |
| https://exa.ai/docs/reference/verticals/code.md | HTTP 200 | Code Search versus Context |
| https://exa.ai/docs/reference/livecrawling-contents.md | HTTP 200 | Freshness/cache/live-fetch guidance |
| https://exa.ai/docs/reference/pricing.md | HTTP 200 | Dated rates and plan entitlement conflict |
| https://exa.ai/docs/reference/rate-limits.md | HTTP 200 | Default QPS ceilings |
| https://exa.ai/docs/reference/error-codes.md | HTTP 200 | HTTP and per-URL errors |
| https://exa.ai/terms | HTTP 200 | Terms endpoint returned PDF; substantive terms UNVERIFIED |
| https://exa.ai/docs/reference/search-best-practices.md | HTTP 200 | Search query/filter best practices |
| https://exa.ai/docs/reference/find-similar.md | HTTP 404 | Find Similar not currently verified; do not resurrect legacy SDK operation |
| https://exa.ai/docs/reference/security.md | HTTP 200 | Enterprise security, regional restrictions |
| https://exa.ai/docs/reference/security/hipaa.md | HTTP 200; targeted additional page | Enterprise enablement and exact cache-only eligibility |

The terms response was a PDF (magic `%PDF-1.3`), not text/HTML. A helper UTF-8 read failed; one bounded standard-library decoding attempt did not yield text. No further fetch or parser was used. **No substantive terms interpretation, redistribution license or training/use permission has been verified.** Consult the official terms before implementation or release; this research is not legal advice. Technical schema names/shapes here are adapted, vendor explanatory prose is paraphrased, and no raw page dumps are tracked.

## Source precedence and conflicts

OpenAPI structural shapes are preserved separately from planned input policy. When guide prose conflicts, record both; do not claim live verification or silently widen REST support.

| Material discrepancy | Evidence | Documentation / planned treatment |
|---|---|---|
| Public max 100 vs requests above 25 enterprise | Search OpenAPI/guide 1–100; pricing Enterprise advertises above 25 and up to 1000; error guide NUM_RESULTS_EXCEEDED | Wire schema 100; initial operator entitlement ceiling 25 until confirmed. 1000 not treated as current public wire schema support |
| `summary` boolean vs object | Coding guides allow true; OpenAPI only object/null | Use object; boolean explicitly deferred in coverage |
| Text lengths above 10000 in guides | Search coding guide example 15000 and other guide examples larger; OpenAPI text/highlights/context caps 10000 | Structural and planned cap 10000, never copy larger examples as valid |
| Custom category strings | Search prose accepts hints; schema enum six values | Expose enumerated categories; non-enumerated hints deferred pending clarification, not called invalid upstream with certainty |
| Synthesis mode restriction | Search OpenAPI says every type; guide output example says deep only | Every-type synchronous outputSchema documented; deep-only label treated as stale example |
| Streaming unconditional claim | Coding guide says stream=true returns SSE; OpenAPI says only with outputSchema | Document conditional upstream behavior, reject true initially under buffered host |
| Dynamic Highlights header naming | Schema metadata `x-exa-beta-flag`; descriptions specify `Exa-Beta` | Actual fixed HTTP header is Exa-Beta; both dynamic/verbosity require it, not just guide dynamic=true example |
| Deprecated highlights | OpenAPI numSentences approximately maps to 1333 chars/sentence; guides say remove; highlightsPerUrl ignored | Both migration-only/rejected; no sentence-count guarantee |
| Freshness guarantee | Freshness guide permits cache fallback after failure; contents guide says maxAgeHours=0 never cache | Preserve source status; request freshness not evidence of actual crawl freshness; no provider fallback calls |
| Dates and null | OpenAPI optional date-time string; guide optional string/null with YYYY-MM-DD | Wire schema retains date-time, robust planned parsing records null/missing as unknown; no fabricated timestamps |
| Subpages richness | OpenAPI children have base metadata; guides describe full nested results | Raw structural schema marks smaller documented shape; planned bounded raw can retain unknown fields, not guarantee recursive extraction |
| Entity examples vs required shape | Vertical examples omit schema-required research metadata | Use structural schemas as documented snapshots, not universal response validators; incomplete metadata must not erase useful URLs |
| Legacy fields | resolvedSearchType can be empty; cost keys neural/keyword persist; crawl dates ignored | No branching on resolvedSearchType, no resurrected mode enums, keep costs as estimates |
| IDs | Guide says same as URL; OpenAPI says temporary document identifier | Preserve separate URL and ID values, no assumed equality/durability |
| Contents required outputs | Raw schema does not require statuses/results; guide says inspect statuses | Missing data is unknown/incomplete, not empty successful extraction |
| HIPAA freshness default | Normal omission allows fallback crawl; HIPAA page defines omission as eligible cache-only profile | Apply only explicit operator-enabled HIPAA profile; do not borrow normal default semantics |
| Context schema/rates/pricing | Context prose examples, no embedded OpenAPI; rates/pricing pages lack separate Context schedule | Prose-derived response subset, searchTime units/tokenizer/exact cost/rate behavior unknown |
| Find Similar availability | Not indexed; targeted reference 404 | Unverified/absent from current docs, not proven globally removed or deprecated; no capability |

## Dated commercial/access brief

As of 2026-09-07, [pricing](https://exa.ai/docs/reference/pricing.md) describes prepaid pay-as-you-go with no subscription/minimum, $20 new-account credit and $10 monthly Free Tier credit. These are public advertised offers, not inspected account entitlement or guaranteed future access.

- Search standard base **$7/1000 requests** including up to 10 results; above 10 adds **$1/1000 results**. AI page summaries add **$1/1000 pages**.
- Deep-lite/deep base **$12/1000**, deep-reasoning **$15/1000**, with the same extra result/page-summary costs. Approximate latencies are not SLAs.
- Contents **$1/1000 pages per content type**; text plus highlights counts separately, summary separately. Linked-subpage billing and incomplete-failure charging details are not established by fetched pages; do not estimate them as free.
- Answer **$5/1000 requests**; per-model distinctions unknown. Separate Context pricing is not published on the fetched pricing page; response examples are not a price quote.
- Default [rate limits](https://exa.ai/docs/reference/rate-limits.md): Search **10 QPS**, Contents **100 QPS**, Answer **10 QPS**. Context unknown. Account/enterprise limits may differ; no concurrency/burst/retry budget guarantee.
- [Security](https://exa.ai/docs/reference/security.md) advertises SOC 2 Type II and enterprise ZDR/HIPAA options, with vendor enablement. It lists regional sanctions/access restrictions including Crimea, Cuba, Iran, North Korea, Russia, Syria, Ukraine and Venezuela; blocks may be HTML Cloudflare WAF responses rather than API JSON. Do not bypass restrictions or treat this dated list as legal advice.

Returned costDollars is an estimate, not an invoice or preflight spend enforcement. Charges can accrue despite downstream projection/truncation. No retries or spend-authorizing payment flow is planned. Terms are unverified as noted above; webpage accessibility does not grant content redistribution rights.

## Public code compatibility sources

The supplied verified Dekopon baseline is cited in [provider plan](provider-plan.md). It was not modified or fetched as part of this vendor-only research. The plan distinguishes existing POST/Bearer/buffered-host support from proposed provider behavior, and does not pretend model manifest schemas are host validation.
