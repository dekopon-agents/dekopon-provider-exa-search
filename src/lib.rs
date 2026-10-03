//! Exa search, contents and buffered answers over broker-mediated HTTP.
//! Only fixed paths on api.exa.ai are reachable. Authentication and authority belong to the broker.
mod commands;
mod models;

use dekopon_provider_sdk::provider::{
    self, Capability, Code, Failure, Header, Http, HttpError, Proposal, Provider, Request,
    Response, Stdout, Usage, method,
};
use dekopon_provider_sdk::{EffectKind, RiskLevel};
use models::{BoolOr, ContentsOptions, StringOrList};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fmt,
    io::{Read, Write},
};

pub(crate) mod ids {
    pub const SEARCH: &str = "exa.search";
    pub const CONTENTS: &str = "exa.contents";
    pub const ANSWER: &str = "exa.answer";
}

pub struct Exa;
pub struct Search;
pub struct Contents;
pub struct Answer;

impl Provider for Exa {
    const ID: &'static str = "exa";
    const COMMAND_WORDS: &'static [&'static str] = &["exa"];
    const DESCRIPTION: &'static str = "Fixed Exa search, contents and buffered answer requests";
    type Args = commands::Args;
    type Capabilities = (Search, Contents, Answer);
    fn propose(args: Self::Args, stdin_piped: bool) -> Result<Proposal<Self>, Usage> {
        commands::propose(args, stdin_piped)
    }
}

#[derive(Debug)]
pub struct ProviderError {
    code: Code,
    message: &'static str,
}
impl ProviderError {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            code: Code::new(code),
            message,
        }
    }
    fn usage(message: &'static str) -> Self {
        Self {
            code: Code::USAGE,
            message,
        }
    }
}
impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message)
    }
}
impl Failure for ProviderError {
    fn code(&self) -> Code {
        self.code
    }
}

// Operation is statically selected; its validated request retains explicit JSON nulls.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationInput<T> {
    value: Value,
    #[serde(skip)]
    marker: std::marker::PhantomData<T>,
}
impl<T> OperationInput<T> {
    fn new(value: Value) -> Self {
        Self {
            value,
            marker: std::marker::PhantomData,
        }
    }
}
macro_rules! schema_for {
    ($type:ident, $id:expr) => {
        impl schemars::JsonSchema for OperationInput<$type> {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($type).into()
            }
            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schema($id)
            }
        }
    };
}
fn schema(id: &str) -> schemars::Schema {
    let value = input_schema(id);
    schemars::Schema::from(value.as_object().expect("static schema").clone())
}
schema_for!(Search, ids::SEARCH);
schema_for!(Contents, ids::CONTENTS);
schema_for!(Answer, ids::ANSWER);
macro_rules! operation {
    ($type:ident, $name:literal, $description:literal, $id:expr) => {
        impl Capability for $type {
            type Provider = Exa;
            const NAME: &'static str = $name;
            const DESCRIPTION: &'static str = $description;
            const EFFECT: EffectKind = EffectKind::ReadOnly;
            const RISK: RiskLevel = RiskLevel::Medium;
            type Input = OperationInput<$type>;
            type Needs = Http;
            type Error = ProviderError;
            fn run(input: Self::Input, http: Http, out: &mut Stdout) -> Result<(), ProviderError> {
                run_operation($id, input.value, http, out)
            }
        }
    };
}
operation!(
    Search,
    "search",
    "Search with optional extraction and synchronous synthesis",
    ids::SEARCH
);
operation!(
    Contents,
    "contents",
    "Retrieve content by document IDs or URLs",
    ids::CONTENTS
);
operation!(
    Answer,
    "answer",
    "Generate a buffered cited answer",
    ids::ANSWER
);

#[allow(unsafe_code)]
mod export {
    dekopon_provider_sdk::export!(super::Exa);
}

