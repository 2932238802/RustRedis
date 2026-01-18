use std::error::Error;
use std::io;
use std::process::exit;

use tokio;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:6379").await?;

    println!("连接成功!");

    // let request = format!("SET birth 041021");
    loop {
        let mut request_buffer = String::from("");
        io::stdin().read_line(&mut request_buffer).unwrap();

        if request_buffer.trim() == "exit" 
        {
            exit(1);
        }

        socket.write_all(request_buffer.as_bytes()).await?;

        let mut read_buf = [0; 1024];
        let number = socket.read(&mut read_buf).await?;

        println!(
            "收到服务器的回复: {}",
            String::from_utf8_lossy(&mut read_buf[0..number])
        );
    }

    Ok(())
}
