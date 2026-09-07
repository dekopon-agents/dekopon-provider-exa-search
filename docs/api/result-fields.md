# SearchResultOutput: documented output fields

Structural inventory adapted from official sources. [Output semantics](outputs.md) explain provenance, confidence, optionality and conflicts. Nested company/person/publication alternatives share a path; consult the [wire definitions](../../schemas/upstream/documented-shapes.json) for variant-specific required lists. Schema-required does not make an absent upstream value known.

| Path | Type | Required in containing variant | Bounds / enum |
|---|---|---|---|
| `title` | string | yes in at least one variant | no additional bound documented |
| `url` | string | yes in at least one variant | format="uri" |
| `publishedDate` | string | no | format="date-time" |
| `author` | string / null | no | no additional bound documented |
| `id` | string | no | no additional bound documented |
| `image` | string | no | format="uri" |
| `favicon` | string | no | format="uri" |
| `text` | string | no | no additional bound documented |
| `highlights` | array of string | no | no additional bound documented |
| `highlightScores` | array of number | no | items: format="float" |
| `summary` | string | no | no additional bound documented |
| `subpages` | array of object | no | no additional bound documented |
| `subpages[].title` | string | yes in at least one variant | no additional bound documented |
| `subpages[].url` | string | yes in at least one variant | format="uri" |
| `subpages[].publishedDate` | string | no | format="date-time" |
| `subpages[].author` | string / null | no | no additional bound documented |
| `subpages[].id` | string | no | no additional bound documented |
| `subpages[].image` | string | no | format="uri" |
| `subpages[].favicon` | string | no | format="uri" |
| `entities` | array of object | no | no additional bound documented |
| `entities[].id` | string | yes in at least one variant | no additional bound documented |
| `entities[].type` | string | yes in at least one variant | const="company"; const="person"; const="publication" |
| `entities[].version` | integer | yes in at least one variant | minimum=1 |
| `entities[].properties` | object | yes in at least one variant | no additional bound documented |
| `entities[].properties.name` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.foundedYear` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.description` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workforce` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workforce.total` | number / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.headquarters` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.headquarters.address` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.headquarters.city` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.headquarters.postalCode` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.headquarters.country` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials.revenueAnnual` | number / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials.fundingTotal` | number / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials.fundingLatestRound` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials.fundingLatestRound.name` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials.fundingLatestRound.date` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.financials.fundingLatestRound.amount` | number / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.visitsMonthly` | number / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.countryRank` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.avgDurationSeconds` | number / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.history` | array of object | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.history[].value` | number | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.history[].dateFrom` | string | yes in at least one variant | no additional bound documented |
| `entities[].properties.webTraffic.history[].dateTo` | string | yes in at least one variant | no additional bound documented |
| `entities[].properties.research` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.worksCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.citationCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.areas` | array of string | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks` | array of object | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].title` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].year` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].venue` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].citationCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].doi` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].id` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.notableWorks[].type` | string / null | yes in at least one variant | enum=["article", "book", "book-chapter", "dataset", "dissertation", "preprint", "report", "review"] |
| `entities[].properties.research.topResearchers` | array of object | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.topResearchers[].person` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.topResearchers[].person.name` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.topResearchers[].person.id` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.topResearchers[].worksCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.topResearchers[].citationCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.firstName` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.lastName` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.location` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory` | array of object | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].title` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].location` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].dates` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].dates.from` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].dates.to` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].company` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].company.id` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.workHistory[].company.name` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory` | array of object | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].degree` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].dates` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].dates.from` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].dates.to` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].institution` | object / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].institution.id` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.educationHistory[].institution.name` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.hIndex` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.firstPublicationYear` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.research.latestPublicationYear` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.title` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.year` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.date` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.type` | string / null | yes in at least one variant | enum=["article", "book", "book-chapter", "dataset", "dissertation", "preprint", "report", "review"] |
| `entities[].properties.language` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.citationCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.authors` | array of object | yes in at least one variant | no additional bound documented |
| `entities[].properties.authors[].name` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.authors[].id` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.referenceCount` | integer / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.abstract` | string / null | yes in at least one variant | no additional bound documented |
| `entities[].properties.doi` | string / null | yes in at least one variant | no additional bound documented |
| `extras` | object | no | no additional bound documented |
| `extras.links` | array of string | no | no additional bound documented |
| `extras.imageLinks` | array of string | no | no additional bound documented |
| `extras.richImageLinks` | array of object | no | no additional bound documented |
| `extras.richImageLinks[].url` | string | yes in at least one variant | no additional bound documented |
| `extras.richImageLinks[].alt` | string | no | no additional bound documented |
| `extras.richLinks` | array of object | no | no additional bound documented |
| `extras.richLinks[].url` | string | yes in at least one variant | no additional bound documented |
| `extras.richLinks[].anchor` | string | no | no additional bound documented |
| `extras.codeBlocks` | array of object | no | no additional bound documented |
| `extras.codeBlocks[].text` | string | yes in at least one variant | no additional bound documented |
| `extras.codeBlocks[].source` | string | yes in at least one variant | no additional bound documented |
