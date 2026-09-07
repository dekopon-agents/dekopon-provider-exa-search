# ContextResponse: documented output fields

Structural inventory adapted from official sources. [Output semantics](outputs.md) explain provenance, confidence, optionality and conflicts. Nested company/person/publication alternatives share a path; consult the [wire definitions](../../schemas/upstream/documented-shapes.json) for variant-specific required lists. Schema-required does not make an absent upstream value known.

| Path | Type | Required in containing variant | Bounds / enum |
|---|---|---|---|
| `requestId` | string | no | no additional bound documented |
| `query` | string | no | no additional bound documented |
| `response` | string | yes in at least one variant | no additional bound documented |
| `resultsCount` | integer | no | no additional bound documented |
| `costDollars` | object | no | no additional bound documented |
| `costDollars.total` | number | no | format="float" |
| `costDollars.search` | object | no | no additional bound documented |
| `costDollars.search.neural` | number | no | format="float" |
| `costDollars.search.keyword` | number | no | format="float" |
| `costDollars.summary` | number | no | format="float" |
| `costDollars.contents` | object | no | no additional bound documented |
| `costDollars.contents.text` | number | no | format="float" |
| `costDollars.contents.highlights` | number | no | format="float" |
| `costDollars.contents.summary` | number | no | format="float" |
| `searchTime` | number | no | no additional bound documented |
| `outputTokens` | integer | no | no additional bound documented |
