//! Multicast Network Dispatcher and Proxy
//!
//! # MPROXY: Server
//! Listen for incoming UDP messages and log to file.
//!
//! ## Quick Start
//! In `Cargo.toml`
//! ```toml
//! [dependencies]
//! mproxy-server = "0.1"
//! ```
//!
//! Example `src/main.rs`
//! ```rust,no_run
//! use std::path::PathBuf;
//! use std::thread::JoinHandle;
//!
//! use mproxy_server::listener;
//!
//! // bind to IPv6 multicast channel on port 9920
//! let listen_addr: String = "[ff01::1]:9920".into();
//!
//! // output filepath
//! let logpath = PathBuf::from("server_demo.log");
//!
//! // copy input to stdout
//! let tee = true;
//!
//! // bind socket listener thread
//! let server_thread: JoinHandle<_> = listener(listen_addr, logpath, tee);
//! server_thread.join().unwrap();
//! ```
//!
//! ## Command Line Interface
//! Install with cargo
//! ```bash
//! cargo install mproxy-server
//! ```
//!
//! ```text
//! MPROXY: UDP Server
//!
//! Listen for incoming UDP messages and log to file or socket.
//!
//! USAGE:
//!   mproxy-server [FLAGS] [OPTIONS] ...
//!
//! OPTIONS:
//!   --path        [FILE_DESCRIPTOR]   Filepath, descriptor, or handle.
//!   --listen-addr [SOCKET_ADDR]       Upstream UDP listening address. May be repeated
//!
//! FLAGS:
//!   -h, --help    Prints help information
//!   -t, --tee     Copy input to stdout
//!
//! EXAMPLE:
//!   mproxy-server --path logfile.log --listen-addr '127.0.0.1:9920' --listen-addr '[::1]:9921'
//! ```
//!
//! ### See Also
//! - [mproxy-client](https://docs.rs/mproxy-client/)
//! - [mproxy-server](https://docs.rs/mproxy-server/)
//! - [mproxy-forward](https://docs.rs/mproxy-forward/)
//! - [mproxy-reverse](https://docs.rs/mproxy-reverse/)
//!

use std::fs::OpenOptions;
use std::io::{stdout, BufWriter, Result as ioResult, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::path::PathBuf;
use std::thread::{Builder, JoinHandle};

use net2::UdpBuilder;

const BUFSIZE: usize = 8096;

pub fn upstream_socket_interface(listen_addr: String) -> ioResult<(SocketAddr, UdpSocket)> {
    let addr = listen_addr
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("parsing socket address");
    let listen_socket;
    match (addr.ip().is_multicast(), addr.ip()) {
        (false, std::net::IpAddr::V4(_)) => {
            //listen_socket = UdpSocket::bind(addr).expect("binding server socket");
            listen_socket = UdpBuilder::new_v4()
                .expect("binding ipv4 socket")
                .reuse_address(true)
                .unwrap()
                .bind(addr)
                .unwrap();
        }
        (false, std::net::IpAddr::V6(_)) => {
            //listen_socket = UdpSocket::bind(addr).expect("binding server socket");
            listen_socket = UdpBuilder::new_v6()
                .expect("binding ipv6 socket")
                .reuse_address(true)
                .unwrap()
                .bind(addr)
                .unwrap();
        }
        (true, std::net::IpAddr::V4(ip)) => {
            #[cfg(not(target_os = "windows"))]
            {
                //listen_socket = UdpSocket::bind(addr).expect("binding server socket");
                listen_socket = UdpBuilder::new_v4()
                    .expect("binding ipv4 socket")
                    .reuse_address(true)
                    .unwrap()
                    .bind(addr)
                    .unwrap();
                listen_socket
                    .join_multicast_v4(&ip, &Ipv4Addr::UNSPECIFIED)
                    .unwrap_or_else(|e| panic!("{}", e));
            }
            #[cfg(target_os = "windows")]
            {
                listen_socket = UdpSocket::bind(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                    addr.port(),
                ))
                .expect("binding server socket");

                listen_socket
                    .join_multicast_v4(&ip, &Ipv4Addr::UNSPECIFIED)
                    .unwrap_or_else(|e| panic!("{}", e));
            }
        }
        (true, std::net::IpAddr::V6(ip)) => {
            /*
            listen_socket = UdpSocket::bind(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                addr.port(),
            ))
            .expect("binding server socket");
            */
            listen_socket = UdpBuilder::new_v6()
                .expect("binding ipv6 socket")
                .reuse_address(true)
                .unwrap()
                .bind(SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                    addr.port(),
                ))
                .unwrap();

            // specify "any available interface" with index 0
            #[cfg(not(target_os = "macos"))]
            let itf = 0; // unspecified
            #[cfg(target_os = "macos")]
            let itf = 12; // en0

            listen_socket
                .join_multicast_v6(&ip, itf)
                .unwrap_or_else(|e| panic!("{}", e));

            #[cfg(target_os = "windows")]
            listen_socket
                .connect(&addr)
                .unwrap_or_else(|e| panic!("{}", e));
        }
    };
    Ok((addr, listen_socket))
}

/// Server UDP socket listener.
/// Binds to UDP socket address `addr`, and logs input to `logfile`.
/// Can optionally copy input to stdout if `tee` is true.
/// `logfile` may be a filepath, file descriptor/handle, etc.
pub fn listener(addr: String, logfile: PathBuf, tee: bool) -> JoinHandle<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&logfile);
    let mut writer = BufWriter::new(file.unwrap());
    let mut output_buffer = BufWriter::new(stdout());

    let (addr, listen_socket) = upstream_socket_interface(addr).unwrap();

    Builder::new()
        .name(format!("{}:server", addr))
        .spawn(move || {
            let mut buf = [0u8; BUFSIZE]; // receive buffer
            loop {
                match listen_socket.recv_from(&mut buf[0..]) {
                    Ok((c, _remote_addr)) => {
                        if tee {
                            let _o = output_buffer
                                .write(&buf[0..c])
                                .expect("writing to output buffer");
                            #[cfg(debug_assertions)]
                            assert!(c == _o);
                        }
                        let _ = writer
                            .write(&buf[0..c])
                            .unwrap_or_else(|_| panic!("writing to {:?}", &logfile));
                    }
                    Err(err) => {
                        writer.flush().unwrap();
                        eprintln!("{}:server: got an error: {}", addr, err);
                        #[cfg(debug_assertions)]
                        panic!("{}:server: got an error: {}", addr, err);
                    }
                }

                writer.flush().unwrap();
                if tee {
                    output_buffer.flush().unwrap();
                }
            }
        })
        .unwrap()
}
