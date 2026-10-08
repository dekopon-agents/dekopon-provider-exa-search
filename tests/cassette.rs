use dekopon_exa_search_provider::Exa;
use dekopon_provider_sdk::provider::{Header, Response};
use dekopon_provider_sdk_testkit::{HttpScript, Native};
use serde_json::{Value, json};

#[test]
fn authored_contents_cassette_replays_default_and_prefixed_base() {
    let exchange: Value =
        serde_json::from_str(include_str!("cassettes/exa/0001-POST-contents.json")).unwrap();
    assert_eq!(exchange["version"], 1);
    let request = &exchange["request"];
    let response = &exchange["response"];
    for (settings, host, base) in [
        (None, "api.exa.ai", "https://api.exa.ai"),
        (Some(json!({})), "api.exa.ai", "https://api.exa.ai"),
        (
            Some(json!({"baseUrl":"https://fixture.example.test/proxy/exa%20api/"})),
            "fixture.example.test",
            "https://fixture.example.test/proxy/exa%20api",
        ),
    ] {
        let mut native = Native::<Exa>::new().http(HttpScript::new(
            host,
            request["method"].as_str().unwrap(),
            Response {
                status: response["status"].as_u64().unwrap().try_into().unwrap(),
                headers: vec![Header::text("content-type", "application/json").unwrap()],
                body: serde_json::to_vec(&response["body"]["json"]).unwrap(),
            },
        ));
        if let Some(settings) = settings {
            native = native.settings(settings);
        }
        let output = native.call("exa.contents", &request["body"]["json"].to_string());
        assert_eq!(output.status, 0, "{}", output.stderr);
        let sent = native.requests();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].method, request["method"]);
        assert!(request["query"].is_null());
        assert_eq!(
            sent[0].uri,
            format!("{base}{}", request["path"].as_str().unwrap())
        );
        for (name, value) in request["headers"].as_object().unwrap() {
            let header = sent[0]
                .headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case(name))
                .unwrap();
            assert_eq!(header.value, value.as_str().unwrap().as_bytes());
        }
        assert!(
            !sent[0]
                .headers
                .iter()
                .any(|header| header.name.eq_ignore_ascii_case("authorization"))
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&sent[0].body).unwrap(),
            request["body"]["json"]
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
            response["body"]["json"]
        );
    }
}
