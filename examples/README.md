# Synthetic examples and fixtures

**All request scenarios and response values are synthetic**, designed from public documented shapes. They were not sent to Exa or captured from Exa. Example domains are reserved illustrative sources; dates, counts, confidence, costs and timings are arbitrary plausible fixture values, not observed measurements, pricing guarantees or implementation promises.

Each operation (`search`, `contents`, `context`, adjacent `answer`) has minimal and feature-rich request files and a realistic response file. Minimal Contents intentionally requests only URLs; add explicit text/highlights for guaranteed intent. Feature-rich examples demonstrate many compatible controls, not every alternative in one conflicting request. Field inventories cover alternatives and restrictions. Search/Contents synthetic response demonstrates rich extras, subpages, nullable authors and costs; Contents includes one failed URL on a nominally successful batch. Search demonstrates structured synthesis with grounding. Structured page summary is represented as a JSON-encoded **string**, matching the current wire schema.

`headers-stable.json` and `headers-beta.json` model fixed public metadata, **not auth**. `provider-*.json` use proposed provider namespaces, including explicit beta opt-in. They are not current CLI invocations. `provider-output.json` is a proposed envelope example, not an upstream wire response.

[fixtures.json](fixtures.json) declares each fixture's local schema and whether acceptance or rejection is expected. Files under `invalid/` are intentional invalid/conflicting provider requests; JSON syntax remains valid. The validator checks that every standalone fixture is registered and validated. [Test limitations](../docs/validation.md) distinguish schema checks from future runtime tests.

Representative files:

- [Minimal Search](search-minimal.json), [rich Search](search-rich.json), [response](search-response.json)
- [Minimal Contents](contents-minimal.json), [rich Contents](contents-rich.json), [mixed outcome](contents-response.json)
- [Minimal Context](context-minimal.json), [rich Context](context-rich.json), [response](context-response.json)
- [Minimal Answer](answer-minimal.json), [rich Answer](answer-rich.json), [response](answer-response.json)
- [Synthetic company/person/publication entities](search-entities-response.json)
- [Provider beta](provider-search-beta.json), [provider output](provider-output.json)
