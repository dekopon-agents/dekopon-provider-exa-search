use dekopon_provider_sdk::clap::{Arg, ArgAction, ArgMatches, Command};
use dekopon_provider_sdk::{CommandInvocation, CommandRun, ProviderError, cli};
use serde_json::{Map, Value, json};

use crate::{ids, validate};

fn usage(message: &str) -> ProviderError {
    ProviderError::new("usage", message)
}
fn list(name: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .action(ArgAction::Append)
        .num_args(1)
}
fn option(name: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .num_args(1)
        .allow_hyphen_values(name == "max-age-hours")
}
fn flag(name: &'static str) -> Arg {
    Arg::new(name).long(name).action(ArgAction::SetTrue)
}
fn query() -> Arg {
    Arg::new("query")
        .value_name("QUERY")
        .help("Search query (omit with --input-json)")
}
fn json_arg() -> Arg {
    option("input-json").value_name("JSON|-").help(
        "Complete typed request JSON; - reads piped stdin. Cannot combine with other input flags",
    )
}
fn extraction(mut cmd: Command) -> Command {
    cmd = cmd
        .arg(flag("text"))
        .arg(flag("highlights"))
        .arg(
            option("contents-json")
                .help("Complete contents options JSON; conflicts with extraction flags"),
        )
        .arg(option("max-age-hours"))
        .arg(option("subpages"));
    cmd
}
fn tree() -> Command {
    let search = extraction(
        Command::new("search")
            .about("Search Exa (including synchronous deep-search and synthesis)")
            .arg(query())
            .arg(json_arg())
            .arg(list("include-domain"))
            .arg(list("exclude-domain"))
            .arg(list("additional-query"))
            .arg(option("type"))
            .arg(option("num-results"))
            .arg(option("start-published-date"))
            .arg(option("end-published-date"))
            .arg(option("category"))
            .arg(option("user-location"))
            .arg(option("compliance"))
            .arg(option("output-schema-json"))
            .arg(option("system-prompt"))
            .arg(flag("moderation")),
    );
    let contents = extraction(
        Command::new("contents")
            .about("Retrieve content by URL (or use --id for document IDs)")
            .arg(Arg::new("urls").value_name("URL").num_args(1..))
            .arg(list("id"))
            .arg(option("compliance"))
            .arg(json_arg()),
    );
    let answer = Command::new("answer")
        .about("Generate a buffered cited answer; no streaming")
        .arg(query())
        .arg(json_arg())
        .arg(flag("text"))
        .arg(option("model"))
        .arg(option("system-prompt"))
        .arg(option("user-location"))
        .arg(option("output-schema-json"));
    Command::new("exa")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Broker-mediated Exa search, content retrieval, and buffered answers")
        .subcommand_required(true)
        .subcommand(search)
        .subcommand(contents)
        .subcommand(answer)
}

