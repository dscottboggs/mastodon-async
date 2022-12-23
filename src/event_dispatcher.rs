macro_rules! dispatch_events {
    ($handler_name:ident, on update: $update_action:expr, on notification: $notif_action:expr, on delete: $delete_action:expr, on filters_changed: $fc_action:expr,) => {
      async fn $handler_name(client: &Mastodon, event: Event) -> Result<()> {
        match event {
          Event::Update(status) => $update_action,
          Event::Notification(notification) => $notif_action,
          Event::Delete(id) => $delete_action,
          Event::FiltersChanged => $fc_action,
        }
      }
    };
    ($handler_name:ident) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on update: $action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $action,
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on notification: $action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: $action,
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on delete: $action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: $action,
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on filters_changed: $action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: $action,
      }
    };
    ($handler_name:ident, on update: $update_action:expr, on notification: $notif_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $update_action,
        on notification: $notif_action,
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on update: $update_action:expr, on delete: $delete_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $update_action,
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: $delete_action,
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on update: $update_action:expr, on filters_changed: $fc_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $update_action,
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: $fc_action,
      }
    };
    ($handler_name:ident, on notification: $notif_action:expr, on delete: $delete_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: $notif_action,
        on delete: $delete_action,
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on notification: $notif_action:expr, on filters_changed: $fc_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: $notif_action,
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: $fc_action,
      }
    };
    ($handler_name:ident, on delete: $delete_action:expr, on filters_changed: $fc_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: $delete_action,
        on filters_changed: $fc_action,
      }
    };
    ($handler_name:ident, on update: $update_action:expr, on notification: $notif_action:expr, on delete: $delete_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $update_action,
        on notification: $notif_action,
        on delete: $delete_action,
        on filters_changed: Ok(log::info!("filter change notification received")),
      }
    };
    ($handler_name:ident, on update: $update_action:expr, on notification: $notif_action:expr, on filters_changed: $fc_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $update_action,
        on notification: $notif_action,
        on delete: Ok(log::info!(id = id; "deletion notice received")),
        on filters_changed: $fc_action,
      }
    };
    ($handler_name:ident, on update: $update_action:expr, on delete: $delete_action:expr, on filters_changed: $fc_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: $update_action,
        on notification: Ok(log::info!(notification = log::as_serde!(notification); "received notification")),
        on delete: $delete_action,
        on filters_changed: $fc_action,
      }
    };
    ($handler_name:ident, on notification: $notif_action:expr, on delete: $delete_action:expr, on filters_changed: $fc_action:expr) => {
      dispatch_events! {
        $handler_name,
        on update: Ok(log::info!(status = log::as_serde!(status); "received status update")),
        on notification: $notif_action,
        on delete: $delete_action,
        on filters_changed: $fc_action,
      }
    };
}

#[cfg(test)]
mod test {
    use crate::{entities::event::Event, Mastodon, Result};
    #[test]
    fn test_default_dispatch_compiles() {
        dispatch_events! { test_dispatcher }
    }
}
