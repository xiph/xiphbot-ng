extern crate irc;

use irc::client::prelude::*;
use std::net::UdpSocket;
use std::sync::Arc;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:61001").unwrap();
    let server = Arc::new(IrcServer::new("config.json").unwrap());
    server.identify().unwrap();
    let server_udp_thread = server.clone();
    std::thread::spawn(move || {
        let mut buf = [0; 1000];
        loop {
            let _ = socket.recv_from(&mut buf);
            let s = std::str::from_utf8(&mut buf).unwrap();
            println!("{}", &s);
            let _ = server_udp_thread.send(Command::PRIVMSG("#xiph".to_string(), s.to_string()));
        }
    });
    for _ in server.iter() {
        // nothing to handle
    }
}