pub(crate) fn run(argv: &[String], stdin: Option<&str>) -> Result<CommandRun, ProviderError> {
    cli::run_command(tree(), argv, stdin, dispatch)
}
fn string(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).cloned()
}
fn insert(map: &mut Map<String, Value>, matches: &ArgMatches, name: &str, field: &str) {
    if let Some(value) = string(matches, name) {
        map.insert(field.into(), json!(value));
    }
}
fn insert_list(map: &mut Map<String, Value>, matches: &ArgMatches, name: &str, field: &str) {
    if let Some(values) = matches.get_many::<String>(name) {
        map.insert(field.into(), json!(values.cloned().collect::<Vec<_>>()));
    }
}
fn insert_number(
    map: &mut Map<String, Value>,
    matches: &ArgMatches,
    name: &str,
    field: &str,
) -> Result<(), ProviderError> {
    if let Some(value) = string(matches, name) {
        let number: i64 = value.parse().map_err(|_| usage("expected an integer"))?;
        map.insert(field.into(), json!(number));
    }
    Ok(())
}
fn insert_json(
    map: &mut Map<String, Value>,
    matches: &ArgMatches,
    name: &str,
    field: &str,
) -> Result<(), ProviderError> {
    if let Some(value) = string(matches, name) {
        let parsed: Value =
            serde_json::from_str(&value).map_err(|_| usage("expected valid inline JSON"))?;
        map.insert(field.into(), parsed);
    }
    Ok(())
}
fn insert_flag(map: &mut Map<String, Value>, matches: &ArgMatches, name: &str, field: &str) {
    if matches.get_flag(name) {
        map.insert(field.into(), json!(true));
    }
}
fn contents_options(
    map: &mut Map<String, Value>,
    matches: &ArgMatches,
    nested: bool,
) -> Result<(), ProviderError> {
    if let Some(value) = string(matches, "contents-json") {
        if matches.get_flag("text")
            || matches.get_flag("highlights")
            || string(matches, "max-age-hours").is_some()
            || string(matches, "subpages").is_some()
        {
            return Err(usage("--contents-json conflicts with extraction flags"));
        }
        let options: Value =
            serde_json::from_str(&value).map_err(|_| usage("expected valid contents JSON"))?;
        if !options.is_object() {
            return Err(usage("--contents-json requires an object"));
        }
        // Neither nested nor top-level extraction JSON may smuggle operation fields.
        let _: crate::models::ContentsOptions = serde_json::from_value(options.clone())
            .map_err(|_| usage("--contents-json accepts contents options only"))?;
        if nested {
            map.insert("contents".into(), options);
        } else {
            map.extend(options.as_object().expect("checked object").clone());
        }
    } else {
        let mut options = Map::new();
        insert_flag(&mut options, matches, "text", "text");
        insert_flag(&mut options, matches, "highlights", "highlights");
        insert_number(&mut options, matches, "max-age-hours", "maxAgeHours")?;
        insert_number(&mut options, matches, "subpages", "subpages")?;
        if nested {
            if !options.is_empty() {
                map.insert("contents".into(), Value::Object(options));
            }
        } else {
            map.extend(options);
        }
    }
    Ok(())
}
fn dispatch(matches: ArgMatches, stdin: Option<&str>) -> Result<CommandInvocation, ProviderError> {
    let (id, matches) = match matches.subcommand() {
        Some(("search", sub)) => (ids::SEARCH, sub),
        Some(("contents", sub)) => (ids::CONTENTS, sub),
        Some(("answer", sub)) => (ids::ANSWER, sub),
        _ => return Err(usage("unknown Exa command")),
    };
    let json = string(matches, "input-json");
    let input = if let Some(json_input) = json {
        // Explicitly reject every other supplied input, including positional query or URLs.
        let others = matches.ids().any(|name| {
            name.as_str() != "input-json"
                && matches.value_source(name.as_str())
                    == Some(dekopon_provider_sdk::clap::parser::ValueSource::CommandLine)
        });
        if others {
            return Err(usage(
                "--input-json cannot be combined with flags or positional input",
            ));
        }
        let text = if json_input == "-" {
            stdin.ok_or_else(|| usage("--input-json - requires stdin"))?
        } else {
            &json_input
        };
        serde_json::from_str(text).map_err(|_| usage("--input-json requires valid JSON"))?
    } else {
        let mut map = Map::new();
        match id {
            ids::SEARCH => {
                insert(&mut map, matches, "query", "query");
                insert_list(&mut map, matches, "include-domain", "includeDomains");
                insert_list(&mut map, matches, "exclude-domain", "excludeDomains");
                insert_list(&mut map, matches, "additional-query", "additionalQueries");
                for (flag, field) in [
                    ("type", "type"),
                    ("category", "category"),
                    ("user-location", "userLocation"),
                    ("compliance", "compliance"),
                    ("system-prompt", "systemPrompt"),
                    ("start-published-date", "startPublishedDate"),
                    ("end-published-date", "endPublishedDate"),
                ] {
                    insert(&mut map, matches, flag, field);
                }
                insert_number(&mut map, matches, "num-results", "numResults")?;
                insert_flag(&mut map, matches, "moderation", "moderation");
                insert_json(&mut map, matches, "output-schema-json", "outputSchema")?;
                contents_options(&mut map, matches, true)?;
            }
            ids::CONTENTS => {
                if let Some(urls) = matches.get_many::<String>("urls") {
                    map.insert("urls".into(), json!(urls.cloned().collect::<Vec<_>>()));
                }
                insert_list(&mut map, matches, "id", "ids");
                insert(&mut map, matches, "compliance", "compliance");
                contents_options(&mut map, matches, false)?;
            }
            ids::ANSWER => {
                insert(&mut map, matches, "query", "query");
                insert_flag(&mut map, matches, "text", "text");
                for (flag, field) in [
                    ("model", "model"),
                    ("system-prompt", "systemPrompt"),
                    ("user-location", "userLocation"),
                ] {
                    insert(&mut map, matches, flag, field);
                }
                insert_json(&mut map, matches, "output-schema-json", "outputSchema")?;
            }
            _ => unreachable!(),
        }
        Value::Object(map)
    };
    validate(id, &input)?;
    Ok(CommandInvocation {
        capability: id.parse().expect("static ID"),
        input,
        secret_use: None,
    })
}
