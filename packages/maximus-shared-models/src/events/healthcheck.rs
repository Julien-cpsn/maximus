use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum HealthCheckEvent {
    IsLoggedIn,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum HealthCheckEventResponse {
    LoginStatus(bool),
}