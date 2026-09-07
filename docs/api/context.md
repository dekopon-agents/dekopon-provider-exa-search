# Code Context

**POST `https://api.exa.ai/context`**, verified directly from the official [Context reference](https://exa.ai/docs/reference/context.md) curl and fetch examples. Its public curl example explicitly uses Authorization Bearer. This is not a GET route and not the deprecated Search/Contents `context` property.

Choose for assembled, token-oriented code snippets and library usage examples from repositories, documentation and Q&A. For independently ranked source rows, source filters, publication dates or per-page highlights choose [Search](search.md), as recommended by the [code vertical](https://exa.ai/docs/reference/verticals/code.md). Do not invent a `code` category or assume this route accepts Search options.

- [Complete input table](context-inputs.md), [output field table](context-output.md)
- [Prose-derived request shape](../../schemas/upstream/context-request.schema.json), [response subset](../../schemas/upstream/context-response.schema.json)
- [Minimal request](../../examples/context-minimal.json), [feature-rich request](../../examples/context-rich.json), [synthetic response](../../examples/context-response.json)
- [Proposed provider input](../../schemas/provider/context-input.schema.json)

```json
{"query":"Python grouping a synthetic irrigation dataset"}
```

Only two body parameters are documented: required `query` string, 1–2000 characters; optional `tokensNum`, default `"dynamic"`, or integer 50–100000. Dynamic lets the vendor size context; 5000 is its suggested ordinary integer starting point, with 10000 for more context. More requested tokens can increase cost. These are token controls, **not exact serialized bytes** and not a promise to return exactly that token count. No other filter, crawl/freshness, category, pagination, stream or structured-output control is documented here.

Output is an assembled `response` string containing formatted snippets and contextual/source URLs, not the Search `results` array. Preserve source URLs as text; do not parse markdown into authoritative citations without validation. `resultsCount` describes matched results according to examples, not a count of independently returned full pages. `outputTokens` is vendor-reported; tokenizer details and strict limit behavior are not established. `requestId` identifies the call; `query` echoes it; `costDollars` is an estimate, not invoice evidence. `searchTime` appears numerically in examples but **its unit is not explicitly specified on this page**; do not borrow Search milliseconds as a guaranteed Context contract.

Context's page has no embedded OpenAPI. Local schemas are explicitly prose-derived documented-shape subsets: request lengths/ranges are sourced, response required `response` is a fixture modeling choice, not an exhaustive upstream guarantee. Nullability/missing-field behavior, rate limit, separate pricing schedule, status-code particulars and stable result ordering are unverified. Generic [transport error handling](transport.md) is conservative, not route-specific proof.

Initial provider output supports `relevant`, `page-text` (the same assembled context string, **not a complete webpage**) and bounded `raw`; `links` and `brief` are rejected instead of manufacturing Search-style rows. No second retrieval, repository cloning, code execution, or follow-up crawl is implied.
