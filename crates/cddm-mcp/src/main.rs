#![forbid(unsafe_code)]

mod prompts;
mod protocol;
mod resources;
mod server;
mod tools;

#[cfg(test)]
mod tests;

use protocol::{JsonRpcRequest, make_error_response, rpc_errors};
use server::handle_mcp_request;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cddm_core::logging::init_default_logging();

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    while let Ok(Some(line)) = reader.next_line().await {
        let line_str = line.trim();
        if line_str.is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(line_str) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = make_error_response(
                    None,
                    rpc_errors::PARSE_ERROR,
                    format!("Parse error: {}", e),
                );
                if let Ok(json_str) = serde_json::to_string(&err_resp) {
                    let mut payload = json_str.into_bytes();
                    payload.push(b'\n');
                    let _ = stdout.write_all(&payload).await;
                    let _ = stdout.flush().await;
                }
                continue;
            }
        };

        if let Some(response) = handle_mcp_request(req).await
            && let Ok(json_str) = serde_json::to_string(&response)
        {
            let mut payload = json_str.into_bytes();
            payload.push(b'\n');
            let _ = stdout.write_all(&payload).await;
            let _ = stdout.flush().await;
        }
    }

    Ok(())
}