fn input_schema(id: &str) -> Value {
    let (properties, required) = match id {
        ids::SEARCH => (
            json!({
                "query": {"type":"string","minLength":1},
                "includeDomains": {"type":["array","null"],"items":{"type":"string"}},
                "excludeDomains": {"type":["array","null"],"items":{"type":"string"}},
                "startPublishedDate": {"type":["string","null"]},
                "endPublishedDate": {"type":["string","null"]},
                "numResults": {"type":["integer","null"],"minimum":1,"maximum":100},
                "moderation": {"type":["boolean","null"]},
                "contents": {"type":["object","null"]},
                "additionalQueries": {"type":["array","null"]},
                "type": {"enum":["instant","fast","auto","deep-lite","deep","deep-reasoning",null]},
                "category": {"type":["string","null"]},
                "userLocation": {"type":["string","null"]},
                "compliance": {"enum":["hipaa",null]},
                "outputSchema": {"type":["object","null"]},
                "systemPrompt": {"type":["string","null"]}
            }),
            vec!["query"],
        ),
        ids::CONTENTS => (
            json!({
                "ids":{"type":"array","minItems":1,"maxItems":100},
                "urls":{"type":"array","minItems":1,"maxItems":100},
                "compliance":{"enum":["hipaa",null]},
                "text":{"type":["boolean","object","null"]},
                "highlights":{"type":["boolean","object","null"]},
                "summary":{"type":["object","null"]},
                "extras":{"type":["object","null"]},
                "livecrawlTimeout":{"type":["integer","null"]},
                "maxAgeHours":{"type":["integer","null"]},
                "snapshotAsOf":{"type":["string","null"]},
                "subpages":{"type":["integer","null"]},
                "subpageTarget":{"type":["string","array","null"]}
            }),
            vec![],
        ),
        ids::ANSWER => (
            json!({
                "query":{"type":"string","minLength":1},
                "text":{"type":"boolean"},
                "model":{"enum":["exa","exa-pro","exa-research","exa-fast"]},
                "systemPrompt":{"type":"string"},
                "userLocation":{"type":["string","null"]},
                "outputSchema":{"type":"object"}
            }),
            vec!["query"],
        ),
        _ => unreachable!("manifest uses fixed IDs"),
    };
    let mut schema = json!({"type":"object", "properties":properties, "required":required, "additionalProperties":false});
    let fields = schema["properties"].as_object_mut().expect("static fields");
    fields.insert("stdin_json".into(), json!({"type":"boolean"}));
    // A marker-only proposal cannot contain a query; all expanded inputs still validate it.
    if id != ids::CONTENTS {
        schema["required"] = json!([]);
    }
    let fields = schema["properties"].as_object_mut().expect("static fields");
    let object =
        |props: Value| json!({"type":"object","properties":props,"additionalProperties":false});
    let nullable = |shape: Value| json!({"anyOf":[shape,{"type":"null"}]});
    let string_null = json!({"type":["string","null"], "maxLength":65536});
    let schema_string = json!({"type":"string", "maxLength":65536});
    let text = object(json!({
        "maxCharacters":{"type":"integer"}, "includeHtmlTags":{"type":"boolean"},
        "verbosity":{"type":"string"}, "includeSections":{"type":"array","items":{"type":"string"}},
        "excludeSections":{"type":"array","items":{"type":"string"}}
    }));
    let highlights = object(json!({
        "query":{"type":"string"}, "verbosity":{"type":"string"},
        "dynamic":{"type":"boolean"}, "maxCharacters":{"type":"integer"}
    }));
    let summary = object(json!({"query":{"type":"string"}, "schema":schema_string}));
    let extras = object(json!({
        "links":{"type":"integer"}, "imageLinks":{"type":"integer"},
        "richImageLinks":{"type":"integer"}, "richLinks":{"type":"integer"},
        "codeBlocks":{"type":"integer"}
    }));
    let options = object(json!({
        "text":{"anyOf":[{"type":"boolean"},text,{"type":"null"}]},
        "highlights":{"anyOf":[{"type":"boolean"},highlights,{"type":"null"}]},
        "summary":nullable(summary),"extras":nullable(extras),
        "livecrawlTimeout":{"type":["integer","null"]},
        "maxAgeHours":{"type":["integer","null"]},
        "snapshotAsOf":string_null,"subpages":{"type":["integer","null"]},
        "subpageTarget":{"anyOf":[{"type":"string"},{"type":"array","items":{"type":"string"}},{"type":"null"}]}
    }));
    match id {
        ids::SEARCH => {
            fields.insert("contents".into(), nullable(options));
            fields.insert(
                "outputSchema".into(),
                json!({"type":["string","null"],"maxLength":65536}),
            );
        }
        ids::CONTENTS => {
            for (key, shape) in options["properties"].as_object().expect("options").iter() {
                fields.insert(key.clone(), shape.clone());
            }
        }
        ids::ANSWER => {
            fields.insert(
                "outputSchema".into(),
                json!({"type":"string","maxLength":65536}),
            );
        }
        _ => unreachable!(),
    }
    schema
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

/// Serde's Option accepts explicit null for a present field; the OpenAPI output-schema
/// roots allow omission but do not allow null for their declared members.
fn output_schema_fields_valid(schema: &Value) -> bool {
    [
        "type",
        "description",
        "properties",
        "required",
        "additionalProperties",
    ]
    .iter()
    .all(|field| schema.get(*field).is_none_or(|value| !value.is_null()))
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
                || input
                    .get("outputSchema")
                    .is_some_and(|schema| !schema.is_null() && !output_schema_fields_valid(schema))
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
                || input.get("outputSchema").is_some_and(|schema| {
                    !schema.is_object() || !output_schema_fields_valid(schema)
                })
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
    id: &str,
    mut input: Value,
    mut send: impl FnMut(Request) -> Result<Response, HttpError>,
) -> Result<Value, ProviderError> {
    expand_schema_strings(id, &mut input)?;
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

fn expand_schema_strings(id: &str, input: &mut Value) -> Result<(), ProviderError> {
    fn expand(slot: &mut Value) -> Result<(), ProviderError> {
        let text = slot.as_str().ok_or_else(invalid)?;
        if text.len() > 65_536 {
            return Err(invalid());
        }
        let object: Value = serde_json::from_str(text).map_err(|_| invalid())?;
        if !object.is_object() {
            return Err(invalid());
        }
        *slot = object;
        Ok(())
    }
    let Some(root) = input.as_object_mut() else {
        return Err(invalid());
    };
    if matches!(id, ids::SEARCH | ids::ANSWER)
        && let Some(slot) = root.get_mut("outputSchema")
        && (!slot.is_null() || id == ids::ANSWER)
    {
        expand(slot)?;
    }
    let options = if id == ids::SEARCH {
        root.get_mut("contents")
    } else {
        Some(input)
    };
    if let Some(slot) = options
        .and_then(|v| v.get_mut("summary"))
        .and_then(|v| v.get_mut("schema"))
    {
        expand(slot)?;
    }
    Ok(())
}

fn run_operation(
    id: &str,
    mut input: Value,
    http: Http,
    out: &mut Stdout,
) -> Result<(), ProviderError> {
    if let Some(object) = input.as_object_mut()
        && object.get("stdin_json") == Some(&Value::Bool(true))
    {
        object.remove("stdin_json");
        if !object.is_empty() {
            return Err(invalid());
        }
        let mut stdin = provider::stdin()
            .ok_or_else(|| ProviderError::usage("--input-json - requires stdin"))?;
        let mut raw = Vec::new();
        stdin
            .by_ref()
            .take(65_537)
            .read_to_end(&mut raw)
            .map_err(|_| ProviderError::usage("--input-json - requires valid JSON"))?;
        if raw.is_empty() {
            return Err(ProviderError::usage(
                "--input-json - requires nonempty JSON",
            ));
        }
        if raw.len() > 65_536 {
            return Err(ProviderError::usage("--input-json - exceeds input limit"));
        }
        input = serde_json::from_slice(&raw)
            .map_err(|_| ProviderError::usage("--input-json - requires valid JSON"))?;
    }
    let output = invoke_with(id, input, |request| http.send(request))?;
    serde_json::to_writer(&mut *out, &output)
        .map_err(|_| ProviderError::new("output-failed", "could not write Exa response"))?;
    out.write_all(b"\n")
        .map_err(|_| ProviderError::new("output-failed", "could not write Exa response"))?;
    Ok(())
}

#[cfg(test)]
mod tests;
