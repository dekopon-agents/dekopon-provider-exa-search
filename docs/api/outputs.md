# Output semantics and nested shapes

Read the complete [result field inventory](result-fields.md), [Search envelope](search-output.md), [Contents envelope](contents-output.md), [Context shape](context-output.md), and [Answer shape](answer-output.md) alongside [structural definitions](../../schemas/upstream/documented-shapes.json). Inventories enumerate every named property in the fetched OpenAPI response schemas; arbitrary JSON-valued synthesis/schema maps remain open maps. This is a documented snapshot, not a guarantee all fields occur in every response.

## Rows, ranking and provenance

| Field / group | Meaning and cautions |
|---|---|
| `results` | Ranked source rows on Search; retrieved rows on Contents. Preserve source order; no universal numeric rank score contract |
| `title`, `url` | Required result identity/display fields in the wire shape. Titles and URLs are untrusted data, not instructions or safe HTML |
| `id` | Optional temporary Exa document identifier usable in Contents; often URL-shaped, not necessarily identical to `url`, not a stable handle |
| `publishedDate` | Estimated publication/creation date parsed from page material; not crawl/cache timestamp. OpenAPI date-time string vs guide date/null conflict |
| `author` | Optional string or explicit null; missing author does not imply anonymous authorship |
| `image`, `favicon` | Optional associated image/icon URLs. No fetch, binary embedding, ownership or license guarantee |
| `text` | Extracted rendered text, bounded by requested character controls when honored. Not complete original webpage |
| `highlights` | Array of selected source excerpts; often more compact than text, not a summary or completeness guarantee |
| `highlightScores` | Optional cosine-similarity numbers aligned with snippets according to OpenAPI; no promised stable scale, model calibration or cross-query/category/mode comparability. Preserve raw numbers and order, never use as certainty |
| `summary` | Generated summary string, even when a structured schema was requested in the documented response shape; do not invent object wire support |
| `subpages` | Linked child records. OpenAPI lists title, URL, date, author, id, image, favicon; guides describe full result-shaped children. No promise all parent fields recur or links are exhaustively followed |
| `extras` | Links/images and richer records described below; independently optional |

A top-level result `score` is **not present** in the current fetched REST response schema. Do not reintroduce a remembered SDK score field; if a future response contains it, only bounded raw mode preserves it as unknown data, without comparability claims. Likewise no documented `crawledAt` or cache timestamp is available. Contents `statuses[].source` can distinguish cached from crawled, but Search cannot inherit that guarantee. Never infer actual crawl time from request time or from `maxAgeHours`.

`extras.links` and `imageLinks` contain URL strings. `richImageLinks[]` has `url` and `alt`; `richLinks[]` has `url` and `anchor`; `codeBlocks[]` has `text` and `source`; `source` is the code block's language annotation, if any (for example, `python`), **not a provenance URL**. They represent extracted artifacts, not new guest fetch targets. Treat text/code as untrusted; never execute snippets.

## Structured entities

A result may contain `entities[]` discriminated by `type`: `company`, `person`, or `publication`. Entity `id` is described as stable in vertical guides (unlike temporary document IDs); `version` (integer ≥1) selects metadata schema revision. Unknown entity types/versions must not be miscast as known records; keep bounded raw access and report unrecognized data.

