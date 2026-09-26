use super::*;
use dekopon_provider_sdk::Provider;
use serde_json::json;

fn id(id: &str) -> CapabilityId {
    id.parse().unwrap()
}
fn response(body: Value) -> Result<Response, HttpError> {
    Ok(Response {
        status: 200,
        headers: vec![],
        body: serde_json::to_vec(&body).unwrap(),
    })
}
fn call(id_name: &str, input: Value, body: Value, expected: &str) -> Value {
    validate(id_name, &input).unwrap_or_else(|error| panic!("{id_name} {input}: {error:?}"));
    invoke_with(&id(id_name), input.clone(), |request| {
        assert_eq!(request.method, "POST");
        assert_eq!(request.uri, format!("https://api.exa.ai{expected}"));
        assert!(
            request
                .headers
                .iter()
                .any(|h| h.name.eq_ignore_ascii_case("content-type")
                    && h.value == b"application/json")
        );
        assert_eq!(
            request
                .headers
                .iter()
                .any(|h| h.name.eq_ignore_ascii_case("exa-beta")),
            input.pointer("/contents/highlights/dynamic").is_some()
                || input.pointer("/contents/highlights/verbosity").is_some()
                || input.pointer("/highlights/dynamic").is_some()
                || input.pointer("/highlights/verbosity").is_some()
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&request.body).unwrap(),
            input
        );
        assert!(
            request
                .headers
                .iter()
                .all(|h| !h.name.eq_ignore_ascii_case("authorization"))
        );
        response(body.clone())
    })
    .unwrap()
}
#[test]
fn three_fixed_operations_keep_upstream_json() {
    let search = json!({"query":"orchards", "type":"deep-reasoning", "additionalQueries":["fruit"], "numResults":10, "includeDomains":["example.org/docs"], "moderation":true, "contents":{"highlights":{"dynamic":true,"verbosity":"medium"},"text":{"maxCharacters":1000,"includeSections":["body"]},"summary":{"schema":{"properties":{"nested":{"items":[{"score":1}]}}}},"extras":{"richLinks":1},"maxAgeHours":0,"subpageTarget":["method"]},"outputSchema":{"type":"object","properties":{"answer":{"type":"array","items":{"type":"object","properties":{"url":{"type":"string"}}}}}}});
    let fixture = json!({"requestId":"abc","results":[{"url":"https://example.org","id":"source","future":{"nested":true}}],"output":{"content":{"answer":[{"url":"https://example.org"}]},"grounding":[{"citations":[{"url":"https://example.org"}]}]},"costDollars":{"total":1.1},"future":77});
    assert_eq!(
        call(ids::SEARCH, search, fixture.clone(), "/search"),
        fixture
    );
    let contents = json!({"ids":["doc-1"],"text":{"verbosity":"full"},"highlights":{"query":"fruit"},"summary":{"query":"key points"},"snapshotAsOf":"2026-01-15","subpages":1});
    let fixture = json!({"results":[{"id":"doc-1","url":"https://example.org","text":"hello","future":true}],"statuses":[],"costDollars":{},"requestId":"id"});
    assert_eq!(
        call(ids::CONTENTS, contents, fixture.clone(), "/contents"),
        fixture
    );
    let answer = json!({"query":"Why orchards?", "model":"exa-research", "text":true, "systemPrompt":"cite sources", "outputSchema":{"type":"object","properties":{"result":{"type":"string"}}}});
    let fixture = json!({"answer":{"result":"reason"},"citations":[{"url":"https://example.org"}],"requestId":"id","future":true});
    assert_eq!(
        call(ids::ANSWER, answer, fixture.clone(), "/answer"),
        fixture
    );
}
#[test]
fn invalid_input_and_unknown_capabilities_never_send() {
    for (cap, input) in [
        (ids::SEARCH, json!({"query":""})),
        (ids::SEARCH, json!({"query":"q","stream":true})),
        (
            ids::SEARCH,
            json!({"query":"q","contents":{"context":true}}),
        ),
        (ids::SEARCH, json!({"query":"q","type":"neural"})),
        (ids::SEARCH, json!({"query":"q","additionalQueries":["a"]})),
        (
            ids::SEARCH,
            json!({"query":"q","outputSchema":{"type":"object","properties":[]}}),
        ),
        (
            ids::SEARCH,
            json!({"query":"q","contents":{"highlights":{"dynamic":true,"maxCharacters":1}}}),
        ),
        (ids::CONTENTS, json!({"ids":["doc"],"urls":["https://a"]})),
        (ids::CONTENTS, json!({"ids":[]})),
        (ids::CONTENTS, json!({"ids":null,"urls":["https://a"]})),
        (
            ids::SEARCH,
            json!({"query":"q","startPublishedDate":"not a date"}),
        ),
        (
            ids::SEARCH,
            json!({"query":"q","startPublishedDate":"2026-02-01T00:00:00Z","endPublishedDate":"2026-01-01T00:00:00Z"}),
        ),
        (
            ids::CONTENTS,
            json!({"urls":["https://a"],"snapshotAsOf":"yesterday"}),
        ),
        (
            ids::CONTENTS,
            json!({"urls":["https://a"],"highlights":{"numSentences":2}}),
        ),
        (
            ids::SEARCH,
            json!({"query":"q","startCrawlDate":"2026-01-01T00:00:00Z"}),
        ),
        (ids::ANSWER, json!({"query":"q","Authorization":"x"})),
        (
            ids::CONTENTS,
            json!({"urls":["https://a"],"livecrawl":"always"}),
        ),
        (
            ids::CONTENTS,
            json!({"urls":["https://a"],"extras":{"links":1001}}),
        ),
        (ids::ANSWER, json!({"query":"q","stream":true})),
        (ids::ANSWER, json!({"query":"q","model":"invalid"})),
        (
            ids::ANSWER,
            json!({"query":"q","outputSchema":{"type":false,"properties":[]}}),
        ),
        (
            ids::ANSWER,
            json!({"query":"q","outputSchema":{"type":"object","properties":[]}}),
        ),
        (
            ids::ANSWER,
            json!({"query":"q","outputSchema":{"type":"object","required":[3]}}),
        ),
        (
            ids::ANSWER,
            json!({"query":"q","outputSchema":{"additionalProperties":"yes"}}),
        ),
        (
            ids::ANSWER,
            json!({"query":"q","outputSchema":{"type":null}}),
        ),
        (ids::ANSWER, json!({"query":"q","outputSchema":null})),
        ("exa.management", json!({})),
    ] {
        let error =
            invoke_with(&id(cap), input, |_| panic!("invalid input must not send")).unwrap_err();
        assert!(
            matches!(error.code(), "invalid-input" | "unknown-capability"),
            "{error:?}"
        );
    }
}
#[test]
fn root_output_schema_extensions_are_preserved_and_known_types_are_checked() {
    let search_schema = json!({"type":"object","$schema":"http://json-schema.org/draft-07/schema#","examples":[{"value":17}],"properties":{"nested":{"type":"object","properties":{"score":{"type":"number"}}}}});
    let search_input = json!({"query":"q","outputSchema":search_schema});
    call(
        ids::SEARCH,
        search_input.clone(),
        json!({"results":[],"output":{"content":{}}}),
        "/search",
    );
    let answer_schema = json!({"type":"object","$schema":"http://json-schema.org/draft-07/schema#","properties":{"result":{"type":"string"}},"required":["result"],"additionalProperties":false});
    call(
        ids::ANSWER,
        json!({"query":"q","outputSchema":answer_schema}),
        json!({"answer":{"result":"ok"}}),
        "/answer",
    );
    use dekopon_provider_sdk::CommandRun;
    let cli = |args: &[&str]| {
        commands::run(
            &args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
            None,
        )
    };
    let search_json = search_input.to_string();
    let CommandRun::Proposal(proposal) = cli(&["search", "--input-json", &search_json]).unwrap()
    else {
        panic!("search input-json must propose");
    };
    assert_eq!(proposal.input, search_input);
    let schema_json = search_schema.to_string();
    let CommandRun::Proposal(proposal) =
        cli(&["search", "q", "--output-schema-json", &schema_json]).unwrap()
    else {
        panic!("search output-schema-json must propose");
    };
    assert_eq!(proposal.input["outputSchema"], search_schema);
    let bad_schema = r#"{"type":false,"properties":[]}"#;
    for args in [
        vec!["answer", "q", "--output-schema-json", bad_schema],
        vec![
            "answer",
            "--input-json",
            r#"{"query":"q","outputSchema":{"type":false,"properties":[]}}"#,
        ],
    ] {
        assert!(
            cli(&args).is_err(),
            "malformed outputSchema must not propose: {args:?}"
        );
    }
}

