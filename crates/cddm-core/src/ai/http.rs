#![forbid(unsafe_code)]

use super::constants::*;
use std::time::Duration;

/// Constructs a configured asynchronous HTTP client with connection pooling and timeouts.
pub fn create_http_client(timeout_secs: Option<u64>) -> reqwest::Client {
    let timeout = Duration::from_secs(timeout_secs.unwrap_or(DEFAULT_PROVIDER_TIMEOUT_SECS));
    reqwest::Client::builder()
        .timeout(timeout)
        .connect_timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .user_agent(DEFAULT_USER_AGENT)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

/// Dispatches an asynchronous HTTP POST request with retry backoff for transient failures.
pub async fn execute_http_post(
    client: &reqwest::Client,
    url: &str,
    extra_headers: &[(&str, &str)],
    payload: &serde_json::Value,
) -> Result<String, String> {
    let mut last_err = String::new();
    let mut backoff = Duration::from_millis(DEFAULT_INITIAL_BACKOFF_MS);

    for attempt in 1..=DEFAULT_MAX_RETRIES {
        let mut req = client
            .post(url)
            .header(HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
            .json(payload);

        for (k, v) in extra_headers {
            req = req.header(*k, *v);
        }

        match req.send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    return resp
                        .text()
                        .await
                        .map_err(|e| format!("Failed to read response body: {e}"));
                }

                let status_code = status.as_u16();
                let err_text = resp.text().await.unwrap_or_default();
                last_err = format!("HTTP {status_code}: {err_text}");

                let is_transient = status_code == 429
                    || status_code == 500
                    || status_code == 502
                    || status_code == 503
                    || status_code == 504;

                if !is_transient || attempt == DEFAULT_MAX_RETRIES {
                    return Err(last_err);
                }
            }
            Err(e) => {
                last_err = format!("HTTP request error: {e}");
                if attempt == DEFAULT_MAX_RETRIES {
                    return Err(last_err);
                }
            }
        }

        tokio::time::sleep(backoff).await;
        backoff = std::cmp::min(backoff * 2, Duration::from_millis(DEFAULT_MAX_BACKOFF_MS));
    }

    Err(last_err)
}

/// Sends an HTTP POST and extracts a text field from the JSON response.
pub async fn post_and_extract(
    client: &reqwest::Client,
    url: &str,
    headers: &[(&str, &str)],
    payload: &serde_json::Value,
    extract: impl FnOnce(&serde_json::Value) -> Option<&str>,
) -> Result<String, String> {
    let resp_str = execute_http_post(client, url, headers, payload).await?;
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&resp_str)
        && let Some(text) = extract(&val)
    {
        return Ok(text.to_string());
    }
    Ok(resp_str)
}

/// Helper to execute an HTTP chat completion request.
pub async fn execute_http_chat(
    client: &reqwest::Client,
    url: &str,
    headers: &[(&str, &str)],
    payload: &serde_json::Value,
    extract_fn: impl Fn(&serde_json::Value) -> Option<&str>,
) -> Result<String, String> {
    post_and_extract(client, url, headers, payload, extract_fn).await
}

/// Helper to execute an HTTP streaming chat request with Server-Sent Events or chunked transfer.
pub async fn execute_http_chat_stream<F>(
    client: &reqwest::Client,
    url: &str,
    headers: &[(&str, &str)],
    payload: &serde_json::Value,
    extract_delta_fn: F,
) -> Result<super::provider::stream::BoxAiStream, String>
where
    F: Fn(&str) -> Option<String> + Send + Sync + 'static,
{
    let mut req = client
        .post(url)
        .header(HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
        .json(payload);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("HTTP request error: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status.as_u16(), err_text));
    }

    let (tx, rx) = tokio::sync::mpsc::channel(64);
    tokio::spawn(async move {
        use tokio_stream::StreamExt;
        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();
        while let Some(chunk_res) = stream.next().await {
            match chunk_res {
                Ok(bytes) => {
                    let chunk_str = String::from_utf8_lossy(&bytes);
                    buffer.push_str(&chunk_str);
                    while let Some(pos) = buffer.find('\n') {
                        let line = buffer[..pos].trim_end_matches('\r').to_string();
                        buffer = buffer[pos + 1..].to_string();
                        if let Some(token) = extract_delta_fn(&line)
                            && tx.send(Ok(token)).await.is_err()
                        {
                            return;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(format!("Stream read error: {e}"))).await;
                    return;
                }
            }
        }
        if !buffer.trim().is_empty()
            && let Some(token) = extract_delta_fn(buffer.trim())
        {
            let _ = tx.send(Ok(token)).await;
        }
    });

    Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_http_client() {
        let client_default = create_http_client(None);
        let client_custom = create_http_client(Some(15));
        assert!(format!("{client_default:?}").contains("Client"));
        assert!(format!("{client_custom:?}").contains("Client"));
    }

    #[tokio::test]
    async fn test_execute_http_post_unreachable() {
        let client = create_http_client(Some(2));
        let payload = serde_json::json!({"test": true});
        let res = execute_http_post(&client, "http://127.0.0.1:59999", &[], &payload).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.contains("HTTP request error") || err.contains("connection"));
    }
}
