use crate::{
    entities::{event::Event, prelude::Notification, status::Status},
    errors::Result,
    Error,
};
use futures::{stream::try_unfold, TryStream};
use log::{as_debug, as_serde, debug, error, info, trace};
use tungstenite::Message;

/// Returns a stream of events at the given url location.
pub fn event_stream(
    location: String,
) -> Result<impl TryStream<Ok = Event, Error = Error, Item = Result<Event>>> {
    trace!(location = location; "connecting to websocket for events");
    let (client, response) = tungstenite::connect(&location)?;
    let status = response.status();
    if !status.is_success() {
        error!(
            status = as_debug!(status),
            body = response.body().as_ref().map(|it| String::from_utf8_lossy(it.as_slice())).unwrap_or("(empty body)".into()),
            location = &location;
            "error connecting to websocket"
        );
        return Err(Error::Api(crate::ApiError {
            error: status.canonical_reason().map(String::from),
            error_description: None,
        }));
    }
    debug!(location = &location, status = as_debug!(status); "successfully connected to websocket");
    Ok(try_unfold((client, location), |mut this| async move {
        let (ref mut client, ref location) = this;
        let mut lines = vec![];
        loop {
            match client.read_message() {
                Ok(Message::Text(line)) => {
                    debug!(message = line, location = &location; "received websocket message");
                    let line = line.trim().to_string();
                    if line.starts_with(":") || line.is_empty() {
                        continue;
                    }
                    lines.push(line);
                    if let Ok(event) = make_event(&lines) {
                        info!(event = as_serde!(event), location = location; "received websocket event");
                        lines.clear();
                        return Ok(Some((event, this)));
                    } else {
                        continue;
                    }
                },
                Ok(Message::Ping(data)) => {
                    debug!(metadata = as_serde!(data); "received ping, ponging");
                    client.write_message(Message::Pong(data))?;
                },
                Ok(message) => return Err(message.into()),
                Err(err) => return Err(err.into()),
            }
        }
    }))
}

fn make_event(lines: &[String]) -> Result<Event> {
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
    trace!(event = event, payload = data; "websocket message parsed");
    Ok(match event {
        "notification" => {
            let data = data
                .ok_or_else(|| Error::Other("Missing `data` line for notification".to_string()))?;
            let notification = serde_json::from_str::<Notification>(&data)?;
            Event::Notification(notification)
        },
        "update" => {
            let data =
                data.ok_or_else(|| Error::Other("Missing `data` line for update".to_string()))?;
            let status = serde_json::from_str::<Status>(&data)?;
            Event::Update(status)
        },
        "delete" => {
            let data =
                data.ok_or_else(|| Error::Other("Missing `data` line for delete".to_string()))?;
            Event::Delete(data)
        },
        "filters_changed" => Event::FiltersChanged,
        _ => return Err(Error::Other(format!("Unknown event `{}`", event))),
    })
}
