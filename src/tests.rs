use super::*;
use dekopon_provider_sdk::{CommandRunOutcome, provider};
use dekopon_provider_sdk_testkit::{HttpScript, Native};

fn proposal(args: &[&str], piped: bool, expected: &str) -> Value {
    let argv: Vec<_> = args.iter().map(|s| (*s).to_owned()).collect();
    let CommandRunOutcome::Proposed {
        capability,
        input,
        secret_use,
    } = provider::command::<Exa>(&argv, piped)
    else {
        panic!("expected proposal: {args:?}")
    };
    assert_eq!(capability.as_str(), expected);
    assert!(secret_use.is_none());
    input
}
fn response(body: Value) -> Response {
    Response {
        status: 200,
        headers: vec![],
        body: serde_json::to_vec(&body).unwrap(),
    }
}
#[test]
fn exact_request_headers_shape_and_stdout_for_all_operations() {
    for (id, input, path, body) in [
        (
            ids::SEARCH,
            json!({"query":"orchards","type":"deep-reasoning","additionalQueries":["fruit"],"contents":{"highlights":{"dynamic":true,"verbosity":"medium"},"summary":{"schema":r#"{"properties":{"answer":{"type":"array"}}}"#}},"outputSchema":r#"{"type":"object","$schema":"https://json-schema.org/draft/2020-12/schema","properties":{"answer":{"type":"array"}}}"#}),
            "/search",
            json!({"results":[],"requestId":"abc","future":{"nested":true}}),
        ),
        (
            ids::CONTENTS,
            json!({"ids":["doc"],"text":{"verbosity":"full"},"snapshotAsOf":"2026-01-15"}),
            "/contents",
            json!({"results":[],"statuses":[],"future":true}),
        ),
        (
            ids::ANSWER,
            json!({"query":"Why?","model":"exa-research","outputSchema":r#"{"type":"object","properties":{"answer":{"type":"string"}}}"#}),
            "/answer",
            json!({"answer":{"result":"yes"},"citations":[],"future":1}),
        ),
    ] {
        let mut wire = input.clone();
        expand_schema_strings(id, &mut wire).unwrap();
        let native = Native::<Exa>::new().http(HttpScript::new(
            "api.exa.ai",
            "POST",
            response(body.clone()),
        ));
        let result = native.call(id, &input.to_string());
        assert_eq!(result.status, 0, "{}: {}", id, result.stderr);
        assert_eq!(result.stdout, format!("{}\n", body).as_bytes());
        assert!(result.stderr.is_empty());
        let requests = native.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].uri, format!("https://api.exa.ai{path}"));
        assert_eq!(requests[0].method, "POST");
        assert_eq!(requests[0].body, serde_json::to_vec(&wire).unwrap());
        assert_eq!(
            requests[0]
                .headers
                .iter()
                .map(|h| (h.name.as_str(), h.value.as_slice()))
                .collect::<Vec<_>>(),
            if id == ids::SEARCH {
                vec![
                    ("content-type", b"application/json".as_slice()),
                    ("accept", b"application/json".as_slice()),
                    ("Exa-Beta", b"dynamic-highlights-2026-08-28".as_slice()),
                ]
            } else {
                vec![
                    ("content-type", b"application/json".as_slice()),
                    ("accept", b"application/json".as_slice()),
                ]
            }
        );
    }
}
#[test]
fn marker_only_proposal_and_bounded_stdin_at_invoke() {
    for (args, id, data) in [
        (
            vec!["search", "--input-json", "-"],
            ids::SEARCH,
            json!({"query":"q"}),
        ),
        (
            vec!["contents", "--input-json", "-"],
            ids::CONTENTS,
            json!({"urls":["https://example.org"]}),
        ),
        (
            vec!["answer", "--input-json", "-"],
            ids::ANSWER,
            json!({"query":"q"}),
        ),
    ] {
        let marker = proposal(&args, true, id);
        assert_eq!(marker, json!({"stdin_json":true}));
        let native = Native::<Exa>::new()
            .stdin(data.to_string().into_bytes())
            .http(HttpScript::new(
                "api.exa.ai",
                "POST",
                response(if id == ids::ANSWER {
                    json!({"answer":"yes"})
                } else {
                    json!({"results":[]})
                }),
            ));
        let run = native.call(id, &marker.to_string());
        assert_eq!(run.status, 0, "{}", run.stderr);
        assert_eq!(
            native.requests()[0].body,
            serde_json::to_vec(&data).unwrap()
        );
        for bytes in [
            Vec::new(),
            b"{".to_vec(),
            b"[]".to_vec(),
            vec![b'x'; 65_537],
        ] {
            let native = Native::<Exa>::new().stdin(bytes);
            let run = native.call(id, &marker.to_string());
            assert_ne!(run.status, 0);
            assert!(run.stdout.is_empty());
            assert!(native.requests().is_empty());
        }
    }
    assert!(matches!(
        provider::command::<Exa>(&["search", "--input-json", "-"].map(str::to_owned), false),
        CommandRunOutcome::Failed { .. }
    ));
}
#[test]
fn malformed_schema_and_invalid_input_never_send() {
    for (id, input) in [
        (ids::SEARCH, json!({"query":"q","outputSchema":"[]"})),
        (ids::SEARCH, json!({"query":"q","outputSchema":"{"})),
        (
            ids::SEARCH,
            json!({"query":"q","outputSchema":"x".repeat(65_537)}),
        ),
        (
            ids::SEARCH,
            json!({"query":"q","contents":{"summary":{"schema":"true"}}}),
        ),
        (ids::SEARCH, json!({"query":"q","stdin_json":false})),
        (ids::SEARCH, json!({"query":"q","stdin_json":"invalid"})),
        (ids::SEARCH, json!({"query":"q","stream":true})),
        (ids::SEARCH, json!({"query":"q","additionalQueries":["a"]})),
        (ids::CONTENTS, json!({"ids":["doc"],"urls":["https://x"]})),
        (ids::ANSWER, json!({"query":"q","stream":true})),
        (ids::ANSWER, json!({"query":"q","outputSchema":null})),
    ] {
        let native = Native::<Exa>::new();
        let result = native.call(id, &input.to_string());
        assert_ne!(result.status, 0);
        assert!(result.stdout.is_empty());
        assert!(native.requests().is_empty());
    }
    for (status, body) in [
        (401, b"secret".to_vec()),
        (429, b"secret".to_vec()),
        (503, b"secret".to_vec()),
        (200, b"not-json".to_vec()),
    ] {
        let native = Native::<Exa>::new().http(HttpScript::new(
            "api.exa.ai",
            "POST",
            Response {
                status,
                headers: vec![],
                body,
            },
        ));
        let result = native.call(ids::SEARCH, &json!({"query":"q"}).to_string());
        assert_ne!(result.status, 0);
        assert!(result.stdout.is_empty());
        assert!(!result.stderr.contains("secret"));
        assert_eq!(native.requests().len(), 1);
    }
}
#[test]
fn manifest_is_closed_and_cli_remains_narrow() {
    let manifest = provider::manifest::<Exa>().unwrap();
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
            .all(|c| c.effect == EffectKind::ReadOnly
                && c.risk == RiskLevel::Medium
                && c.input_schema["additionalProperties"] == false)
    );
    assert_eq!(
        manifest.capabilities[0].input_schema["properties"]["outputSchema"]["type"],
        json!(["string", "null"])
    );
    assert_eq!(
        proposal(
            &["search", "q", "--type", "deep", "--num-results", "2"],
            false,
            ids::SEARCH
        )["numResults"],
        2
    );
    assert_eq!(
        proposal(
            &["contents", "--id", "doc", "--max-age-hours", "-1"],
            false,
            ids::CONTENTS
        )["maxAgeHours"],
        -1
    );
    assert_eq!(
        proposal(&["answer", "q", "--model", "exa-pro"], false, ids::ANSWER)["model"],
        "exa-pro"
    );
    let contents = proposal(
        &[
            "contents",
            "--id",
            "doc",
            "--contents-json",
            r#"{"summary":{"schema":{"properties":{"x":{"type":"object"}}}}}"#,
        ],
        false,
        ids::CONTENTS,
    );
    assert_eq!(
        contents["summary"]["schema"],
        r#"{"properties":{"x":{"type":"object"}}}"#
    );
    assert_eq!(
        proposal(
            &[
                "search",
                "q",
                "--output-schema-json",
                r#"{"type":"object","properties":{"x":{"type":"string"}}}"#
            ],
            false,
            ids::SEARCH
        )["outputSchema"],
        r#"{"type":"object","properties":{"x":{"type":"string"}}}"#
    );
    for words in [
        vec!["search", "q", "--input-json", "-"],
        vec!["contents", "--id", "a", "https://x"],
        vec!["answer", "q", "--input-json", "{}"],
    ] {
        assert!(!matches!(
            provider::command::<Exa>(
                &words.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
                true
            ),
            CommandRunOutcome::Proposed { .. }
        ));
    }
}
