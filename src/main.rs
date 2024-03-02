use altair::Client;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // localnet
    let _rpc_url = "http://localhost:8899";
    let socket_url = "ws://localhost:8900".to_string();
    let client = Client::new(socket_url);


    // let mut threads = Vec::new();

    Ok(())
}
