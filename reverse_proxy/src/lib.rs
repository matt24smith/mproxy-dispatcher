use std::io::{BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{spawn, JoinHandle};

use mproxy_client::target_socket_interface;
use mproxy_server::upstream_socket_interface;
use mproxy_socket_dispatch::BUFSIZE;

fn handle_client_tcp(downstream: TcpStream, multicast_addr: String) {
    let (_multicast_addr, multicast_socket) =
        if let Ok((addr, socket)) = upstream_socket_interface(multicast_addr) {
            if !addr.ip().is_multicast() {
                panic!("not a multicast address {}", addr);
            }
            (addr, socket)
        } else {
            panic!()
        };

    let mut buf = [0u8; BUFSIZE];
    let mut tcp_writer = BufWriter::new(downstream);

    loop {
        match multicast_socket.recv_from(&mut buf[0..]) {
            Ok((count_input, _remote_addr)) => {
                let _count_output = tcp_writer.write(&buf[0..count_input]);
            }
            Err(err) => {
                eprintln!("reverse_proxy: got an error: {}", err);
                break;
            }
        }
        if let Err(_e) = tcp_writer.flush() {
            #[cfg(debug_assertions)]
            eprintln!("reverse_proxy: closing {:?} {}", multicast_socket, _e);
            break;
        }
    }
}

/// Forward a UDP socket stream (e.g. from a multicast channel) to connected TCP clients.
/// Spawns a listener thread, plus one thread for each incoming TCP connection.
pub fn reverse_proxy_udp_tcp(multicast_addr: String, tcp_listen_addr: String) -> JoinHandle<()> {
    spawn(move || {
        let listener = TcpListener::bind(tcp_listen_addr).unwrap();
        for stream in listener.incoming() {
            #[cfg(debug_assertions)]
            println!("new client {:?}", stream);
            let multicast_addr = multicast_addr.clone();
            let _tcp_client = spawn(move || {
                handle_client_tcp(stream.unwrap(), multicast_addr);
            });
        }
    })
}

/// Forward bytes from UDP upstream socket address to UDP downstream socket address
pub fn reverse_proxy_udp(udp_input_addr: String, udp_output_addr: String) -> JoinHandle<()> {
    spawn(move || {
        let (addr, listen_socket) = upstream_socket_interface(udp_input_addr).unwrap();
        let (outaddr, output_socket) = target_socket_interface(&udp_output_addr).unwrap();

        let mut buf = [0u8; BUFSIZE];
        loop {
            match listen_socket.recv_from(&mut buf[0..]) {
                Ok((c, _remote_addr)) => {
                    if c == 0 {
                        panic!("{}", outaddr);
                    }
                    let c_out = output_socket
                        .send_to(&buf[0..c], outaddr)
                        .expect("forwarding UDP downstream");
                    assert!(c == c_out);
                }
                Err(err) => {
                    eprintln!("{}:reverse_proxy: error {}", addr, err);
                    break;
                }
            }
        }
    })
}

/// Listen for incoming TCP connections and forward received bytes to a UDP socket address
pub fn reverse_proxy_tcp_udp(upstream_tcp: String, downstream_udp: String) -> JoinHandle<()> {
    //pub fn reverse_proxy_tcp_udp(upstream_tcp: String, downstream_udp: String) {
    spawn(move || {
        let listener = TcpListener::bind(upstream_tcp).expect("binding TCP socket");

        for upstream in listener.incoming() {
            let (target_addr, target_socket) = target_socket_interface(&downstream_udp).unwrap();
            let mut buf = [0u8; BUFSIZE];
            //let mut stream = stream.as_ref().expect("connecting to stream");

            match upstream {
                Ok(mut input) => {
                    spawn(move || loop {
                        match input.read(&mut buf[0..]) {
                            Ok(c) => {
                                target_socket
                                    .send_to(&buf[0..c], target_addr)
                                    .expect("sending to UDP socket");
                            }
                            Err(e) => {
                                eprintln!("err: {}", e);
                                break;
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("dropping client: {}", e);
                }
            }
        }
    })
}
