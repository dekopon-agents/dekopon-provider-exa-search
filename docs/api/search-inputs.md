# Search input inventory

All body properties of the documented [wire schema](../../schemas/upstream/search-request.schema.json), including nested object alternatives. Snapshot: 2026-09-07. Read with [search endpoint notes](search.md).

Types include explicit null where the OpenAPI permits it. Omission is distinct from null; defaults below are vendor defaults, never provider-output defaults. “Required” is relative to the containing variant; /contents additionally requires exactly one of ids/urls. Open object maps are caller-defined JSON, not a finite list of additional vendor parameters.

| JSON body path | Type | Required | Vendor default | Bounds / enum | Meaning / plan |
|---|---|---|---|---|---|
| `includeDomains` | array of string / null | no | unspecified | maxItems=1200 | Restrict sources to hostnames, path prefixes, or wildcard subdomains; use instead of site: query duplication. **supported**. |
| `excludeDomains` | array of string / null | no | unspecified | maxItems=1200 | Remove matching hostnames, path prefixes, or wildcard subdomains; unsupported for people/company. **supported**. |
| `startCrawlDate` | string / null | no | unspecified | format="date-time" | Legacy crawl-date lower bound; API ignores it. Not a freshness control. **deprecated/ignored**. |
| `endCrawlDate` | string / null | no | unspecified | format="date-time" | Legacy crawl-date upper bound; API ignores it. Not a freshness control. **deprecated/ignored**. |
| `startPublishedDate` | string / null | no | unspecified | format="date-time" | Keep estimated publication dates after this ISO 8601 boundary; unsupported for people/company. **supported**. |
| `endPublishedDate` | string / null | no | unspecified | format="date-time" | Keep estimated publication dates before this ISO 8601 boundary; unsupported for people/company. **supported**. |
| `numResults` | integer / null | no | 10 | minimum=1; maximum=100 | Requested result count, not pagination. Public schema ceiling 100; plan entitlement may be 25; fewer may return. **supported**. |
| `context` | boolean / object / null | no | unspecified | no additional bound documented | Deprecated combined page-content string switch/options, distinct from the /context endpoint; reject in new provider input. **deprecated/ignored**. |
| `context.maxCharacters` | integer | no | unspecified | minimum=1; maximum=10000 | Legacy combined-context character limit; reject rather than imply current full-page retrieval. **deprecated/ignored**. |
| `moderation` | boolean / null | no | false | no additional bound documented | Enable vendor unsafe-content filtering (false leaves this option disabled, not a guarantee of unsafe or safe results). **supported**. |
| `contents` | object / null | no | unspecified | no additional bound documented | Content extraction and crawl controls nested here only for /search. **supported**. |
| `contents.text` | boolean / object / null | no | false | no additional bound documented | Boolean enables/disables extracted page text; object chooses rendering/length. Never the original complete webpage. **supported**. |
| `contents.text.maxCharacters` | integer / null | no | unspecified | minimum=1; maximum=10000 | Maximum extracted text characters per page, not bytes; no implied original-page completeness. **supported**. |
| `contents.text.includeHtmlTags` | boolean / null | no | false | no additional bound documented | Request lightweight HTML rather than markdown-style rendering; fresh fetch needed to apply new rendering. **supported**. |
| `contents.text.verbosity` | string / null | no | "compact" | enum=["compact", "standard", "full"] | compact: main content; standard: more surrounding context; full: broadest rendered text. standard/full may coincide. **supported**. |
| `contents.text.includeSections` | array of string / null | no | unspecified | items: enum=["header", "navigation", "banner", "body", "sidebar", "footer", "metadata"] | Best-effort inclusion of classified page sections; not a strict semantic filter. **supported**. |
| `contents.text.excludeSections` | array of string / null | no | unspecified | items: enum=["header", "navigation", "banner", "body", "sidebar", "footer", "metadata"] | Best-effort exclusion of classified sections; overlap precedence is undocumented, so provider rejects overlap. **supported**. |
| `contents.highlights` | boolean / object / null | no | false | no additional bound documented | Boolean enables/disables selected excerpts; object steers selection and budgets. **supported**. |
| `contents.highlights.query` | string / null | no | unspecified | no additional bound documented | Override the question used to choose relevant excerpts. **supported**. |
| `contents.highlights.verbosity` | string / null | no | unspecified | enum=["low", "medium", "high"] | Beta low/medium/high: progressively larger vendor-tuned token budgets; per page normally, shared across results with dynamic. Conflicts with maxCharacters and numSentences. **beta**. |
| `contents.highlights.dynamic` | boolean / null | no | unspecified | no additional bound documented | Research-preview result-set-wide allocation of a shared context budget. Requires fixed Exa-Beta flag; incompatible with maxCharacters. **beta**. |
| `contents.highlights.maxCharacters` | integer / null | no | unspecified | minimum=1; maximum=10000 | Total highlight characters per URL; incompatible with dynamic and verbosity. Not a byte ceiling. **supported**. |
| `contents.highlights.numSentences` | integer / null | no | unspecified | minimum=1 | Deprecated: OpenAPI says approximately 1333 characters per sentence mapping; guides say remove. Not a supported sentence-count guarantee; provider rejects. **deprecated/ignored**. |
| `contents.highlights.highlightsPerUrl` | integer / null | no | unspecified | minimum=1 | Deprecated and ignored upstream; provider rejects. **deprecated/ignored**. |
| `contents.summary` | object / null | no | unspecified | no additional bound documented | LLM-generated page summary; object supports query and JSON Schema. Boolean is guide-only conflict, deferred. **supported**. |
| `contents.summary.query` | string / null | no | unspecified | no additional bound documented | Instructions for the per-page generated summary. **supported**. |
| `contents.summary.schema` | object / null | no | unspecified | no additional bound documented | Caller-supplied structured summary schema; arbitrary JSON Schema keywords in a JSON object (guide says Draft 7). **supported**. |
| `contents.extras` | object / null | no | unspecified | no additional bound documented | Optional extracted page artifacts, independently requested counts. **supported**. |
| `contents.extras.links` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of outgoing URLs requested per result. **supported**. |
| `contents.extras.imageLinks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of image URL strings requested per result. **supported**. |
| `contents.extras.richImageLinks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of image URL/alt-text records requested per result. **supported**. |
| `contents.extras.richLinks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of URL/anchor-text records requested per result. **supported**. |
| `contents.extras.codeBlocks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of extracted code text/source records requested per result. **supported**. |
| `contents.context` | boolean / object / null | no | unspecified | no additional bound documented | Deprecated combined page-content string switch/options, distinct from the /context endpoint; reject in new provider input. **deprecated/ignored**. |
| `contents.context.maxCharacters` | integer | no | unspecified | minimum=1; maximum=10000 | Legacy combined-context character limit; reject rather than imply current full-page retrieval. **deprecated/ignored**. |
| `contents.livecrawl` | string / null | no | unspecified | enum=["never", "always", "fallback", "preferred"] | Deprecated never/always/fallback/preferred crawl policy; use maxAgeHours. Never combine both; legacy string does not guarantee new parser output. **deprecated/ignored**. |
| `contents.livecrawlTimeout` | integer / null | no | 10000 | maximum=90000; exclusiveMinimum=0 | Vendor live-fetch timeout in milliseconds, distinct from native HTTP deadline. **supported**. |
| `contents.maxAgeHours` | integer / null | no | unspecified | minimum=-1; maximum=720 | -1: cache only; 0: fresh fetch; positive: accept cache younger than N hours, otherwise fetch; omitted: fetch if no cache. Null semantics unspecified. **supported**. |
| `contents.subpages` | integer / null | no | 0 | minimum=0; maximum=100 | Additional linked pages per parent, best-effort count subject to server constraints; not an exact traversal guarantee. **supported**. |
| `contents.subpageTarget` | string / array of string / null | no | unspecified | minLength=1; maxLength=100; items: minLength=1; maxLength=100; minItems=0; maxItems=100 | One term or array of terms to prioritize subpage selection; not a guaranteed path match. **supported**. |
| `query` | string | yes (variant) | unspecified | minLength=1 | Natural-language information need; retrieval query, not a URL or SQL expression. **supported**. |
| `additionalQueries` | array of string / null | no | unspecified | minItems=1; maxItems=10 | One to ten alternative queries used with the main query; requires deep-lite/deep/deep-reasoning. **supported**. |
| `type` | string / null | no | "auto" | enum=["instant", "fast", "auto", "deep-lite", "deep", "deep-reasoning"] | auto balanced; fast low latency; instant minimum latency; deep-lite lightweight synthesis (~4s); deep multi-step synthesis; deep-reasoning stronger analysis. **supported**. |
| `category` | string / null | no | unspecified | enum=["company", "publication", "news", "personal site", "financial report", "people"] | company organization pages; people profiles; publication scholarly works; news reporting; personal site individual websites; financial report financial filings/reports. Unknown hints conflict with schema enum. **supported**. |
| `userLocation` | string / null | no | unspecified | no additional bound documented | Two-letter ISO country hint for user geography, not a verified person/company location filter. **supported**. |
| `compliance` | string / null | no | unspecified | enum=["hipaa"] | Enterprise HIPAA/cache-only mode requiring explicit vendor enablement/BAA; operator-only, not a model-controlled compliance claim. **operator-only**. |
| `outputSchema` | object / null | no | unspecified | no additional bound documented | Shape for generated synthesis, separate from page summaries and provider output projection. Search supports text/object roots. **supported**. |
| `outputSchema.type` | string | yes (variant) | unspecified | const="text"; const="object" | Root kind: text gives prose (search); object gives structured properties. Answer describes typically object rather than a strict enum. **supported**. |
| `outputSchema.description` | string | no | unspecified | no additional bound documented | Instructions describing desired synthesized structure/content. **supported**. |
| `outputSchema.properties` | object | no | unspecified | no additional bound documented | Named user-defined property schemas; arbitrary JSON values allowed by structural wire schema, not proof vendor supports every keyword. **supported**. |
| `outputSchema.required` | array of string | no | unspecified | no additional bound documented | Names that generated object must contain; no provider-runtime generation guarantee. **supported**. |
| `outputSchema.additionalProperties` | boolean | no | unspecified | no additional bound documented | Whether generated object may contain unspecified fields. **supported**. |
| `systemPrompt` | string / null | no | unspecified | no additional bound documented | Guidance for generated output; in deep modes also search planning. Not an independent ranking weight API. **supported**. |
| `stream` | boolean / null | no | false | no additional bound documented | SSE response selection; search requires outputSchema to actually stream. Initial provider rejects true because HTTP responses are buffered. **explicitly deferred**. |
