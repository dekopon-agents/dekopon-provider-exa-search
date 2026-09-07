# Small offline documentation gate

Run the [README commands](../README.md#documentation-checks). The only development dependencies are pinned `jsonschema==4.25.1` and `PyYAML==6.0.2` in [requirements-dev.txt](../requirements-dev.txt). PyYAML supports authoring/source inspection, not runtime vendor access. No global installation, Rust build, search request, API key or secret store is needed. Dependency installation may contact the package registry; the validation script itself is offline and resolves only repository-local schema references.

[scripts/validate_docs.py](../scripts/validate_docs.py) checks:

1. Standalone JSON syntax and JSON-labeled Markdown fences.
2. Every schema's Draft 2020-12 well-formedness and local reference/JSON pointer integrity using the pinned JSON Schema implementation.
3. Every registered valid fixture against its declared schema, including date-time formats; every invalid fixture must be rejected. No standalone example may escape the fixture manifest.
4. Relative Markdown file links and heading anchors, without crawling external URLs.
5. Coverage dispositions, unique endpoint/location/path records, documentation/schema pointers, endpoint documentation existence, and exact named-body-path equality against each local documented request schema (plus guide-only deprecated useAutoprompt). This checks internal snapshot completeness, not drift from current vendor pages.

Representative invalid tests cover missing query, non-deep additional queries, people/date conflicts, missing beta opt-in, dynamic/character conflict, streaming, both Contents identifier lists, oversized text, deprecated livecrawl, Context token range, and unknown native fields.

## What schemas enforce vs documentation only

| Condition | Gate today |
|---|---|
| Native structural types/ranges/enums, explicit nullability | Upstream documented schemas, fixture checked |
| Contents exactly one ids/urls | Upstream oneOf, invalid fixture checked |
| Deep-only additionalQueries, people/company forbidden filters | Proposed provider schema, representative invalid fixtures |
| Beta caller opt-in and dynamic/maxCharacters conflict | Proposed provider schema, invalid fixtures |
| Highlight verbosity conflict with maxCharacters/numSentences | Proposed schema; specific runtime coverage still needed |
| Deprecated model options, stream=true, native compliance, unknown top-level fields | Proposed provider schema rejects; representative fixtures, not every permutation |
| Output option ranges and Context allowed modes | Proposed schema; exact envelope data is only a documented subset |
| ISO country membership, publication-date ordering, section/domain overlap | **Documentation-only** future typed validation |
| Search generated schema depth ≤2 and total properties ≤10, supported JSON Schema semantics | **Documentation-only** future explicit validation; arbitrary user schema contents not proven supported upstream |
| Operator entitlements/ceilings, HIPAA/vendor enablement, beta operator permission | **Documentation-only**; no account checks or operator configuration executed |
| Unknown category hints/boolean summary source conflicts | Deferred rather than claimed as wire support; see sources |
| Native HTTP response/request/deadline ceilings, broker Bearer injection | **Not runtime-tested**; future cross-boundary acceptance required |
| Exact UTF-8 serialized byte limits, grounding preservation, deterministic truncation, bounded raw behavior | **Documentation-only** algorithm; output subset schema cannot prove these |
| Current vendor availability/terms, costs/freshness, missing fields or extra response fields | **Unverified live behavior**; offline fixtures are not API tests |

The [docs workflow](../.github/workflows/docs.yml) has read-only contents permissions and checkout pinned to a full commit SHA. It uses system Python plus a local virtualenv, no vendor network or credentials in checks. Local authoring additionally runs `actionlint` and `git diff --check`. No CI execution is claimed until maintainers publish and observe the exact head.

Before committing, check the tracked/public candidate files for credentials, private organization wiring and absolute machine-local paths; source scratch stays ignored. This repository intentionally contains no private provenance/authoring logs. Required acceptance includes the actual gate output and review of material source conflicts, not only this document's assertion that checks exist.