#[test]
fn status_transport_and_malformed_response() {
    for (status, code) in [
        (401, "unauthorized"),
        (403, "forbidden"),
        (429, "rate-limited"),
        (422, "unprocessable"),
        (503, "unexpected-status"),
    ] {
        let error = invoke_with(&id(ids::ANSWER), json!({"query":"hi"}), |_| {
            Ok(Response {
                status,
                headers: vec![],
                body: b"secret".to_vec(),
            })
        })
        .unwrap_err();
        assert_eq!(error.code(), code);
        assert!(!error.message().contains("secret"));
    }
    let error = invoke_with(&id(ids::SEARCH), json!({"query":"hi"}), |_| {
        Ok(Response {
            status: 200,
            headers: vec![],
            body: b"no".to_vec(),
        })
    })
    .unwrap_err();
    assert_eq!(error.code(), "invalid-response");
    let error = invoke_with(&id(ids::ANSWER), json!({"query":"hi"}), |_| {
        response(json!({"results":[]}))
    })
    .unwrap_err();
    assert_eq!(error.code(), "invalid-response");
    let error = invoke_with(&id(ids::CONTENTS), json!({"ids":["doc"]}), |_| {
        Err(HttpError {
            code: dekopon_provider_http::HttpErrorCode::Denied,
            message: "sensitive host detail".into(),
        })
    })
    .unwrap_err();
    assert_eq!(error.code(), "http-failed");
    assert!(!error.message().contains("sensitive"));
}
#[test]
fn manifest_declares_only_these_capabilities() {
    let manifest = Exa::manifest();
    assert_eq!(manifest.command_words, ["exa"]);
    assert_eq!(
        manifest
            .capabilities
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        [ids::SEARCH, ids::CONTENTS, ids::ANSWER]
    );
    assert!(
        manifest
            .capabilities
            .iter()
            .all(|c| c.input_schema["additionalProperties"] == false)
    );
}
#[test]
fn cli_proposes_validated_input_without_egress() {
    use dekopon_provider_sdk::CommandRun;
    let run = |args: &[&str], stdin: Option<&str>| {
        commands::run(
            &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            stdin,
        )
    };
    for (args, expected) in [
        (
            vec![
                "search",
                "q",
                "--type",
                "deep",
                "--num-results",
                "2",
                "--include-domain",
                "example.org",
                "--text",
            ],
            ids::SEARCH,
        ),
        (
            vec!["contents", "https://example.org", "--max-age-hours", "-1"],
            ids::CONTENTS,
        ),
        (
            vec!["contents", "--id", "doc-1", "--highlights"],
            ids::CONTENTS,
        ),
        (vec!["answer", "why?", "--model", "exa-pro"], ids::ANSWER),
    ] {
        let CommandRun::Proposal(invocation) = run(&args, None).unwrap() else {
            panic!("expected invocation {args:?}")
        };
        assert_eq!(invocation.capability.as_str(), expected);
        validate(expected, &invocation.input).unwrap();
    }
    let CommandRun::Proposal(invocation) = run(&["search","--input-json","-"], Some(r#"{"query":"q","contents":{"summary":{"schema":{"properties":{"x":{"type":"object"}}}}}}"#)).unwrap() else {panic!("JSON input");};
    assert_eq!(
        invocation.input["contents"]["summary"]["schema"]["properties"]["x"]["type"],
        "object"
    );
    assert!(
        run(
            &["search", "q", "--input-json", r#"{"query":"other"}"#],
            None
        )
        .is_err()
    );
    assert!(run(&["contents", "--id", "a", "https://a"], None).is_err());
    assert!(run(&["answer", "q", "--input-json", r#"{"query":"q"}"#], None).is_err());
    assert!(
        run(
            &[
                "contents",
                "https://a",
                "--contents-json",
                r#"{"urls":["https://other"]}"#
            ],
            None
        )
        .is_err()
    );
    assert!(
        run(
            &[
                "search",
                "q",
                "--contents-json",
                r#"{"highlights":true}"#,
                "--highlights"
            ],
            None
        )
        .is_err()
    );
    let optional = json!({"query":"q","moderation":null,"contents":{"text":null,"summary":{"schema":{"properties":{"nested":{"type":"array"}}}}}});
    assert_eq!(
        call(ids::SEARCH, optional, json!({"results":[]}), "/search"),
        json!({"results":[]})
    );
    for args in [
        &["--help"][..],
        &["search", "--help"],
        &["unknown"],
        &["answer"],
    ] {
        assert!(matches!(
            run(args, None),
            Ok(CommandRun::Rendered { .. }) | Err(_)
        ));
    }
}
