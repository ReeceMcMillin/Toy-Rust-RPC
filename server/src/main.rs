#![warn(clippy::pedantic)]

use std::{io, collections::HashMap};
use model::{Call, Data, Proxy};
use tokio::net::UdpSocket;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    port: u16,
}

#[derive(Debug)]
struct Server {
    port: u16,
    workers: Vec<Proxy>
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port, workers: Vec::new() }
    }

    pub async fn attach_worker(&mut self, port: u16) {
        self.workers.push(Proxy::new(port).await);
    }

    async fn get_by_name(&self, name: String) -> Data {
        let mut response: Data = HashMap::new();

        for worker in &self.workers {
            response.extend(worker.get_by_name(name.clone()).await);
        }

        response
    }
    
    async fn get_by_location(&self, location: String) -> Data {
        let mut response: Data = HashMap::new();

        for worker in &self.workers {
            response.extend(worker.get_by_location(location.clone()).await);
        }

        response
    }
    
    async fn get_by_year(&self, location: String, year: u16) -> Data {
        let mut response: Data = HashMap::new();

        for worker in &self.workers {
            response.extend(worker.get_by_year(location.clone(), year).await);
        }

        response
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
    
            // Call procedure stubs to dispatch to workers
            let person = match call {
                Call::Name { name } => self.get_by_name(name).await,
                Call::Location { location } => self.get_by_location(location).await,
                Call::Year { location, year } => self.get_by_year(location, year).await,
            };

            let response = serde_json::to_value(person).unwrap().to_string();
    
            let _len = socket.send_to(response.as_bytes(), addr).await?;
    
            // Clear buffer
            buf = [0; 1024];
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let mut server = Server::new(args.port);
    
    // This requires the workers to be running before the server, failure is hard
    server.attach_worker(23001).await;
    server.attach_worker(23002).await;

    server.listen().await.unwrap();
}
