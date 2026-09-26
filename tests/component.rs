//! Real production component under the SDK broker host; no paid network calls.
//! HTTP success uses synthetic injected-send unit fixtures because the SDK testkit cannot
//! substitute an HTTPS response for the fixed production origin.
use dekopon_provider_sdk_testkit::{CommandRunOutcome, FakeBroker};
use serde_json::json;

fn argv(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).into()).collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn component_exports_narrow_surface_and_denies_ungranted_http()
-> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::var("DEKOPON_PROVIDER_COMPONENT")
        .expect("build component and set DEKOPON_PROVIDER_COMPONENT");
    let broker = FakeBroker::builder()
        .component(path)
        .provider("exa")
        .build()
        .await?;
    let manifests: Vec<_> = broker.registry().manifests().collect();
    assert_eq!(manifests.len(), 1);
    assert_eq!(manifests[0].command_words, ["exa"]);
    assert_eq!(
        manifests[0]
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
        let error = broker
            .invoke(capability, input)
            .await
            .expect_err("HTTP grant omitted");
        assert!(
            format!("{error:?}").contains("HostCallRejected"),
            "{error:?}"
        );
    }
    let error = broker
        .invoke("exa.search", json!({"query":"q","stream":true}))
        .await
        .expect_err("invalid before HTTP");
    assert_eq!(
        error.provider_failure().map(|(code, _)| code),
        Some("invalid-input")
    );
    let error = broker
        .invoke("exa.management", json!({}))
        .await
        .expect_err("not in manifest");
    assert!(error.provider_failure().is_none());
    for words in [
        &["--help"][..],
        &["search", "--help"],
        &["answer", "--help"],
    ] {
        match broker.run_command("exa", &argv(words), None).await? {
            CommandRunOutcome::Rendered {
                stdout, status: 0, ..
            } => assert!(stdout.contains("Usage:"), "{stdout}"),
            other => panic!("help must not reach HTTP: {other:?}"),
        }
    }
    for words in [
        &["search"][..],
        &["missing"],
        &["answer", "q", "--stream"],
        &["contents", "--id", "x", "https://x"],
    ] {
        match broker.run_command("exa", &argv(words), None).await? {
            CommandRunOutcome::Rendered { status: 2, .. } | CommandRunOutcome::Failed { .. } => {}
            other => panic!("bad input must not reach HTTP: {other:?}"),
        }
    }
    for (words, capability) in [
        (vec!["search", "q", "--type", "deep"], "exa.search"),
        (vec!["contents", "--id", "doc-1"], "exa.contents"),
        (vec!["answer", "why?", "--model", "exa-pro"], "exa.answer"),
    ] {
        match broker.run_command("exa", &argv(&words), None).await? {
            CommandRunOutcome::Proposed {
                capability: proposed,
                input,
                ..
            } => {
                assert_eq!(proposed.as_str(), capability);
                let error = broker
                    .invoke(capability, input)
                    .await
                    .expect_err("no HTTP grant");
                assert!(
                    format!("{error:?}").contains("HostCallRejected"),
                    "{error:?}"
                );
            }
            other => panic!("expected proposal {words:?}: {other:?}"),
        }
    }
    Ok(())
}
