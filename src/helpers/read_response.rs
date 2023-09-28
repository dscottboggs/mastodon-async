use std::{fmt::Debug, time::Duration};

use crate::{as_value, errors::Result, Error};
use futures::pin_mut;
use futures_util::StreamExt;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use tracing::{debug, trace, warn};

/// Adapter for reading JSON data from a response with better logging and a
/// fail-safe timeout.
///
/// The reason for this is largely because there was an issue with responses
/// being received, but not closed, we add a timeout on each read and try
/// to parse whatever we got before the timeout.
pub async fn read_response<T>(response: Response) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Serialize + Debug,
{
    let mut bytes = vec![];
    let status = response.status();
    trace!(
        response = as_value!(response, Response),
        "attempting to stream response"
    );
    let stream = response.bytes_stream();
    pin_mut!(stream);
    loop {
        if let Ok(data) = timeout(Duration::from_secs(10), stream.next()).await {
            // as of here, we did not time out
            let Some(data) = data else {
                break;
            };
            // as of here, we have not hit the end of the stream yet
            let data = data?;
            // as of here, we did not hit an error while reading the body
            bytes.extend_from_slice(&data);
            trace!(
                bytes_received_so_far = bytes.len(),
                data = %String::from_utf8_lossy(&data),
                "data chunk received"
            );
        } else {
            warn!(data_received = bytes.len(), "API response timed out");
            break;
        }
    }
    // done growing the vec, let's just do this once.
    let bytes = bytes.as_slice();
    trace!(data = %String::from_utf8_lossy(bytes), "parsing response");
    if status.is_success() {
        // the the response should deserialize to T
        let result = serde_json::from_slice(bytes)?;
        debug!(?result, "result parsed successfully");
        Ok(result)
    } else {
        // we've received an error message, let's deserialize that instead.
        let response = serde_json::from_slice(bytes)?;
        debug!(?status, ?response, "error received from API");
        Err(Error::Api { status, response })
    }
}
