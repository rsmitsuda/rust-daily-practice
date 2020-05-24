use std::net::TcpListener;
use std::net::SocketAddr;
use std::sync::mpsc::channel;
// use std::sync::mpsc;

fn main() {
    let local_add = [
        SocketAddr::from(([127, 0, 0, 1], 3000)),
        SocketAddr::from(([127, 0, 0,1], 289))
    ];
    
    //create the TCP stream
    let tcp_streams = TcpListener::bind(&local_add[..]).expect("could not bind");

    //listen for connections
    tcp_streams.set_nonblocking(true).expect("blocking is broken");

    //create channel
    //let (tx, rx) = mpsc::channel::<String>();
    let (tx, rx) = channel::<String>();


    loop {
        let result = match tcp_streams.accept() {
            Ok((mut tcpstream, socketaddr)) => {
                // let (mut stream, addr) = (mut tcpstream, socketaddr);
                print!("address: {:?}\n", socketaddr);
            }
            Err(e) => {
                print!("there was an error\n");
                break;
            }
        };
    }

    //iterate over connections received by tcp listener
/*    for stream in tcp_streams.incoming() {
        match stream {
            Ok(stream) => {
                print!("{:?}\n", "everything is good with tcp");
            }
            Err(e) => {
                print!("{:?}\n", "the connection is bad");
            }
            _ => {
                print!("{:?}\n", "default");
            }
        }
    }*/
}
