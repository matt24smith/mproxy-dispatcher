use std::io::{stdout, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::thread::spawn;
use std::thread::{Builder, JoinHandle};

use client::target_socket_interface;
use server::upstream_socket_interface;
use socket_dispatch::BUFSIZE;

pub fn proxy_thread(listen_addr: String, downstream_addrs: &[String], tee: bool) -> JoinHandle<()> {
    //let listen_socket = new_listen_socket(listen_addr);
    let (_addr, listen_socket) = upstream_socket_interface(listen_addr).unwrap();
    let mut output_buffer = BufWriter::new(stdout());
    let targets: Vec<(SocketAddr, UdpSocket)> = downstream_addrs
        .iter()
        .map(|t| target_socket_interface(t).unwrap())
        .collect();
    let mut buf = [0u8; BUFSIZE]; // receive buffer
    Builder::new()
        .name(format!("{:#?}", listen_socket))
        .spawn(move || {
            //listen_socket.read_timeout().unwrap();
            listen_socket.set_broadcast(true).unwrap();
            loop {
                match listen_socket.recv_from(&mut buf[0..]) {
                    Ok((c, _remote_addr)) => {
                        for (target_addr, target_socket) in &targets {
                            target_socket
                                .send_to(&buf[0..c], target_addr)
                                .expect("sending to server socket");
                        }
                        if tee {
                            let _o = output_buffer
                                .write(&buf[0..c])
                                .expect("writing to output buffer");
                            #[cfg(debug_assertions)]
                            assert!(c == _o);
                        }
                    }
                    Err(err) => {
                        //output_buffer.flush().unwrap();
                        eprintln!("proxy_thread: got an error: {}", err);
                        #[cfg(debug_assertions)]
                        panic!("proxy_thread: got an error: {}", err);
                    }
                }
                output_buffer.flush().unwrap();
            }
        })
        .unwrap()
}

pub fn proxy_gateway(
    downstream_addrs: &[String],
    listen_addrs: &[String],
    tee: bool,
) -> Vec<JoinHandle<()>> {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for listen_addr in listen_addrs {
        #[cfg(debug_assertions)]
        println!(
            "proxy: forwarding {:?} -> {:?}",
            listen_addr, downstream_addrs
        );
        threads.push(proxy_thread(listen_addr.to_string(), downstream_addrs, tee));
    }
    threads
}

pub fn proxy_tcp_udp(upstream_tcp: String, downstream_udp: String) -> JoinHandle<()> {
    let mut buf = [0u8; BUFSIZE];
    let mut stream = TcpStream::connect(upstream_tcp).expect("connecting to TCP address");
    let (target_addr, target_socket) =
        target_socket_interface(&downstream_udp).expect("UDP downstream interface");

    spawn(move || loop {
        match stream.read(&mut buf[0..]) {
            Ok(c) => {
                if c == 0 {
                    panic!("encountered EOF, disconnecting TCP proxy thread...");
                }
                //println!("{:?}", String::from_utf8_lossy(&buf[0..c]));
                target_socket
                    .send_to(&buf[0..c], target_addr)
                    .expect("sending to UDP socket");
            }
            Err(e) => {
                panic!("err: {}", e);
            }
        }
    })
}
