## Async TCP Chat Server in Rust
This project is a simple asynchronous TCP-based chat server written in Rust using the tokio runtime. It supports multiple clients connecting at once, and broadcasts messages from one client to all other connected clients.

## 🚀 Features

- Handles multiple TCP clients concurrently.

- Uses tokio::broadcast to publish messages to all subscribers efficiently.

- Clients receive messages from others, but not their own.

- Demonstrates use of async I/O with tokio.

## 🛠️ Technologies Used
- Rust

- Tokio for asynchronous runtime

- tokio::net::TcpListener for handling TCP connections

- tokio::sync::broadcast for message broadcasting between clients

- AsyncReadExt and AsyncWriteExt for async I/O

  
## 📦 Installation
Make sure you have Rust and Cargo installed. Then:

```
git clone https://github.com/your-username/multi-chat.git
cd multi-chat && cd chat-server
cargo build
```

## 🧪 Running the Server
```
cargo run
```

## 💻 Connecting Clients
You can use telnet, netcat, or write your own TCP client.
Using netcat:
```
nc 127.0.0.1 8000
```
Open multiple terminal windows and run the above command to simulate multiple clients. Messages sent from one client will be broadcasted to the others.

## 📁 Example
```
Terminal 1:
$ nc 127.0.0.1 8000
Hello from client 1!

Terminal 2:
$ nc 127.0.0.1 8000
> Hello from client 1!
```
## Note
- The server prevents a client from receiving its own messages.

- Broadcast buffer is limited (currently set to 16); older messages may be dropped if not received in time.

