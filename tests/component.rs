//! Real production component under the SDK broker host; no paid network calls.
use dekopon_exa_search_provider::Exa;
use dekopon_provider_sdk::provider;
use dekopon_provider_sdk_testkit::{Harness, conformance};
use serde_json::json;
use std::path::PathBuf;

fn component() -> PathBuf {
    std::env::var_os("DEKOPON_PROVIDER_COMPONENT")
        .expect("build component and set DEKOPON_PROVIDER_COMPONENT")
        .into()
}

#[test]
fn real_component_conforms_and_denies_ungranted_http() -> Result<(), Box<dyn std::error::Error>> {
    let path = component();
    conformance::<Exa>(&path)?;
    let manifest = provider::manifest::<Exa>()?;
    assert_eq!(
        manifest
            .capabilities
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["exa.search", "exa.contents", "exa.answer"]
    );
    for (capability, input) in [
        ("exa.search", json!({"query":"unpaid test"})),
        ("exa.contents", json!({"urls":["https://example.org"]})),
        ("exa.answer", json!({"query":"unpaid test"})),
    ] {
        assert!(
            Harness::<Exa>::get(&path).call(capability, input).is_err(),
            "no HTTP grant"
        );
    }
    for input in [
        json!({"query":"q","stream":true}),
        json!({"stdin_json":true,"query":"q"}),
        json!({"query":"q","outputSchema":"[]"}),
    ] {
        let run = Harness::<Exa>::get(&path).call("exa.search", input)?;
        assert_ne!(run.status, 0);
        assert!(run.stdout.is_empty());
        assert!(run.http_calls.is_empty());
    }
    for bytes in [
        Vec::new(),
        b"{".to_vec(),
        b"[]".to_vec(),
        vec![b'x'; 65_537],
    ] {
        let run = Harness::<Exa>::get(&path)
            .stdin(bytes)
            .call("exa.search", json!({"stdin_json":true}))?;
        assert_ne!(run.status, 0);
        assert!(run.stdout.is_empty());
        assert!(run.http_calls.is_empty());
    }
    assert_eq!(Harness::<Exa>::compiled_identities(), 1);
    Ok(())
}

#[test]
fn real_component_uses_owner_base_and_rejects_invalid_settings()
-> Result<(), Box<dyn std::error::Error>> {
    use dekopon_provider_sdk::provider::Response;
    use dekopon_provider_sdk_testkit::HttpScript;
    for settings in [
        json!({"baseUrl":"https://example.test?query=1"}),
        json!({"baseUrl":"https://user@example.test"}),
        json!({"baseUrl":"https://example.test#fragment"}),
        json!({"baseUrl":"ftp://example.test"}),
        json!({"baseUrl":"example.test"}),
        json!({"baseUrl":17}),
        json!({"baseUrl":null}),
        json!({"unknown":true}),
    ] {
        let run = Harness::<Exa>::get(component())
            .settings(settings)
            .call("exa.search", json!({"query":"orchards"}))?;
        assert_ne!(run.status, 0);
        assert!(
            run.stderr
                .contains("the provider settings do not match their schema"),
            "{}",
            run.stderr
        );
        assert!(run.stdout.is_empty());
        assert!(run.http_calls.is_empty());
    }
    for (id, input, body) in [
        (
            "exa.search",
            json!({"query":"orchards"}),
            json!({"results":[]}),
        ),
        (
            "exa.contents",
            json!({"urls":["https://example.org"]}),
            json!({"results":[]}),
        ),
        (
            "exa.answer",
            json!({"query":"orchards"}),
            json!({"answer":"synthetic"}),
        ),
    ] {
        let owner = Harness::<Exa>::get(component()).http(HttpScript::new(
            "localhost",
            "POST",
            Response {
                status: 200,
                headers: vec![],
                body: serde_json::to_vec(&body)?,
            },
        ));
        let origin = owner.origin().expect("script origin").to_owned();
        let run = owner
            .settings(json!({"baseUrl":format!("{origin}/proxy/exa")}))
            .call(id, input)?;
        assert_eq!(run.status, 0, "{}", run.stderr);
        assert_eq!(run.http_calls.len(), 1);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&run.stdout)?,
            body
        );
    }
    Ok(())
}
