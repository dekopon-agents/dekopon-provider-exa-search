# Exa synthetic cassette

`0001-POST-contents.json` is an authored cassette v1 fixture, derived from the scripted contents case in `src/tests.rs::exact_request_headers_shape_and_stdout_for_all_operations`. It is not a vendor recording. It uses only synthetic content, with no credentials, cookies, account identifiers or household data. The content URL query is request-body data; Exa API requests have no URI query. Replay asserts the exact URI, including the configured prefix and absence of a query, and preserves the provider's unchanged JSON output.
