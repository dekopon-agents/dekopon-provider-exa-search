# Context input inventory

All body properties of the documented [wire schema](../../schemas/upstream/context-request.schema.json), including nested object alternatives. Snapshot: 2026-09-07. Read with [context endpoint notes](context.md).

Types include explicit null where the OpenAPI permits it. Omission is distinct from null; defaults below are vendor defaults, never provider-output defaults. “Required” is relative to the containing variant; /contents additionally requires exactly one of ids/urls. Open object maps are caller-defined JSON, not a finite list of additional vendor parameters.

| JSON body path | Type | Required | Vendor default | Bounds / enum | Meaning / plan |
|---|---|---|---|---|---|
| `query` | string | yes (variant) | unspecified | minLength=1; maxLength=2000 | Natural-language information need; retrieval query, not a URL or SQL expression. **supported**. |
| `tokensNum` | integer / string | no | "dynamic" | minimum=50; maximum=100000; const="dynamic" | Context token target: dynamic vendor-selected length or integer budget. Not an exact byte cap or complete repository guarantee. **supported**. |
