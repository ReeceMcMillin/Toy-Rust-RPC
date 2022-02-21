#![warn(clippy::pedantic)]

use std::io;
use tokio::net::UdpSocket;
use clap::Parser;
use model::{Call, Group, Data};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    port: u16,
    
    #[clap(short, long)]
    group: String,
}

struct Worker {
    data: Data,
    port: u16,
}

#[must_use]
pub fn load_data(group: &Group) -> Data {
    let filename = format!("data-{}.json", group);
    let data = std::fs::read_to_string(filename).expect("Failed to find file {filename}");
    serde_json::from_str(&data).expect("Failed to deserialize file {filename} to json")
}

impl Worker {
    pub fn new(port: u16, group: &Group) -> Self {
        Self { data: load_data(group), port }
    }

    async fn listen(&self) -> io::Result<()> {
        let socket = UdpSocket::bind(("0.0.0.0", self.port)).await?;
        println!("listening on {}...", self.port);
    
        let mut buf = [0; 1024];
    
        loop {
            let (_len, addr) = socket.recv_from(&mut buf).await?;
            let msg = std::str::from_utf8(&buf).unwrap().replace('\u{0}', "");
    
            // Deserialize JSON string into `Call` message enum
            let call = serde_json::from_str::<Call>(msg.trim_end()).unwrap();
    
            let person = match call {
                Call::Name { name } => self.get_by_name(name).await,
                Call::Location { location } => self.get_by_location(location).await,
                Call::Year { location, year } => self.get_by_year(location, year).await,
            };

            let value = serde_json::to_value(person).unwrap();
            let response = value.to_string();
    
            let _len = socket.send_to(response.as_bytes(), addr).await?;
    
            buf = [0; 1024];
        }
    }

    async fn get_by_name(&self, name: String) -> Data {
        self.data
            .iter()
            .filter(|&(_key, person)| { person.name == name })
            .map(|(key, person)| (key.clone(), person.clone()))
            .collect::<Data>()
    }
    
    async fn get_by_location(&self, location: String) -> Data {
        self.data
            .iter()
            .filter(|&(_key, person)| { person.location == location })
            .map(|(key, person)| (key.clone(), person.clone()))
            .collect::<Data>()
    }
    
    async fn get_by_year(&self, location: String, year: u16) -> Data {
        self.data
            .iter()
            .filter(|&(_key, person)| { person.location == location && person.year == year })
            .map(|(key, person)| (key.clone(), person.clone()))
            .collect::<Data>()
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let port = args.port;

    let group = Group::try_from(args.group).unwrap();

    Worker::new(port, &group)
        .listen()
        .await?;

    Ok(())
}
