use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CFMessage {
    pub code: u32,
    pub message: String,
}

impl Display for CFMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code={} {}", self.code, self.message)
    }
}

#[derive(Deserialize)]
pub struct CFEmailDestination {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CFEmailDestinations {
    pub messages: Vec<CFMessage>,
    pub success: bool,
    pub result: Vec<CFEmailDestination>,
}
