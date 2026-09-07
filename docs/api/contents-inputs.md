# Contents input inventory

All body properties of the documented [wire schema](../../schemas/upstream/contents-request.schema.json), including nested object alternatives. Snapshot: 2026-09-07. Read with [contents endpoint notes](contents.md).

Types include explicit null where the OpenAPI permits it. Omission is distinct from null; defaults below are vendor defaults, never provider-output defaults. “Required” is relative to the containing variant; /contents additionally requires exactly one of ids/urls. Open object maps are caller-defined JSON, not a finite list of additional vendor parameters.

| JSON body path | Type | Required | Vendor default | Bounds / enum | Meaning / plan |
|---|---|---|---|---|---|
| `ids` | array of string | no | unspecified | items: minLength=1; maxLength=2048; minItems=1; maxItems=100 | Search document IDs/URL strings. Supply exactly one of ids or urls; temporary IDs are not durable handles. **supported**. |
| `urls` | array of string | no | unspecified | items: minLength=1; maxLength=2048; minItems=1; maxItems=100 | Known source URLs; exactly one of urls or ids. JSON array, not a comma-separated query string. **supported**. |
| `compliance` | string / null | no | unspecified | enum=["hipaa"] | Enterprise HIPAA/cache-only mode requiring explicit vendor enablement/BAA; operator-only, not a model-controlled compliance claim. **operator-only**. |
| `text` | boolean / object / null | no | false | no additional bound documented | Boolean enables/disables extracted page text; object chooses rendering/length. Never the original complete webpage. **supported**. |
| `text.maxCharacters` | integer / null | no | unspecified | minimum=1; maximum=10000 | Maximum extracted text characters per page, not bytes; no implied original-page completeness. **supported**. |
| `text.includeHtmlTags` | boolean / null | no | false | no additional bound documented | Request lightweight HTML rather than markdown-style rendering; fresh fetch needed to apply new rendering. **supported**. |
| `text.verbosity` | string / null | no | "compact" | enum=["compact", "standard", "full"] | compact: main content; standard: more surrounding context; full: broadest rendered text. standard/full may coincide. **supported**. |
| `text.includeSections` | array of string / null | no | unspecified | items: enum=["header", "navigation", "banner", "body", "sidebar", "footer", "metadata"] | Best-effort inclusion of classified page sections; not a strict semantic filter. **supported**. |
| `text.excludeSections` | array of string / null | no | unspecified | items: enum=["header", "navigation", "banner", "body", "sidebar", "footer", "metadata"] | Best-effort exclusion of classified sections; overlap precedence is undocumented, so provider rejects overlap. **supported**. |
| `highlights` | boolean / object / null | no | false | no additional bound documented | Boolean enables/disables selected excerpts; object steers selection and budgets. **supported**. |
| `highlights.query` | string / null | no | unspecified | no additional bound documented | Override the question used to choose relevant excerpts. **supported**. |
| `highlights.verbosity` | string / null | no | unspecified | enum=["low", "medium", "high"] | Beta low/medium/high: progressively larger vendor-tuned token budgets; per page normally, shared across results with dynamic. Conflicts with maxCharacters and numSentences. **beta**. |
| `highlights.dynamic` | boolean / null | no | unspecified | no additional bound documented | Research-preview result-set-wide allocation of a shared context budget. Requires fixed Exa-Beta flag; incompatible with maxCharacters. **beta**. |
| `highlights.maxCharacters` | integer / null | no | unspecified | minimum=1; maximum=10000 | Total highlight characters per URL; incompatible with dynamic and verbosity. Not a byte ceiling. **supported**. |
| `highlights.numSentences` | integer / null | no | unspecified | minimum=1 | Deprecated: OpenAPI says approximately 1333 characters per sentence mapping; guides say remove. Not a supported sentence-count guarantee; provider rejects. **deprecated/ignored**. |
| `highlights.highlightsPerUrl` | integer / null | no | unspecified | minimum=1 | Deprecated and ignored upstream; provider rejects. **deprecated/ignored**. |
| `summary` | object / null | no | unspecified | no additional bound documented | LLM-generated page summary; object supports query and JSON Schema. Boolean is guide-only conflict, deferred. **supported**. |
| `summary.query` | string / null | no | unspecified | no additional bound documented | Instructions for the per-page generated summary. **supported**. |
| `summary.schema` | object / null | no | unspecified | no additional bound documented | Caller-supplied structured summary schema; arbitrary JSON Schema keywords in a JSON object (guide says Draft 7). **supported**. |
| `extras` | object / null | no | unspecified | no additional bound documented | Optional extracted page artifacts, independently requested counts. **supported**. |
| `extras.links` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of outgoing URLs requested per result. **supported**. |
| `extras.imageLinks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of image URL strings requested per result. **supported**. |
| `extras.richImageLinks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of image URL/alt-text records requested per result. **supported**. |
| `extras.richLinks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of URL/anchor-text records requested per result. **supported**. |
| `extras.codeBlocks` | integer / null | no | 0 | minimum=0; maximum=1000 | Count of extracted code text/source records requested per result. **supported**. |
| `context` | boolean / object / null | no | unspecified | no additional bound documented | Deprecated combined page-content string switch/options, distinct from the /context endpoint; reject in new provider input. **deprecated/ignored**. |
| `context.maxCharacters` | integer | no | unspecified | minimum=1; maximum=10000 | Legacy combined-context character limit; reject rather than imply current full-page retrieval. **deprecated/ignored**. |
| `livecrawl` | string / null | no | unspecified | enum=["never", "always", "fallback", "preferred"] | Deprecated never/always/fallback/preferred crawl policy; use maxAgeHours. Never combine both; legacy string does not guarantee new parser output. **deprecated/ignored**. |
| `livecrawlTimeout` | integer / null | no | 10000 | maximum=90000; exclusiveMinimum=0 | Vendor live-fetch timeout in milliseconds, distinct from native HTTP deadline. **supported**. |
| `maxAgeHours` | integer / null | no | unspecified | minimum=-1; maximum=720 | -1: cache only; 0: fresh fetch; positive: accept cache younger than N hours, otherwise fetch; omitted: fetch if no cache. Null semantics unspecified. **supported**. |
| `subpages` | integer / null | no | 0 | minimum=0; maximum=100 | Additional linked pages per parent, best-effort count subject to server constraints; not an exact traversal guarantee. **supported**. |
| `subpageTarget` | string / array of string / null | no | unspecified | minLength=1; maxLength=100; items: minLength=1; maxLength=100; minItems=0; maxItems=100 | One term or array of terms to prioritize subpage selection; not a guaranteed path match. **supported**. |
