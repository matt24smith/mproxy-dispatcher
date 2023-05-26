#![feature(test)]

extern crate test;
use test::Bencher;

use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::thread::sleep;
use std::thread::Builder;
use std::time::{Duration, Instant};

const BUFSIZE: usize = 8096;

extern crate mproxy_server;
use mproxy_server::upstream_socket_interface;

use mproxy_client::client_socket_stream;

#[cfg(all(test, unix))]
#[bench]
fn test_client_bitrate(b: &mut Bencher) {
    let target_addr = "127.0.0.1:9917".to_string();
    let listen_addr = "0.0.0.0:9917".to_string();

    let (_addr, listen_socket) = upstream_socket_interface(listen_addr).unwrap();

    sleep(Duration::from_millis(15));

    let mut bytecount: i64 = 0;
    let mut buf = [0u8; BUFSIZE];

    let _c = Builder::new().spawn(move || {
        client_socket_stream(&PathBuf::from("/dev/random"), vec![target_addr], false)
    });

    let start = Instant::now();
    b.iter(|| {
        let (c, _remote) = listen_socket.recv_from(&mut buf[0..BUFSIZE]).unwrap();
        bytecount += c as i64;
        assert!(c > 0);
    });
    let elapsed = start.elapsed();

    println!(
        "transferred: {} Mb  elapsed: {:.3}s\tbitrate: {:.1} Mbps",
        bytecount / 1000000,
        elapsed.as_secs_f32(),
        bytecount as f64 / elapsed.as_secs_f64() / 1000000 as f64
    );
}