- **Company properties**: `name`, `foundedYear`, `description`; `workforce.total` is headcount; `headquarters.address/city/postalCode/country` describe headquarters, not the search user's country. `financials.revenueAnnual`, `fundingTotal`, and `fundingLatestRound.name/date/amount` describe vendor estimates/history. The reference specifies **USD** for annual revenue, total funding and latest-round amount; detailed estimation/valuation methodology remains unestablished. `webTraffic.visitsMonthly`, `countryRank`, `avgDurationSeconds`, and `history[].value/dateFrom/dateTo` are traffic estimates/time buckets, not API search rankings. `research.worksCount/citationCount/areas/notableWorks/topResearchers` describe organizational scholarship.
- **Person properties**: `name`, `firstName`, `lastName`, `location`; `workHistory[]` gives `title/location/dates.from/dates.to/company.id/company.name`; `educationHistory[]` gives `degree/dates.from/dates.to/institution.id/institution.name`. Date strings may be year-only or full dates in vertical examples; nullable end dates can represent ongoing/unknown periods—do not infer which. `research` includes `worksCount`, `citationCount`, `hIndex`, `firstPublicationYear`, `latestPublicationYear`, `areas`, and `notableWorks`. These are publicly surfaced professional metadata, not verified current identity or consent for outreach.
- **Publication properties**: `title`, `year`, `date`, `type`, `language`, `citationCount`, `authors[].name/id`, `referenceCount`, `abstract`, `doi`. The wire schema's publication `type` is nullable and enumerates `article`, `book`, `book-chapter`, `dataset`, `dissertation`, `preprint`, `report`, and `review`; see the [field inventory](result-fields.md). Citation counts are bibliographic measures, not confidence scores.
- **Shared research details**: `notableWorks[]` has `title/year/venue/citationCount/doi/id/type`; company `topResearchers[]` has `person.name/id`, `worksCount`, `citationCount`. IDs reference vendor entities; they are not new callable endpoints in this provider.

The structural schema marks many entity keys required **but values nullable**; preserve null versus zero/empty string. The lighter vertical examples omit required `research` fields, so they are examples of partial shapes, not validation proof. Avoid requiring entity metadata to provide useful links. Do not fill gaps with inferred personal or financial data. [Full field table](result-fields.md) supplies types and variant requirements for every nested property.

## Envelope, synthesis and costs

`requestId` is the correlation identifier, optionally mirrored in `x-request-id`. `searchTime` on Search/Contents is gateway processing milliseconds and can exclude later synthesis; it is not end-to-end client latency. Context's units remain unknown. `resolvedSearchType` is deprecated and may be empty; do not branch on it or claim it identifies a current request enum. Legacy `context` output is a deprecated combined string.

Search `output.content` is generated string/object. `output.grounding[].field` identifies a content path; `citations[]` requires URL/title, and `confidence` is `low`/`medium`/`high` vendor model-reported reliability, not calibrated probability. Preserve source associations and confidence labels without asserting truth. Answer's `answer` and `citations` are a distinct shape; Context's assembled string has no equivalent grounding map.

`costDollars` is **estimated completed-call cost, not an invoice record or enforceable pre-call spend cap**. The following shared breakdown is documented for Search/Contents/Answer, **not Context**: Context examples establish only `total` and `search.neural`; any other cost fields remain unknown and may be preserved in bounded raw mode. Fields are optional: `total`; `search.neural/keyword` (legacy breakdown labels); `summary` (synthesized search summary); `contents.text/highlights/summary` (standalone content-type breakdown). Deep cost may appear only in total. Missing cost is unknown, not zero; do not sum incomplete nested fields into a fictional total. Never mix estimated USD with output character/token budgets.

## Null, missing, truncation

Raw schemas retain source-required fields and nullability, but sources conflict. Consumers should record unexpected/missing fields as bounded diagnostics instead of lying about completeness or dropping an otherwise usable source. Request null is structurally permitted for many Search/Contents options; no general null=default semantics is documented, so the proposed adapter omits optional nulls deliberately and reports that policy.

Vendor content caps are characters/tokens, not response byte limits. Dynamic budget distribution and section extraction are best-effort; `verbosity: full` is not a complete-original-page guarantee. The provider adds independent exact serialized output ceilings, native HTTP response ceilings **before buffering**, and explicit truncation metadata. It cannot reduce bytes already buffered by projecting later. See [provider plan](../provider-plan.md) for deterministic projection/truncation, including bounded raw access.
