use std::{
    io::{ ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
fn main() {
    let server = TcpListener::bind(LOCAL).expect("Stream failed to connect");
    server.set_nonblocking(true).expect("Can't Do it");

    let mut clients = vec![];
    let (tx, rs) = mpsc::channel::<String>();
    loop {
        let res = server.accept();
        match res {
            Ok((mut stream, add)) => {
                let tx = tx.clone();
                clients.push(stream.try_clone().expect("Can't clone"));
                thread::spawn(move || loop {
                    let mut buff = vec![0; MSG_SIZE];
                    match stream.read_exact(&mut buff) {
                        Ok(_) => {
                            let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                            println!("Message From Client ({}) :{}",add, msg);
                                let sending=String::from("Hello From Server");
                                tx.send(sending).expect("msg");
                        }
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => panic!("encountered IO error:"),
                    }
                });
            }
           
            Err(_) => {
            }
        }
        thread::sleep(::std::time::Duration::from_millis(100));
        if let Ok(msg) = rs.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
    }
}

//     for stream in server.incoming() {

//         match stream {
//             Ok(mut e) => {
//                 println!("{:?}", e);
//                 let mut buff = [0; 512];
//                 match e.read(&mut buff) {
//                 Ok(_) => {
//                     println!("a");
//                     println!("{}", String::from_utf8_lossy(&buff));
//                     e.write(String::from("HTTP/1.1 200 OK\r\n\r\n WTFMAN\r\n").as_buff()).expect("msg");
//                 }
//                 Err(e) => panic!("encountered IO error: {e}"),
//             }
//         }
//         Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
//             continue;
//         }
//         Err(_e) => {
//             println!("d");
//             println!("CONN FAILED");
//         }
//     };
// }
// }
// }
