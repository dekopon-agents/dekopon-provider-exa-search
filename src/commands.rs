use crate::{Answer, Contents, Exa, OperationInput, Search, expand_schema_strings, ids, validate};
use clap::{Args as ClapArgs, Parser, Subcommand};
use dekopon_provider_sdk::provider::{Proposal, Usage};
use serde_json::{Map, Value, json};

#[derive(Parser)]
#[command(
    name = "exa",
    version,
    about = "Broker-mediated Exa search, content retrieval, and buffered answers"
)]
pub struct Args {
    #[command(subcommand)]
    operation: Operation,
}
#[derive(Subcommand)]
enum Operation {
    Search(Box<SearchArgs>),
    Contents(Box<ContentsArgs>),
    Answer(Box<AnswerArgs>),
}
#[derive(ClapArgs)]
struct SearchArgs {
    query: Option<String>,
    #[arg(long)]
    input_json: Option<String>,
    #[command(flatten)]
    extraction: Extraction,
    #[arg(long)]
    include_domain: Vec<String>,
    #[arg(long)]
    exclude_domain: Vec<String>,
    #[arg(long)]
    additional_query: Vec<String>,
    #[arg(long)]
    r#type: Option<String>,
    #[arg(long)]
    num_results: Option<i64>,
    #[arg(long)]
    start_published_date: Option<String>,
    #[arg(long)]
    end_published_date: Option<String>,
    #[arg(long)]
    category: Option<String>,
    #[arg(long)]
    user_location: Option<String>,
    #[arg(long)]
    compliance: Option<String>,
    #[arg(long)]
    output_schema_json: Option<String>,
    #[arg(long)]
    system_prompt: Option<String>,
    #[arg(long)]
    moderation: bool,
}
#[derive(ClapArgs)]
struct ContentsArgs {
    #[arg(num_args=0..)]
    urls: Vec<String>,
    #[arg(long = "id")]
    ids: Vec<String>,
    #[arg(long)]
    compliance: Option<String>,
    #[arg(long)]
    input_json: Option<String>,
    #[command(flatten)]
    extraction: Extraction,
}
#[derive(ClapArgs)]
struct AnswerArgs {
    query: Option<String>,
    #[arg(long)]
    input_json: Option<String>,
    #[arg(long)]
    text: bool,
    #[arg(long)]
    model: Option<String>,
    #[arg(long)]
    system_prompt: Option<String>,
    #[arg(long)]
    user_location: Option<String>,
    #[arg(long)]
    output_schema_json: Option<String>,
}
#[derive(ClapArgs)]
struct Extraction {
    #[arg(long)]
    text: bool,
    #[arg(long)]
    highlights: bool,
    #[arg(long)]
    contents_json: Option<String>,
    #[arg(long, allow_hyphen_values = true)]
    max_age_hours: Option<i64>,
    #[arg(long)]
    subpages: Option<i64>,
}
impl Extraction {
    fn supplied(&self) -> bool {
        self.text
            || self.highlights
            || self.contents_json.is_some()
            || self.max_age_hours.is_some()
            || self.subpages.is_some()
    }
}
fn usage(message: &'static str) -> Usage {
    Usage::new(message)
}
fn put(map: &mut Map<String, Value>, key: &str, value: Option<impl Into<Value>>) {
    if let Some(value) = value {
        map.insert(key.into(), value.into());
    }
}
fn list(map: &mut Map<String, Value>, key: &str, value: Vec<String>) {
    if !value.is_empty() {
        map.insert(key.into(), json!(value));
    }
}
fn json_field(map: &mut Map<String, Value>, key: &str, value: Option<String>) -> Result<(), Usage> {
    if let Some(value) = value {
        if value.len() > 65_536
            || !serde_json::from_str::<Value>(&value).is_ok_and(|v| v.is_object())
        {
            return Err(usage("expected bounded JSON object"));
        }
        map.insert(key.into(), Value::String(value));
    }
    Ok(())
}
fn extraction(map: &mut Map<String, Value>, args: Extraction, nested: bool) -> Result<(), Usage> {
    let mut options = Map::new();
    if let Some(text) = args.contents_json {
        if args.text || args.highlights || args.max_age_hours.is_some() || args.subpages.is_some() {
            return Err(usage("--contents-json conflicts with extraction flags"));
        }
        let value: Value =
            serde_json::from_str(&text).map_err(|_| usage("expected valid contents JSON"))?;
        if !value.is_object()
            || serde_json::from_value::<crate::models::ContentsOptions>(value.clone()).is_err()
        {
            return Err(usage("--contents-json accepts contents options only"));
        }
        options = value.as_object().expect("checked object").clone();
        // Preserve --contents-json's object syntax while the typed proposal uses a string leaf.
        if let Some(schema) = options.get_mut("summary").and_then(|v| v.get_mut("schema")) {
            let text =
                serde_json::to_string(schema).map_err(|_| usage("invalid summary schema"))?;
            if text.len() > 65_536 || !schema.is_object() {
                return Err(usage("invalid summary schema"));
            }
            *schema = Value::String(text);
        }
    } else {
        if args.text {
            options.insert("text".into(), json!(true));
        }
        if args.highlights {
            options.insert("highlights".into(), json!(true));
        }
        put(&mut options, "maxAgeHours", args.max_age_hours);
        put(&mut options, "subpages", args.subpages);
    }
    if nested {
        if !options.is_empty() {
            map.insert("contents".into(), Value::Object(options));
        }
    } else {
        map.extend(options);
    }
    Ok(())
}
fn input_json(
    text: Option<String>,
    others: bool,
    stdin_piped: bool,
) -> Result<Option<Value>, Usage> {
    let Some(text) = text else {
        return Ok(None);
    };
    if others {
        return Err(usage(
            "--input-json cannot be combined with flags or positional input",
        ));
    }
    if text == "-" {
        if !stdin_piped {
            return Err(usage("--input-json - requires stdin"));
        }
        return Ok(Some(json!({"stdin_json":true})));
    }
    Ok(Some(
        serde_json::from_str(&text).map_err(|_| usage("--input-json requires valid JSON"))?,
    ))
}
fn checked(id: &str, input: &Value) -> Result<(), Usage> {
    if input == &json!({"stdin_json":true}) {
        return Ok(());
    }
    let mut expanded = input.clone();
    expand_schema_strings(id, &mut expanded)
        .and_then(|()| validate(id, &expanded))
        .map_err(|_| usage("input does not match the Exa operation contract"))?;
    Ok(())
}
pub fn propose(args: Args, stdin_piped: bool) -> Result<Proposal<Exa>, Usage> {
    match args.operation {
        Operation::Search(a) => {
            let others = a.query.is_some()
                || a.extraction.supplied()
                || !a.include_domain.is_empty()
                || !a.exclude_domain.is_empty()
                || !a.additional_query.is_empty()
                || a.r#type.is_some()
                || a.num_results.is_some()
                || a.start_published_date.is_some()
                || a.end_published_date.is_some()
                || a.category.is_some()
                || a.user_location.is_some()
                || a.compliance.is_some()
                || a.output_schema_json.is_some()
                || a.system_prompt.is_some()
                || a.moderation;
            let input = if let Some(value) = input_json(a.input_json, others, stdin_piped)? {
                value
            } else {
                let mut map = Map::new();
                put(&mut map, "query", a.query);
                list(&mut map, "includeDomains", a.include_domain);
                list(&mut map, "excludeDomains", a.exclude_domain);
                list(&mut map, "additionalQueries", a.additional_query);
                put(&mut map, "type", a.r#type);
                put(&mut map, "numResults", a.num_results);
                put(&mut map, "startPublishedDate", a.start_published_date);
                put(&mut map, "endPublishedDate", a.end_published_date);
                put(&mut map, "category", a.category);
                put(&mut map, "userLocation", a.user_location);
                put(&mut map, "compliance", a.compliance);
                put(&mut map, "systemPrompt", a.system_prompt);
                json_field(&mut map, "outputSchema", a.output_schema_json)?;
                if a.moderation {
                    map.insert("moderation".into(), json!(true));
                }
                extraction(&mut map, a.extraction, true)?;
                Value::Object(map)
            };
            checked(ids::SEARCH, &input)?;
            Ok(Proposal::to::<Search>(OperationInput::new(input)))
        }
        Operation::Contents(a) => {
            let others = !a.urls.is_empty()
                || !a.ids.is_empty()
                || a.compliance.is_some()
                || a.extraction.supplied();
            let input = if let Some(value) = input_json(a.input_json, others, stdin_piped)? {
                value
            } else {
                let mut map = Map::new();
                list(&mut map, "urls", a.urls);
                list(&mut map, "ids", a.ids);
                put(&mut map, "compliance", a.compliance);
                extraction(&mut map, a.extraction, false)?;
                Value::Object(map)
            };
            checked(ids::CONTENTS, &input)?;
            Ok(Proposal::to::<Contents>(OperationInput::new(input)))
        }
        Operation::Answer(a) => {
            let others = a.query.is_some()
                || a.text
                || a.model.is_some()
                || a.system_prompt.is_some()
                || a.user_location.is_some()
                || a.output_schema_json.is_some();
            let input = if let Some(value) = input_json(a.input_json, others, stdin_piped)? {
                value
            } else {
                let mut map = Map::new();
                put(&mut map, "query", a.query);
                if a.text {
                    map.insert("text".into(), json!(true));
                }
                put(&mut map, "model", a.model);
                put(&mut map, "systemPrompt", a.system_prompt);
                put(&mut map, "userLocation", a.user_location);
                json_field(&mut map, "outputSchema", a.output_schema_json)?;
                Value::Object(map)
            };
            checked(ids::ANSWER, &input)?;
            Ok(Proposal::to::<Answer>(OperationInput::new(input)))
        }
    }
}
