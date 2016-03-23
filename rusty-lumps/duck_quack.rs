use std::io::net::tcp::TcpListener;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::{Acceptor, Listener};

fn handle_client(stream: ~TcpListener) {
    for mut client in stream.listen().incoming() {
        println!("RX: {}", client.read_to_str().unwrap());
    }
}

fn main() {
    let addr = SocketAddr {ip: Ipv4Addr(127, 0, 0, 1), port: 4040 };
    let listener = TcpListener::bind(addr);

    let mut acceptor = listener.listen();

    for client in acceptor.incoming() {
        spawn(proc() {
                handle_client(stream);
            });
    }

    drop(acceptor);
}
