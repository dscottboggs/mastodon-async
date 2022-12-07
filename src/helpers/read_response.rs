use std::time::Duration;

use crate::errors::Result;
use futures::pin_mut;
use futures_util::StreamExt;
use log::{as_serde, debug, trace, warn};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

/// Adapter for reading JSON data from a response with better logging and a
/// fail-safe timeout.
pub async fn read_response<T>(response: Response) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let mut bytes = vec![];
    let url = response.url().clone();
    // let status = log_serde!(response Status);
    // let headers = log_serde!(response Headers);
    let stream = response.bytes_stream();
    pin_mut!(stream);
    loop {
        if let Ok(data) = timeout(Duration::from_secs(10), stream.next()).await {
            // as of here, we did not time out
            let Some(data) = data else { break; };
            // as of here, we have not hit the end of the stream yet
            let data = data?;
            // as of here, we did not hit an error while reading the body
            bytes.extend_from_slice(&data);
            debug!(
                data = String::from_utf8_lossy(&data), url = url.as_str(),
                bytes_received_so_far = bytes.len();
                "data chunk received"
            );
        } else {
            warn!(
                url = url.as_str(), // status = status, headers = headers,
                data_received = bytes.len();
                "API response timed out"
            );
            break;
        }
    }
    trace!(
        url = url.as_str(), // status = status, headers = headers,
        data_received = bytes.len();
        "parsing response"
    );
    let result = serde_json::from_slice(bytes.as_slice())?;
    debug!(
        url = url.as_str(), // status = status, headers = headers,
        result = as_serde!(result);
        "result parsed successfully"
    );
    Ok(result)
}
