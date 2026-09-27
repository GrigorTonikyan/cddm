#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Supported MCP generative UI widget identifiers.
pub mod widget_types {
    pub const DIFF_SPLIT_VIEW: &str = "diff-split-view";
    pub const CLUSTER_TREEMAP: &str = "cluster-treemap";
}

/// An interactive generative UI widget returned in MCP tool responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AppWidget {
    #[serde(rename = "type")]
    pub content_type: String,
    pub widget_type: String,
    pub title: String,
    pub html: String,
    pub data: serde_json::Value,
}

impl AppWidget {
    pub fn new(
        widget_type: impl Into<String>,
        title: impl Into<String>,
        html: impl Into<String>,
        data: serde_json::Value,
    ) -> Self {
        Self {
            content_type: "app_widget".to_string(),
            widget_type: widget_type.into(),
            title: title.into(),
            html: html.into(),
            data,
        }
    }
}

/// Descriptor for discoverable MCP Apps widgets.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppWidgetDescriptor {
    pub id: String,
    pub widget_type: String,
    pub title: String,
    pub name: String,
    pub description: String,
    pub supported_modes: Vec<String>,
    pub supported_params: Vec<String>,
}
