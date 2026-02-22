use dioxus::fullstack::{WebSocketOptions, Websocket};
use dioxus::prelude::*;
use maximus_shared_models::events::healthcheck::{HealthCheckEvent, HealthCheckEventResponse};
use crate::user::login::is_logged;

#[get("/api/healthcheck_ws")]
pub async fn healthcheck_ws(options: WebSocketOptions) -> Result<Websocket<HealthCheckEvent, HealthCheckEventResponse>> {
    Ok(options.on_upgrade(on_healthcheck_event))
}

#[cfg(feature = "server")]
async fn on_healthcheck_event(mut socket: dioxus::fullstack::TypedWebsocket<HealthCheckEvent, HealthCheckEventResponse>) {
    while let Ok(event) = socket.recv().await {
        debug!("{}", event);
        match event {
            HealthCheckEvent::IsLoggedIn => {
                let is_logged_in = is_logged().await.unwrap_or(false);
                let _  =socket.send(HealthCheckEventResponse::LoginStatus(is_logged_in)).await;
            }
        }
    }
}