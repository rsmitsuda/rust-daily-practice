use std::net::TcpListener;
use std::net::SocketAddr;

fn main() {
    let local_add = [
        SocketAddr::from(([127, 0, 0, 1], 3000)),
        SocketAddr::from(([127, 0, 0,1], 289)),
    ];
    
    //create the TCP stream
    let tcp_streams = TcpListener::bind(&local_add[..]).expect("could not bind");

    //listen for connections
    tcp_streams.set_nonblocking(true).expect("blocking is broken");

    //iterate over connections received by tcp listener
    for stream in tcp_streams.incoming() {
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
    }
}
