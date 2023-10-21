use std::io;

use crate::{errors::Result, prelude::*, Error};
use futures::{stream::try_unfold, TryStream, TryStreamExt};
use log::{as_debug, as_serde, debug, error, info, trace};
use reqwest::Response;
use tokio::io::AsyncBufReadExt;
use tokio_util::io::StreamReader;

/// Return a stream of events from the given response by parsing Server-Sent
/// Events as they come in.
///
/// See <https://docs.joinmastodon.org/methods/streaming/> for more info
pub fn event_stream(
    response: Response,
    location: String,
    client: &Mastodon,
) -> impl TryStream<Ok = (Event, Mastodon), Error = Error> + '_ {
    let stream = StreamReader::new(response.bytes_stream().map_err(|err| {
        error!(err = as_debug!(err); "error reading stream");
        io::Error::new(io::ErrorKind::BrokenPipe, format!("{err:?}"))
    }));
    let lines_iter = stream.lines();
    try_unfold((lines_iter, location, client), |mut this| async move {
        let (ref mut lines_iter, ref location, client) = this;
        let mut lines = vec![];
        while let Some(line) = lines_iter.next_line().await? {
            debug!(message = line, location = &location; "received message");
            let line = line.trim().to_string();
            if line.starts_with(':') || line.is_empty() {
                continue;
            }
            lines.push(line);
            if let Ok(event) = make_event(&lines) {
                info!(event = as_serde!(event), location = location; "received event");
                lines.clear();
                return Ok(Some(((event, client.clone()), this)));
            } else {
                continue;
            }
        }
        Ok(None)
    })
}

pub(crate) fn make_event(lines: &[String]) -> Result<Event> {
    let event;
    let data;
    if let Some(event_line) = lines.iter().find(|line| line.starts_with("event:")) {
        event = event_line[6..].trim().to_string();
        data = lines
            .iter()
            .find(|line| line.starts_with("data:"))
            .map(|x| x[5..].trim().to_string());
    } else {
        #[derive(Deserialize)]
        struct Message {
            pub event: String,
            pub payload: Option<String>,
        }
        let message = serde_json::from_str::<Message>(&lines[0])?;
        event = message.event;
        data = message.payload;
    }
    let event: &str = &event;
    trace!(event = event, payload = data; "SSE message parsed");
    Ok(match event {
        "notification" => {
            let data = data
                .ok_or_else(|| Error::Other("Missing `data` line for notification".to_string()))?;
            let notification = serde_json::from_str::<Notification>(&data)?;
            Event::Notification(notification)
        }
        "update" => {
            let data =
                data.ok_or_else(|| Error::Other("Missing `data` line for update".to_string()))?;
            let status = serde_json::from_str::<Status>(&data)?;
            Event::Update(status)
        }
        "delete" => {
            let data =
                data.ok_or_else(|| Error::Other("Missing `data` line for delete".to_string()))?;
            Event::Delete(data)
        }
        "filters_changed" => Event::FiltersChanged,
        _ => return Err(Error::Other(format!("Unknown event `{event}`"))),
    })
}
