#![forbid(unsafe_code)]

pub mod diff_widget;
pub mod handlers;
pub mod treemap_widget;
pub mod types;

pub use diff_widget::generate_diff_split_view_widget;
pub use handlers::{get_supported_app_widgets, handle_apps_render, handle_apps_widgets_list};
pub use treemap_widget::{TreemapClusterItem, generate_cluster_treemap_widget};
pub use types::{AppWidget, AppWidgetDescriptor, widget_types};
