use tokio::{
sync::broadcast
};
use tokio::net::TcpListener;
use std::error::{self, Error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main() -> Result<(),Box< dyn Error>> {


    let listerner = TcpListener::bind("127.0.0.1:8000").await?;

    println!("Listening {:?}", listerner);
    loop {
        let (mut socket, addr) = listerner.accept().await.unwrap();
    
    tokio::spawn(async move {

        loop {
            let mut buf = [0u8; 1024];
            
            let byte_count = socket.read(& mut buf)
            .await
            .expect("Failed to read message");

        if byte_count == 0{
            return ;
        }
        socket.write_all(&buf[0..byte_count])
        .await
        .expect("Failed to write to scoket");
            println!("Accepted connection from  {:?}", addr);
            println!(" {:?}", byte_count);
        }
    
    });
}
    Ok(())
}
