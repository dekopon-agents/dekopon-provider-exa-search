# Exa Search provider for Dekopon

`exa` contributes three separately grantable, billable read capabilities: `exa.search` (POST `https://api.exa.ai/search`), `exa.contents` (POST `/contents`), and `exa.answer` (POST `/answer`, buffered JSON only). Stdout is the validated vendor JSON followed by a newline, with result order, URLs, citations, usage, request IDs and response extensions unchanged; errors go to stderr. Each invocation makes one broker-mediated HTTP request; no endpoint, path, header, credential, or base-URL override exists. There are no retries, native sockets, filesystem access, or SSE. The broker injects credentials and enforces authorization and byte/time/request bounds.

## Commands

```sh
exa search 'orchard irrigation' --type fast --num-results 3 --include-domain example.org
exa contents https://example.org/report --text --max-age-hours -1
exa contents --id 'document-id' --contents-json '{"highlights":{"query":"irrigation"}}'
exa answer 'What are the findings?' --model exa-fast
exa search --input-json - <<'JSON'
{"query":"orchard irrigation","type":"deep","additionalQueries":["drip irrigation"],"contents":{"summary":{"schema":"{\"type\":\"object\",\"properties\":{\"finding\":{\"type\":\"string\"}}}"}},"outputSchema":"{\"type\":\"object\",\"properties\":{\"finding\":{\"type\":\"string\"}}}"}
JSON
```

`--input-json JSON` (or `--input-json -` for piped stdin) supplies the **entire** request and cannot be mixed with flags or positionals. `-` proposes only a marker; the guest reads at most 65,536 bytes at invocation, after broker authorization, then validates before HTTP. Direct capability input and inline/streamed JSON encode `outputSchema` and `summary.schema` as JSON-object **strings** (at most 65,536 bytes each). These are parsed back into JSON objects before the Exa request; malformed, non-object or oversized strings cause no HTTP request. `--output-schema-json` takes a JSON object argument; `--contents-json` retains its nested object syntax and converts its `summary.schema` into a string in the proposal. The same validator runs on direct capability input and on CLI proposals. `--contents-json JSON` accepts only nested content options (not IDs, URLs or other top-level fields), conflicts with `--text`, `--highlights`, `--max-age-hours`, `--subpages`. Common search flags include `--include-domain`, `--exclude-domain`, `--additional-query` (repeatable), `--start-published-date`, `--end-published-date`, `--user-location`, `--category`, `--compliance`, `--moderation`, `--system-prompt`, `--output-schema-json`; see `exa search --help`. `exa contents` accepts positional URLs *or* repeatable `--id` values, not both. Complex/nested extraction, beta highlights, output schemas and optional explicit JSON nulls are available through the typed JSON routes. Nulls and arbitrary nested output schemas are preserved on the wire. If dynamic highlights or verbosity is requested, the only extra guest header is Exa's fixed `Exa-Beta: dynamic-highlights-2026-08-28`.

The source contract is Exa OpenAPI 3.1.0 API version 2.0.0, `https://exa.ai/docs/exa-spec.yaml` (pinned SHA-256 `0c7eafdb8baac39ddfdab53e82c8053868bbbb2a289f9ee856524dcbef41a5c7`). Request models are handwritten: generated models lost nested `outputSchema` properties and rejected new response fields. Only `/search`, `/contents`, `/answer` are supported. Deprecated `/findSimilar`, deprecated `context`, `livecrawl`, `crawledBeforeDate`, crawl-date and text filters, `stream`, and all management or asynchronous APIs are rejected; use `maxAgeHours`, `snapshotAsOf`, `text` or `highlights` instead. Full synchronous deep modes (`deep-lite`, `deep`, `deep-reasoning`) and synthesis output are supported. Broker limits may truncate/refuse oversized output; the guest never silently truncates it.

## Operator deployment contract (not deployed by this repository)

Built against the published core SDK `=0.33.0`, using provider WIT `dekopon:provider/provider@0.4.0`, `dekopon:stdio/streams@0.1.0` and `dekopon:http/client@1.1.0`. Install a verified component digest in the provider set; configure a broker capability entry for provider `exa` with individually grantable `exa.search`, `exa.contents`, `exa.answer`. Minimum HTTP grant for each: `allowedHosts: [api.exa.ai]`, `allowedMethods: [POST]`, `maxRequests: 1`; HTTPS only. Provide finite `timeoutMs`, `maxOutputBytes`, `maxRequestBytes`, `maxResponseBytes` consistent with host limits and expected extraction size; deep search/answer can require more time than instant search. Bind a broker-owned Exa API credential with **Authorization: Bearer** injection for that host (the public spec permits Bearer or x-api-key). No guest-set authorization or exposed key. A cheap smoke after independently authorized rollout: `exa search 'test' --type instant --num-results 1` (still billable). No live/paying calls are performed in the tests here.

## Local validation

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --locked --target wasm32-unknown-unknown --lib -- -D warnings
cargo deny --all-features check bans licenses sources advisories
../provider-workflows/build.sh   # when this repo is alongside provider-workflows
DEKOPON_PROVIDER_COMPONENT="$PWD/exa-search-provider.wasm" cargo test --locked
wasm-tools component wit exa-search-provider.wasm
```

Native synthetic adapters cover successful responses for each operation and reject malformed/status cases; real component/testkit tests cover the broker host's denied-HTTP path and no-egress help/invalid input. The testkit cannot mock a response for the fixed HTTPS origin; successful real-component HTTP response handling, credential injection and paid vendor behavior remain unverified pending separately authorized deployment smoke. CI uses provider-workflows v4; release and merge are separate review-gated actions.
