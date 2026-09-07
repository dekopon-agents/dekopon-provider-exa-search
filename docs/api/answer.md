# Answer — adjacent generated synthesis, deferred

**POST `https://api.exa.ai/answer`**. Official [Answer OpenAPI](https://exa.ai/docs/reference/answer.md) declares base `https://api.exa.ai` and **apiKey OR Bearer** security, just like Search/Contents. This endpoint searches and generates a cited answer rather than primarily returning retrieval results. Indexed for discovery; **no initial Answer capability or CLI is planned**.

- [Complete documented input table](answer-inputs.md), [output fields](answer-output.md)
- [Wire request](../../schemas/upstream/answer-request.schema.json), [wire response](../../schemas/upstream/answer-response.schema.json)
- [Minimal request](../../examples/answer-minimal.json), [feature-rich request](../../examples/answer-rich.json), [synthetic response](../../examples/answer-response.json)

```json
{"query":"What is an irrigation sensor?"}
```

Required `query` is nonempty natural language. Optional `model` defaults `exa`; `exa-pro`, `exa-research` and `exa-fast` are also enumerated. Their exact relative quality, routing, access, per-model price and latency are not established by the fetched reference; names are not a substitute for evidence. `text` default false enables source text in citations; `systemPrompt` adds generation guidance; nullable `userLocation` supplies a two-letter country hint.

`outputSchema` describes a Draft 7 generated answer structure (type, properties, required, description, additionalProperties plus extensible JSON-valued keys); otherwise `answer` is a string. The response requires `answer`, either string or object; citations and costs are optional. Citation rows require title/URL, with optional estimated publishedDate, nullable author, temporary id, image, favicon and requested text. These are source references, not independent proof that every generated claim is supported. There is no documented Search-style grounding confidence map here.

`stream` defaults false; true yields SSE/chat-completion-shaped chunks rather than one JSON response. `AnswerStreamChunk` is preserved in [structural definitions](../../schemas/upstream/documented-shapes.json) for reference only. Existing buffered HTTP does not provide streaming delivery. No arbitrary URL, model dispatch, or generated synthesis fallback is added to the initial provider.

Public pricing lists $0.005/request and default rate limit 10 QPS as of 2026-09-07; model-specific differences are unknown. Errors include ordinary API failures and 501 `UNABLE_TO_GENERATE_RESPONSE`, or 403 `PROHIBITED_CONTENT`; no automatic retry/rephrase. See [transport](transport.md) and [source conflicts](../sources.md).
