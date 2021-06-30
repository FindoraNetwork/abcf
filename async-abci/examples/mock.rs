use async_abci::Server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    Server::new(())
        .bind("127.0.0.1:26658")
        .await
        .unwrap()
        .run()
        .await
        .unwrap();
    Ok(())
}
