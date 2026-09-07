# Answer input inventory

All body properties of the documented [wire schema](../../schemas/upstream/answer-request.schema.json), including nested object alternatives. Snapshot: 2026-09-07. Read with [answer endpoint notes](answer.md).

Types include explicit null where the OpenAPI permits it. Omission is distinct from null; defaults below are vendor defaults, never provider-output defaults. “Required” is relative to the containing variant; /contents additionally requires exactly one of ids/urls. Open object maps are caller-defined JSON, not a finite list of additional vendor parameters.

| JSON body path | Type | Required | Vendor default | Bounds / enum | Meaning / plan |
|---|---|---|---|---|---|
| `query` | string | yes (variant) | unspecified | minLength=1 | Natural-language information need; retrieval query, not a URL or SQL expression. **explicitly deferred**. |
| `stream` | boolean | no | false | no additional bound documented | SSE response selection; search requires outputSchema to actually stream. Initial provider rejects true because HTTP responses are buffered. **explicitly deferred**. |
| `text` | boolean | no | false | no additional bound documented | Boolean enables/disables extracted page text; object chooses rendering/length. Never the original complete webpage. **explicitly deferred**. |
| `model` | string | no | "exa" | enum=["exa", "exa-pro", "exa-research", "exa-fast"] | Answer generator: exa default; exa-pro, exa-research, exa-fast named variants. Exact capability/price/latency differences unverified. **explicitly deferred**. |
| `systemPrompt` | string | no | unspecified | no additional bound documented | Guidance for generated output; in deep modes also search planning. Not an independent ranking weight API. **explicitly deferred**. |
| `userLocation` | string / null | no | unspecified | no additional bound documented | Two-letter ISO country hint for user geography, not a verified person/company location filter. **explicitly deferred**. |
| `outputSchema` | object | no | unspecified | no additional bound documented | Shape for generated synthesis, separate from page summaries and provider output projection. Search supports text/object roots. **explicitly deferred**. |
| `outputSchema.type` | string | no | unspecified | no additional bound documented | Root kind: text gives prose (search); object gives structured properties. Answer describes typically object rather than a strict enum. **explicitly deferred**. |
| `outputSchema.properties` | object | no | unspecified | no additional bound documented | Named user-defined property schemas; arbitrary JSON values allowed by structural wire schema, not proof vendor supports every keyword. **explicitly deferred**. |
| `outputSchema.required` | array of string | no | unspecified | no additional bound documented | Names that generated object must contain; no provider-runtime generation guarantee. **explicitly deferred**. |
| `outputSchema.description` | string | no | unspecified | no additional bound documented | Instructions describing desired synthesized structure/content. **explicitly deferred**. |
| `outputSchema.additionalProperties` | boolean | no | false | no additional bound documented | Whether generated object may contain unspecified fields. **explicitly deferred**. |
