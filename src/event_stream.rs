use crate::{
    entities::{event::Event, prelude::Notification, status::Status},
    errors::Result,
    Error,
};
use futures::{stream::try_unfold, TryStream};
use log::debug;
use tungstenite::Message;

/// Returns a stream of events at the given url location.
pub fn event_stream(
    location: impl AsRef<str>,
) -> Result<impl TryStream<Ok = Event, Error = Error, Item = Result<Event>>> {
    let (client, response) = tungstenite::connect(location.as_ref())?;
    let status = response.status();
    if !status.is_success() {
        return Err(Error::Api(crate::ApiError {
            error: status.canonical_reason().map(String::from),
            error_description: None,
        }));
    }
    Ok(try_unfold(client, |mut client| async move {
        let mut lines = vec![];
        loop {
            match client.read_message() {
                Ok(Message::Text(message)) => {
                    let line = message.trim().to_string();
                    if line.starts_with(":") || line.is_empty() {
                        continue;
                    }
                    lines.push(line);
                    if let Ok(event) = make_event(&lines) {
                        lines.clear();
                        return Ok(Some((event, client)));
                    } else {
                        continue;
                    }
                },
                Ok(Message::Ping(data)) => {
                    debug!("received ping, ponging (metadata: {data:?})");
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
