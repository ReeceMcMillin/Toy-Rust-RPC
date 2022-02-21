#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use std::{fmt::Display, collections::HashMap};
use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;

pub type Data = HashMap<String, Person>;

/// # Note: these docs outdated once `server` crate implemented
/// A remote procedure call message.
/// 
/// Embedding possible procedure calls into the data model with an enum allows Rust's compiler to guarantee exhaustiveness.
/// Adding, removing, or modifying a variant breaks downstream compilation until consumers appropriately handle the new variant.
/// - In adding a variant, the `worker` process must be prepared to respond to a message of that type.
/// - In removing a variant, both the `client` and `worker` must change:
///     - the `client` process can no longer send messages of that type.
///     - the `worker` process must remove logic dependent on messages of that type.
/// - In modifying a variant, both the `client` and `worker` must account for the new semantics/content of a message.
/// 
/// In other words: there is a single formally verifiable source of truth for the validity of a message. If a consumer compiles, it's type-checked against all possible messages.
#[derive(Debug, Serialize, Deserialize)]
pub enum Call {
    Name{ name: String },
    Location { location: String },
    Year { location: String, year: u16 },
}

/// By all rights *should* be private, all public right now as I'm returning mock data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub record_id: u64,
    pub name: String,
    pub location: String,
    pub year: u16,
}

#[derive(Debug)]
pub enum Group {
    Am,
    Nz,
    Unimplemented,
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Group::Am => "am",
            Group::Nz => "nz",
            Group::Unimplemented => "unimplemented"
        };

        write!(f, "{repr}", )
    }
}

impl TryFrom<String> for Group {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "am" => Ok(Self::Am),
            "nz" => Ok(Self::Nz),
            _ => Err("The only valid groups are `am` and `nz`.")
        }
    }
}

#[derive(Debug)]
pub struct Proxy {
    socket: UdpSocket,
    remote_addr: String,
}

impl Proxy {
    pub async fn new(remote_port: u16) -> Self {
        Self { socket: UdpSocket::bind("0.0.0.0:0").await.unwrap(), remote_addr: format!("0.0.0.0:{remote_port}") }
    }

    async fn send_request(&self, call: Call) -> String {
        let mut buf = [0; 1024];
        let serialized = serde_json::to_value(call).unwrap();
        self.socket.send_to(serialized.to_string().as_bytes(), &self.remote_addr).await.unwrap();

        let (len, _addr) = self.socket.recv_from(&mut buf).await.unwrap();
        let response = std::str::from_utf8(&buf[..len]).unwrap();

        String::from(response)
    }

    pub async fn get_by_name(&self, name: String) -> HashMap<String, Person> {
        let response = self.send_request(Call::Name{ name: name.to_string() }).await;
        serde_json::from_str::<HashMap<String, Person>>(&response).unwrap()
    }

    pub async fn get_by_location(&self, location: String) -> HashMap<String, Person> {
        let response = self.send_request(Call::Location { location: location.to_string() }).await;
        serde_json::from_str::<HashMap<String, Person>>(&response).unwrap()
    }

    pub async fn get_by_year(&self, location: String, year: u16) -> HashMap<String, Person> {
        let response = self.send_request(Call::Year { location: location.to_string(), year }).await;
        serde_json::from_str::<HashMap<String, Person>>(&response).unwrap()
    }
}