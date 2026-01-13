use tokio::net::{TcpListener, TcpStream};
pub struct Gateway;

pub async fn process(stream: TcpStream) -> anyhow::Result<()> {
    Ok(())
}
pub async fn accept() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::task::spawn(async move {
            let socket = socket;
            let addr = addr;
            println!("{:?}", addr);
        });
    }
}

