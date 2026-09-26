//! Exa search, contents and buffered answers over broker-mediated HTTP.
//! Only fixed paths on api.exa.ai are reachable. Authentication and authority belong to the broker.
mod commands;
mod models;

use dekopon_provider_http::{Header, HttpError, Request, Response, method};
use dekopon_provider_sdk::{
    CapabilityId, CommandRun, EffectKind, Provider, ProviderApiVersion, ProviderCapability,
    ProviderError, ProviderManifest, RiskLevel,
};
use models::{BoolOr, ContentsOptions, StringOrList};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

pub(crate) mod ids {
    pub const SEARCH: &str = "exa.search";
    pub const CONTENTS: &str = "exa.contents";
    pub const ANSWER: &str = "exa.answer";
}

mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "provider",
        generate_all,
        pub_export_macro: true,
    });
}

struct Exa;
impl Provider for Exa {
    fn manifest() -> ProviderManifest {
        ProviderManifest {
            api_version: ProviderApiVersion::V1Alpha1,
            id: "exa".parse().expect("static ID"),
            description: "Fixed Exa search, contents and buffered answer requests".into(),
            command_words: vec!["exa".into()],
            capabilities: [
                (
                    ids::SEARCH,
                    "Search with optional extraction and synchronous synthesis",
                ),
                (ids::CONTENTS, "Retrieve content by document IDs or URLs"),
                (ids::ANSWER, "Generate a buffered cited answer"),
            ]
            .into_iter()
            .map(|(id, description)| ProviderCapability {
                id: id.parse().expect("static capability ID"),
                description: description.into(),
                // Search/answer are billable reads. A grant is necessary even without a write.
                effect: EffectKind::ReadOnly,
                risk: RiskLevel::Medium,
                input_schema: input_schema(id),
            })
            .collect(),
        }
    }
    fn run_command(argv: &[String], stdin: Option<&str>) -> Result<CommandRun, ProviderError> {
        commands::run(argv, stdin)
    }
    fn invoke(capability: &CapabilityId, input: Value) -> Result<Value, ProviderError> {
        invoke_with(capability, input, dekopon_provider_http::send)
    }
}

fn input_schema(id: &str) -> Value {
    let (properties, required) = match id {
        ids::SEARCH => (
            json!({"query": {"type":"string","minLength":1}, "includeDomains":{"type":"array","items":{"type":"string"}}, "excludeDomains":{"type":"array","items":{"type":"string"}}, "startPublishedDate":{"type":"string"}, "endPublishedDate":{"type":"string"}, "numResults":{"type":"integer","minimum":1,"maximum":100}, "moderation":{"type":"boolean"}, "contents":{"type":"object"}, "additionalQueries":{"type":"array"}, "type":{"enum":["instant","fast","auto","deep-lite","deep","deep-reasoning"]}, "category":{"type":"string"}, "userLocation":{"type":"string"}, "compliance":{"enum":["hipaa"]}, "outputSchema":{"type":"object"}, "systemPrompt":{"type":"string"}}),
            vec!["query"],
        ),
        ids::CONTENTS => (
            json!({"ids":{"type":"array","minItems":1,"maxItems":100}, "urls":{"type":"array","minItems":1,"maxItems":100}, "compliance":{"enum":["hipaa"]}, "text":{}, "highlights":{}, "summary":{"type":"object"}, "extras":{"type":"object"}, "livecrawlTimeout":{"type":"integer"}, "maxAgeHours":{"type":"integer"}, "snapshotAsOf":{"type":"string"}, "subpages":{"type":"integer"}, "subpageTarget":{}}),
            vec![],
        ),
        ids::ANSWER => (
            json!({"query":{"type":"string","minLength":1}, "text":{"type":"boolean"}, "model":{"enum":["exa","exa-pro","exa-research","exa-fast"]}, "systemPrompt":{"type":"string"}, "userLocation":{"type":"string"}, "outputSchema":{"type":"object"}}),
            vec!["query"],
        ),
        _ => unreachable!("manifest uses fixed IDs"),
    };
    json!({"type":"object", "properties":properties, "required":required, "additionalProperties":false})
}

