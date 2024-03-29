use futures::executor::block_on;
use async_std::io::prelude::*;
use async_std::net;
use async_std::task;

async fn cheapo_request(host: &str, prot: u16, path: &str) -> std::io::Result<String> {
    println!("begin connect {}", host);
    let mut socket = net::TcpStream::connect((host, prot)).await?;
    println!("connect {} over", host);
    let request = format!("GET {} HTTP/1.1\r\nHOST: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    Ok(response)
}

async fn cheapo_owning_request(host: String, prot: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, prot, &path).await
}

async fn many_request(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut handles = vec![];
    for (host, prot, path) in requests {
        handles.push(task::spawn_local(cheapo_owning_request(host, prot, path)));
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}


async fn fa() {
    println!("function a");
}

async fn exec() {
    fa().await;
}
use futures::future;
use futures::select;
pub fn main() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => { 
                println!("get a");
                total += a;
            },
            b = b_fut => {
                println!("get b");
                total += b;
            },
            complete => break,
            default => panic!(), // 该分支永远不会运行，因为 `Future` 会先运行，然后是 `complete`
        };
    }
    assert_eq!(total, 10);
}
//fn main() {
//    // let requests = vec![
//    //     ("example.com".to_string(), 80, "/".to_string()),
//    //     ("www.red-bean.com".to_string(), 80, "/".to_string()),
//    //     // ("en.wikipedia.org".to_string(), 80, "/".to_string()),
//    // ];
//    // let results = async_std::task::block_on(many_request(requests));
//    // for result in results {
//    //     match result {
//    //         Ok(response) => println!("{}", response),
//    //         Err(err) => eprintln!("error: {}", err),
//    //     }
//    // }
//    async_std::task::block_on(exec());
//}


