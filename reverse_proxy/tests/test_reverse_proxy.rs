use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use mproxy_client::client_socket_stream;
use mproxy_reverse::reverse_proxy_udp_tcp;

use testconfig::TESTDATA;

#[test]
fn test_reverse_proxy_tcp() {
    //let udp_upstream_addr = "0.0.0.0:8892".to_string();
    let multicast_addr = "224.0.0.1:8990".to_string();
    let proxy_tcp_output_addr = "0.0.0.0:8992".to_string();
    let client_target_addr = "127.0.0.1:8991".to_string();
    let data = PathBuf::from(TESTDATA);

    // start reverse proxy and wait a moment for thread to spawn
    let _r = reverse_proxy_udp_tcp(multicast_addr, proxy_tcp_output_addr);
    sleep(Duration::from_millis(30));

    // send some data via the proxy
    let _c = client_socket_stream(&data, vec![client_target_addr], false);
    sleep(Duration::from_millis(15));
}

#[test]
fn test_reverse_proxy_tcp_ipv6() {
    //let udp_upstream_addr = "0.0.0.0:8892".to_string();
    let multicast_addr = "[ff02::1]:8993".to_string();
    let proxy_tcp_output_addr = "[::]:8995".to_string();
    let client_target_addr = "[ff02::1]:8994".to_string();
    let data = PathBuf::from(TESTDATA);

    // start reverse proxy and wait a moment for thread to spawn
    let _r = reverse_proxy_udp_tcp(multicast_addr, proxy_tcp_output_addr);
    sleep(Duration::from_millis(30));

    // send some data via the proxy
    let _c = client_socket_stream(&data, vec![client_target_addr], false);
    sleep(Duration::from_millis(15));
}
