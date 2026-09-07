# ContextResponse: documented output fields

Prose-derived documented subset from the [Context reference](https://exa.ai/docs/reference/context.md), not an official OpenAPI schema. [Output semantics](outputs.md) explain provenance, confidence, optionality and conflicts; consult the [wire definitions](../../schemas/upstream/documented-shapes.json) for the local subset. Only `costDollars.total` and `costDollars.search.neural` are established by the Context examples. Other breakdown fields remain unknown, with open schema objects allowing bounded raw preservation rather than asserting a shared endpoint contract. Schema-required does not make an absent upstream value known.

| Path | Type | Required in containing variant | Bounds / enum |
|---|---|---|---|
| `requestId` | string | no | no additional bound documented |
| `query` | string | no | no additional bound documented |
| `response` | string | yes in at least one variant | no additional bound documented |
| `resultsCount` | integer | no | no additional bound documented |
| `costDollars` | object | no | no additional bound documented |
| `costDollars.total` | number | no | no additional bound documented |
| `costDollars.search` | object | no | no additional bound documented |
| `costDollars.search.neural` | number | no | no additional bound documented |
| `searchTime` | number | no | no additional bound documented |
| `outputTokens` | integer | no | no additional bound documented |
