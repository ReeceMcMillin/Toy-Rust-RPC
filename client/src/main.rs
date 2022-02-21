#![warn(clippy::pedantic)]

use clap::Parser;
use model::Proxy;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let proxy = Proxy::new(args.port).await;

    dbg!(proxy.get_by_name("rakin".to_string()).await);
    dbg!(proxy.get_by_location("Kansas City".to_string()).await);
    dbg!(proxy.get_by_year("Kansas City".to_string(), 2018).await);
}
