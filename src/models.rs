//! Handwritten request shapes from Exa OpenAPI 2.0.0 (pinned spec SHA-256
//! 0c7eafdb8baac39ddfdab53e82c8053868bbbb2a289f9ee856524dcbef41a5c7).
//! Deserialize for validation, but forward the original JSON: Option<T> cannot preserve
//! explicit null, and JSON Schema values must never be reserialized through lossy models.
#![allow(dead_code)] // Wire fields are inspected by serde; only cross-field constraints are read here.

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct Search {
    pub query: String,
    pub include_domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub start_published_date: Option<String>,
    pub end_published_date: Option<String>,
    pub num_results: Option<u32>,
    pub moderation: Option<bool>,
    pub contents: Option<ContentsOptions>,
    pub additional_queries: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub search_type: Option<SearchType>,
    pub category: Option<String>,
    pub user_location: Option<String>,
    pub compliance: Option<Compliance>,
    pub output_schema: Option<SearchOutputSchema>,
    pub system_prompt: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum SearchType {
    Instant,
    Fast,
    Auto,
    DeepLite,
    Deep,
    DeepReasoning,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Compliance {
    Hipaa,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum SearchOutputSchema {
    Text {
        description: Option<String>,
    },
    Object {
        description: Option<String>,
        properties: Option<serde_json::Map<String, Value>>,
        required: Option<Vec<String>>,
        #[serde(rename = "additionalProperties")]
        additional_properties: Option<bool>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Contents {
    pub ids: Option<Vec<String>>,
    pub urls: Option<Vec<String>>,
    pub compliance: Option<Compliance>,
}

// Contents is decoded by removing ids/urls/compliance first because serde's deny_unknown_fields
// cannot correctly enforce unknowns on a flattened struct.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct ContentsOptions {
    pub text: Option<BoolOr<TextOptions>>,
    pub highlights: Option<BoolOr<HighlightOptions>>,
    pub summary: Option<SummaryOptions>,
    pub extras: Option<ExtrasOptions>,
    pub livecrawl_timeout: Option<u32>,
    pub max_age_hours: Option<i32>,
    pub snapshot_as_of: Option<String>,
    pub subpages: Option<u32>,
    pub subpage_target: Option<StringOrList>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum BoolOr<T> {
    Bool(bool),
    Options(T),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum StringOrList {
    String(String),
    List(Vec<String>),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct TextOptions {
    pub max_characters: Option<u32>,
    pub include_html_tags: Option<bool>,
    pub verbosity: Option<TextVerbosity>,
    pub include_sections: Option<Vec<Section>>,
    pub exclude_sections: Option<Vec<Section>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TextVerbosity {
    Compact,
    Standard,
    Full,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Section {
    Header,
    Navigation,
    Banner,
    Body,
    Sidebar,
    Footer,
    Metadata,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct HighlightOptions {
    pub query: Option<String>,
    pub verbosity: Option<HighlightVerbosity>,
    pub dynamic: Option<bool>,
    pub max_characters: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum HighlightVerbosity {
    Low,
    Medium,
    High,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SummaryOptions {
    pub query: Option<String>,
    pub schema: Option<serde_json::Map<String, Value>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct ExtrasOptions {
    pub links: Option<u32>,
    pub image_links: Option<u32>,
    pub rich_image_links: Option<u32>,
    pub rich_links: Option<u32>,
    pub code_blocks: Option<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct Answer {
    pub query: String,
    pub stream: Option<bool>,
    pub text: Option<bool>,
    pub model: Option<AnswerModel>,
    pub system_prompt: Option<String>,
    pub user_location: Option<String>,
    pub output_schema: Option<AnswerOutputSchema>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AnswerOutputSchema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub properties: Option<serde_json::Map<String, Value>>,
    pub required: Option<Vec<String>>,
    pub description: Option<String>,
    pub additional_properties: Option<bool>,
    #[serde(flatten)]
    pub extensions: serde_json::Map<String, Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AnswerModel {
    Exa,
    ExaPro,
    ExaResearch,
    ExaFast,
}
