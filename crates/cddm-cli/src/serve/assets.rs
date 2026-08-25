#![forbid(unsafe_code)]

use super::types::*;
use axum::{
    body::Body,
    http::{HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../webui/dist"]
pub struct WebUIAssets;

pub async fn static_asset_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let asset_path = if path.is_empty() { INDEX_HTML } else { path };

    match WebUIAssets::get(asset_path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(asset_path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            // SPA fallback to index.html
            match WebUIAssets::get(INDEX_HTML) {
                Some(index_content) => Response::builder()
                    .status(StatusCode::OK)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(MIME_TEXT_HTML),
                    )
                    .body(Body::from(index_content.data))
                    .unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(NOT_FOUND_MSG))
                    .unwrap(),
            }
        }
    }
}
