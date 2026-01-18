

// use std::{collections::HashMap, f32::consts::E, sync::{Arc, Mutex, MutexGuard}};


// #[tokio::main]
// async fn main() {

//     let db:Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

//     // 引用 计数 ++    
//     let db_1 = db.clone();
//     let pro1 = tokio::spawn(async move {
//         let mut data= db_1.lock().unwrap();
//         data.insert(String::from("value1"), String::from("key1"));
//         println!("任务1 写入 成功!");
//     });

//     let db_2 = db.clone();

//     let pro2 = tokio::spawn(async move {
//         let mut data = db_2.lock().unwrap();
//         data.insert(String::from("value2"), String::from("key2"));
//         println!("任务2 写入 成功!");
//     });

//     let _ = tokio::join!(pro1,pro2);

//     let data = db.lock().unwrap();
//     match data.get("value1") {
//         Some(content)=>{
//             println!("value1 = {}",content);
//         }
//         _ =>{
//             println!("value1 没有东西");
//         }
//     }

//     match data.get("value2") {
//         Some(content)=>{
//             println!("value2 = {}",content);
//         }
//         _ =>{
//             println!("value2 没有东西");
//         }
//     }
// }



use std::collections::HashMap;
use std::fmt::format;
use std::sync::{Arc, Mutex};
use std::{error::Error};

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt; 
use tokio::io::AsyncWriteExt; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    let db:Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));


    loop{
        // 接受连接
        let (mut socket,addr)  = listener.accept().await?;

        let db_clone = db.clone();

        tokio::spawn(async move{
            let mut read_buf = [0;1024];
            
            // 不断 等待 连接好的 客户端 信息
            loop{
                let number = match socket.read(&mut read_buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) =>{
                        println!("发生错误是: {}",e);
                        return;
                    }
                };

                 let response:String;

                let client_content = String::from_utf8_lossy(&read_buf[0..number]);
                // println!("来自ip为:{}发来信息: {}",addr.ip(),client_content);

                let client_format:Vec<&str> = client_content.trim().split_whitespace().collect();

                match client_format.as_slice() {

                    ["SET",key,value] =>{
                        let mut data: std::sync::MutexGuard<'_, HashMap<String, String>> = db_clone.lock().unwrap();
                        data.insert(key.to_string(), value.to_string());
                        response = String::from("数据加入成功");
                        println!("用户存入数据[{}] -> [{}]",key,value);
                    },
                    ["GET",key] => {
                        let data = db_clone.lock().unwrap();
                        let value = match data.get(*key) {
                            Some(val) => val,
                            _ => ""
                        };
                        response = format!("数据读取成功: {}",value);
                        println!("用户读取数据[{}] -> [{}]",key,value);
                    },
                    _ =>{
                        response = format!("未知操作符号");
                    }
                }
               
                match socket.write_all(response.as_bytes()).await {
                    Ok(_) => (),
                    Err(e) =>{
                        println!("发生错误: {}",e);
                        return;
                    }
                }
            }
        });
    };

    
    Ok(())
}