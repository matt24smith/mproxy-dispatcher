//! Multicast Network Dispatcher and Proxy
//!
//! # MPROXY: Client
//! Stream file or socket data via UDP. Supports multicast routing
//!
//!
//! ## Quick Start
//! In `Cargo.toml`
//! ```toml
//! [dependencies]
//! mproxy-client = "0.1"
//! ```
//!
//! Example `src/main.rs`
//! ```rust,no_run
//! use std::path::PathBuf;
//! use std::thread::spawn;
//!
//! use mproxy_client::client_socket_stream;
//!
//! // read input from stdin
//! let path = PathBuf::from("-");
//!
//! // downstream UDP socket addresses
//! let server_addrs =  vec!["127.0.0.1:9919".into(), "localhost:9921".into(), "[ff02::1]:9920".into()];
//!
//! // copy input to stdout
//! let tee = true;
//!
//! let client_thread = spawn(move || {
//!     client_socket_stream(&path, server_addrs, tee).unwrap();
//! });
//!
//! // run client until EOF
//! client_thread.join().unwrap();
//! ```
//!
//! ## Command Line Interface
//! Install with cargo
//! ```bash
//! cargo install mproxy-client
//! ```
//!
//! ```text
//! MPROXY: UDP Client
//!
//! Stream local data to logging servers via UDP
//!
//! USAGE:
//!   mproxy-client [FLAGS] [OPTIONS] ...
//!
//! OPTIONS:
//!   --path        [FILE_DESCRIPTOR]   Filepath, descriptor, or handle. Use "-" for stdin
//!   --server-addr [HOSTNAME:PORT]     Downstream UDP server address. May be repeated
//!
//! FLAGS:
//!   -h, --help    Prints help information
//!   -t, --tee     Copy input to stdout
//!
//! EXAMPLE:
//!   mproxy-client --path /dev/random --server-addr '127.0.0.1:9920' --server-addr '[::1]:9921'
//!   mproxy-client --path - --server-addr '224.0.0.1:9922' --server-addr '[ff02::1]:9923' --tee >> logfile.log
//! ```
//!
//! ### See Also
//! - [mproxy-client](https://docs.rs/mproxy-client/)
//! - [mproxy-server](https://docs.rs/mproxy-server/)
//! - [mproxy-forward](https://docs.rs/mproxy-forward/)
//! - [mproxy-reverse](https://docs.rs/mproxy-reverse/)
//!

use std::fs::OpenOptions;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Result as ioResult, Write};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket, IpAddr, Ipv6Addr};
use std::path::PathBuf;
use std::str::FromStr;

//use mproxy_socket_dispatch::{bind_socket, new_socket, BUFSIZE};
use mproxy_socket_dispatch::BUFSIZE;

pub fn target_socket_interface(server_addr: &String) -> ioResult<(SocketAddr, UdpSocket)> {
    let target_addr = server_addr
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("parsing socket address");

    // Binds to a random UDP port for sending to downstream.
    let unspec: SocketAddr = if target_addr.is_ipv4() {
        SocketAddr::new(std::net::Ipv4Addr::UNSPECIFIED.into(), 0)
    } else {
        SocketAddr::new(std::net::Ipv6Addr::UNSPECIFIED.into(), 0)
    };

    let target_socket;

    match (target_addr.ip().is_multicast(), target_addr.ip()) {
        (false, IpAddr::V4(_)) => {
            target_socket = UdpSocket::bind(unspec).expect("binding client socket");
            target_socket.connect(target_addr).unwrap_or_else(|e| panic!("{}", e));
        }
        (false, IpAddr::V6(_)) => {
            target_socket = UdpSocket::bind(unspec).expect("binding client socket");
            target_socket.connect(target_addr).unwrap_or_else(|e| panic!("{}", e));
        },
        (true, IpAddr::V4(ip)) => {
            target_socket = UdpSocket::bind(unspec).expect("binding client socket");
            target_socket.connect(target_addr).unwrap_or_else(|e| panic!("{}", e));
            target_socket
            .join_multicast_v4(&ip, &std::net::Ipv4Addr::UNSPECIFIED)
            .unwrap();
        },
        (true, IpAddr::V6(ip)) => {
            target_socket = UdpSocket::bind(unspec).expect("binding client socket");
            // specify "any available interface" for multicast with index 0
            target_socket.join_multicast_v6(&ip, 0).unwrap_or_else(|e| panic!("{}", e));
            target_socket.connect(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), target_addr.port())).unwrap_or_else(|e| panic!("{}", e));
            //target_socket.connect(target_addr).unwrap_or_else(|e| panic!("{}", e));
        },
    };

    Ok((target_addr, target_socket))
}

/// Read bytes from `path` info a buffer, and forward to downstream UDP server addresses.
/// Optionally copy output to stdout
pub fn client_socket_stream(path: &PathBuf, server_addrs: Vec<String>, tee: bool) -> ioResult<()> {
    let mut targets = vec![];

    for server_addr in server_addrs {
        let (target_addr, target_socket) = target_socket_interface(&server_addr)?;

        targets.push((target_addr, target_socket));
        println!(
            "logging from {}: sending to {}",
            &path.as_os_str().to_str().unwrap(),
            server_addr,
        );
    }

    // if path is "-" set read buffer to stdin
    // otherwise, create buffered reader from given file descriptor
    let mut reader: Box<dyn BufRead> = if path == &PathBuf::from_str("-").unwrap() {
        Box::new(BufReader::new(stdin()))
    } else {
        Box::new(BufReader::new(
            OpenOptions::new()
                .create(false)
                .write(false)
                .read(true)
                .open(path)
                .unwrap_or_else(|e| {
                    panic!("opening {}, {}", path.as_os_str().to_str().unwrap(), e)
                }),
        ))
    };

    let mut buf = vec![0u8; BUFSIZE];
    let mut output_buffer = BufWriter::new(stdout());

    while let Ok(c) = reader.read(&mut buf) {
        if c == 0 {
            #[cfg(debug_assertions)]
            println!(
                "\nclient: encountered EOF in {}, exiting...",
                &path.display(),
            );
            break;
        } else if c == 1 && String::from_utf8(buf[0..c].to_vec()).unwrap() == *"\n" {
            // skip empty lines
            continue;
        }

        //#[cfg(debug_assertions)]
        //println!("\nc:{} |{:?}|", c, String::from_utf8(buf[0..c].to_vec()));

        for (target_addr, target_socket) in &targets {
            target_socket
                .send_to(&buf[0..c], target_addr)
                .expect("sending to server socket");
            /*
            target_socket
            .send(&buf[0..c])
            .expect("sending to server socket");
            */
        }
        if tee {
            let _o = output_buffer
                .write(&buf[0..c])
                .expect("writing to output buffer");
            output_buffer.flush().unwrap();
            #[cfg(debug_assertions)]
            assert!(c == _o);
        }
    }
    Ok(())
}
