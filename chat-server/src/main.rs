use tokio::io::unix::AsyncFd;
use tokio::{
sync::broadcast
};
use tokio::net::TcpListener;
use std::error::{ Error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main() -> Result<(),Box< dyn Error>> {

    let listerner = TcpListener::bind("127.0.0.1:8000").await?;
    let (tx, _) = broadcast::channel::<String>(16);

    println!("Listening {:?}", listerner);
    loop {

        let (mut socket, addr) = listerner.accept().await.unwrap();
        println!("Connection recieved from {}", addr);
        
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut  reader,mut  writer )= socket.into_split();
    tokio::spawn(async move {

        loop {
            let mut buf = [0u8; 1024];
            
           
            let byte_count = reader.read(& mut buf)
            .await
            .expect("Failed to read message");
            println!("{}", byte_count);
            if byte_count == 0{
                return ;
            }
            let msg = String::from_utf8_lossy(&buf[..byte_count]).to_string();
        let _ = tx.send(msg).unwrap();
        }
    
    });



tokio::spawn( async move {
    while let Ok(msg) = rx.recv().await {

        writer.write_all(msg.as_bytes()).await;
    }
});

};
    Ok(())
}