fn invalid() -> ProviderError {
    ProviderError::new(
        "invalid-input",
        "input does not match the Exa operation contract",
    )
}
fn decode<T: DeserializeOwned>(value: &Value) -> Result<T, ProviderError> {
    serde_json::from_value(value.clone()).map_err(|_| invalid())
}
fn nonempty(value: &str) -> bool {
    !value.trim().is_empty()
}
fn within(value: Option<u32>, min: u32, max: u32) -> bool {
    value.is_none_or(|number| (min..=max).contains(&number))
}
fn strings(values: &[String], min: usize, max: usize, max_len: usize) -> bool {
    (min..=max).contains(&values.len())
        && values
            .iter()
            .all(|value| nonempty(value) && value.len() <= max_len)
}
fn country(value: Option<&str>) -> bool {
    value.is_none_or(|code| code.len() == 2 && code.bytes().all(|c| c.is_ascii_uppercase()))
}
fn date_time(value: &str) -> Option<time::OffsetDateTime> {
    use time::format_description::well_known::Rfc3339;
    // The pinned spec documents both RFC 3339 and "2023-01-01 00:00:00 UTC".
    let normalized = if let Some(value) = value.strip_suffix(" UTC") {
        format!("{}Z", value.replace(' ', "T"))
    } else {
        value.to_owned()
    };
    time::OffsetDateTime::parse(&normalized, &Rfc3339).ok()
}
fn snapshot_date(value: &str) -> bool {
    date_time(value).is_some()
        || time::format_description::parse_borrowed::<3>("[year]-[month]-[day]")
            .is_ok_and(|format| time::Date::parse(value, &format).is_ok())
}
fn validate_options(options: &ContentsOptions) -> Result<bool, ProviderError> {
    if !within(options.livecrawl_timeout, 1, 90_000)
        || options
            .snapshot_as_of
            .as_deref()
            .is_some_and(|v| !snapshot_date(v))
        || options
            .max_age_hours
            .is_some_and(|n| !(-1..=720).contains(&n))
        || !within(options.subpages, 0, 100)
    {
        return Err(invalid());
    }
    if let Some(target) = &options.subpage_target {
        let valid = match target {
            StringOrList::String(s) => nonempty(s) && s.len() <= 100,
            StringOrList::List(v) => strings(v, 0, 100, 100),
        };
        if !valid {
            return Err(invalid());
        }
    }
    if let Some(BoolOr::Options(text)) = &options.text
        && !within(text.max_characters, 1, 1_000_000)
    {
        return Err(invalid());
    }
    let mut beta = false;
    if let Some(BoolOr::Options(highlights)) = &options.highlights {
        beta = highlights.verbosity.is_some() || highlights.dynamic.is_some();
        if !within(highlights.max_characters, 1, 1_000_000)
            || highlights.dynamic == Some(true) && highlights.max_characters.is_some()
            || highlights.verbosity.is_some() && highlights.max_characters.is_some()
        {
            return Err(invalid());
        }
    }
    if let Some(extras) = &options.extras
        && [
            extras.links,
            extras.image_links,
            extras.rich_image_links,
            extras.rich_links,
            extras.code_blocks,
        ]
        .into_iter()
        .any(|v| !within(v, 0, 1000))
    {
        return Err(invalid());
    }
    Ok(beta)
}

