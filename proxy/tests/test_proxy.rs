use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use mproxy_client::client_socket_stream;
use mproxy_forward::forward_udp;
use mproxy_server::listener;

use testconfig::{truncate, TESTDATA, TESTINGDIR};

#[test]
fn test_forward_udp_ipv4_unicast() {
    let client_target = "127.0.0.1:8890".to_string();
    let proxy_listen = "0.0.0.0:8890".to_string();
    let proxy_target = "127.0.0.1:8891".to_string();
    //let proxy_target = "0.0.0.0:8891".to_string();
    let server_listen = "0.0.0.0:8891".to_string();

    let data = PathBuf::from(TESTDATA);
    let pathstr = &[TESTINGDIR, "streamoutput_forward_udp_ipv4_output.log"].join(&"");
    let output = PathBuf::from(pathstr);
    assert!(data.is_file());

    let _l = listener(server_listen, output, true);
    sleep(Duration::from_millis(15));

    let targets = vec![proxy_target];
    let _p = forward_udp(proxy_listen, &targets, false);
    sleep(Duration::from_millis(15));

    let _c = client_socket_stream(&data, vec![client_target], false);

    let output = PathBuf::from(pathstr);
    let bytesize = truncate(output);
    assert!(bytesize > 0);
}

#[test]
fn test_forward_udp_ipv6_unicast() {
    let client_target = "[::1]:8892".to_string();
    let proxy_listen = "[::]:8892".to_string();
    let proxy_target = "[::1]:8893".to_string();
    let server_listen = "[::]:8893".to_string();

    let data = PathBuf::from(TESTDATA);
    let pathstr = &[TESTINGDIR, "streamoutput_forward_udp_ipv6_output.log"].join(&"");
    let output = PathBuf::from(pathstr);
    assert!(data.is_file());

    let _l = listener(server_listen, output, false);
    sleep(Duration::from_millis(15));

    let targets = vec![proxy_target];
    let _p = forward_udp(proxy_listen, &targets, false);
    sleep(Duration::from_millis(15));

    let _c = client_socket_stream(&data, vec![client_target], false);

    let output = PathBuf::from(pathstr);
    let bytesize = truncate(output);
    assert!(bytesize > 0);
}

#[test]
fn test_forward_udp_ipv4_multicast() {
    let client_target = "127.0.0.1:8894".to_string();
    let proxy_listen = "0.0.0.0:8894".to_string();
    let proxy_target = "127.0.0.1:8895".to_string();
    let server_listen = "0.0.0.0:8895".to_string();

    let data = PathBuf::from(TESTDATA);
    let pathstr = &[
        TESTINGDIR,
        "streamoutput_forward_udp_ipv4_multicast_output.log",
    ]
    .join(&"");
    let output = PathBuf::from(pathstr);
    assert!(data.is_file());

    let _l = listener(server_listen, output, false);
    sleep(Duration::from_millis(15));

    let targets = vec![proxy_target];
    let _p = forward_udp(proxy_listen, &targets, false);
    sleep(Duration::from_millis(15));

    let _c = client_socket_stream(&data, vec![client_target], false);

    let output = PathBuf::from(pathstr);
    let bytesize = truncate(output);
    assert!(bytesize > 0);
}

#[test]
fn test_forward_udp_ipv6_multicast() {
    let client_target = "[ff02::1]:8896".to_string();
    let proxy_listen = "[ff02::]:8896".to_string();
    let proxy_target = "[ff02::1]:8897".to_string();
    let server_listen = "[ff02::]:8897".to_string();

    let data = PathBuf::from(TESTDATA);
    let pathstr = &[
        TESTINGDIR,
        "streamoutput_forward_udp_ipv6_multicast_output.log",
    ]
    .join(&"");
    let output = PathBuf::from(pathstr);
    assert!(data.is_file());

    let _l = listener(server_listen, output, false);
    sleep(Duration::from_millis(15));

    let targets = vec![proxy_target];
    let _p = forward_udp(proxy_listen, &targets, false);
    sleep(Duration::from_millis(15));

    let _c = client_socket_stream(&data, vec![client_target], false);
    sleep(Duration::from_millis(15));

    let output = PathBuf::from(pathstr);
    let bytesize = truncate(output);
    assert!(bytesize > 0);
}
