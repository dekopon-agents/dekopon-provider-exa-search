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