/// Returns whether the request requires Exa's fixed dynamic-highlights beta header.
fn validate(id: &str, input: &Value) -> Result<bool, ProviderError> {
    if !input.is_object() {
        return Err(invalid());
    }
    match id {
        ids::SEARCH => {
            let request: models::Search = decode(input)?;
            if !nonempty(&request.query)
                || !within(request.num_results, 1, 100)
                || request
                    .include_domains
                    .as_ref()
                    .is_some_and(|v| !strings(v, 0, 1200, usize::MAX))
                || request
                    .exclude_domains
                    .as_ref()
                    .is_some_and(|v| !strings(v, 0, 1200, usize::MAX))
                || request
                    .additional_queries
                    .as_ref()
                    .is_some_and(|v| !strings(v, 1, 10, usize::MAX))
                || !country(request.user_location.as_deref())
                || request
                    .start_published_date
                    .as_deref()
                    .is_some_and(|v| date_time(v).is_none())
                || request
                    .end_published_date
                    .as_deref()
                    .is_some_and(|v| date_time(v).is_none())
                || request
                    .start_published_date
                    .as_deref()
                    .zip(request.end_published_date.as_deref())
                    .is_some_and(|(start, end)| date_time(start) > date_time(end))
                || request.category.as_deref().is_some_and(|v| !nonempty(v))
                || request.additional_queries.is_some()
                    && !matches!(
                        request.search_type,
                        Some(
                            models::SearchType::DeepLite
                                | models::SearchType::Deep
                                | models::SearchType::DeepReasoning
                        )
                    )
                || matches!(request.category.as_deref(), Some("people" | "company"))
                    && (request.start_published_date.is_some()
                        || request.end_published_date.is_some()
                        || request.exclude_domains.is_some())
            {
                return Err(invalid());
            }
            request
                .contents
                .as_ref()
                .map_or(Ok(false), validate_options)
        }
        ids::CONTENTS => {
            let request: models::Contents = decode(input)?;
            let fields = input.as_object().expect("object checked");
            if fields.contains_key("ids") == fields.contains_key("urls")
                || fields.get("ids").is_some_and(Value::is_null)
                || fields.get("urls").is_some_and(Value::is_null)
                || request
                    .ids
                    .as_ref()
                    .is_some_and(|v| !strings(v, 1, 100, 2048))
                || request
                    .urls
                    .as_ref()
                    .is_some_and(|v| !strings(v, 1, 100, 2048))
            {
                return Err(invalid());
            }
            let mut options = input.clone();
            let object = options.as_object_mut().expect("object checked");
            object.remove("ids");
            object.remove("urls");
            object.remove("compliance");
            validate_options(&decode::<ContentsOptions>(&options)?)
        }
        ids::ANSWER => {
            let request: models::Answer = decode(input)?;
            if !nonempty(&request.query)
                || request.stream.is_some_and(|v| v)
                || !country(request.user_location.as_deref())
            {
                return Err(invalid());
            }
            Ok(false)
        }
        _ => Err(ProviderError::new(
            "unknown-capability",
            "unsupported Exa capability",
        )),
    }
}

fn invoke_with(
    capability: &CapabilityId,
    input: Value,
    mut send: impl FnMut(Request) -> Result<Response, HttpError>,
) -> Result<Value, ProviderError> {
    let id = capability.as_str();
    let path = match id {
        ids::SEARCH => "/search",
        ids::CONTENTS => "/contents",
        ids::ANSWER => "/answer",
        _ => {
            return Err(ProviderError::new(
                "unknown-capability",
                "unsupported Exa capability",
            ));
        }
    };
    let beta = validate(id, &input)?;
    let header = |name, value| {
        Header::text(name, value)
            .map_err(|_| ProviderError::new("invalid-request", "could not construct Exa request"))
    };
    let mut request = Request::new(method::POST, format!("https://api.exa.ai{path}"))
        .map_err(|_| ProviderError::new("invalid-request", "could not construct Exa request"))?
        .with_header(header("content-type", "application/json")?)
        .with_header(header("accept", "application/json")?)
        .with_body(serde_json::to_vec(&input).map_err(|_| invalid())?);
    if beta {
        request = request.with_header(header("Exa-Beta", "dynamic-highlights-2026-08-28")?);
    }
    let response = send(request)
        .map_err(|_| ProviderError::new("http-failed", "broker HTTP request failed"))?;
    if response.status != 200 {
        let code = match response.status {
            401 => "unauthorized",
            403 => "forbidden",
            429 => "rate-limited",
            400 | 422 => "unprocessable",
            _ => "unexpected-status",
        };
        return Err(ProviderError::new(code, "Exa refused the request"));
    }
    let output: Value = serde_json::from_slice(&response.body)
        .map_err(|_| ProviderError::new("invalid-response", "Exa returned invalid JSON"))?;
    let valid = match id {
        ids::ANSWER => output
            .get("answer")
            .is_some_and(|value| value.is_string() || value.is_object()),
        ids::SEARCH => output.get("results").and_then(Value::as_array).is_some(),
        ids::CONTENTS => output.get("results").and_then(Value::as_array).is_some(),
        _ => false,
    };
    if !valid {
        return Err(ProviderError::new(
            "invalid-response",
            "Exa returned an invalid response",
        ));
    }
    Ok(output)
}

dekopon_provider_sdk::export_provider_with_cli!(Exa, bindings);

#[cfg(test)]
mod tests;
