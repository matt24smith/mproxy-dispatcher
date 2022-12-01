# MPROXY: Multicast Network Dispatcher and Proxy
Streams files and raw socket data over the network. Includes client, proxy, 
reverse-proxy, and server applications, as well as a library API. Provides a 
complete network stack using [UDP Multicast](https://en.wikipedia.org/wiki/Multicast) as 
an intermediate route, enabling scalable stream multiplexing and aggregate feeds.

- [X] Stream arbitrary data over the network
- [X] Complete networking stack
  - Send, proxy, reverse-proxy, and receive to/from multiple endpoints simultaneously
  - Stream multiplexing and aggregation via multicast IP routing
  - Hostname resolution
- [X] Fast
  - 500+ Mbps read/transfer/write speed (UDP)
- [X] Minimal 
  - Compiled binaries ~350KB
  - Tiny memory footprint
  - Stateless: no shared resources between threads. Communication between threads are routed via UDP multicast


### Compatible with
- [X] UDP
- [X] TCP (via `proxy` or `reverse_proxy`)
- [X] TLS (partial support for client TLS via `proxy`. Requires feature `tls` enabled)
- [X] IPv4
- [X] IPv6
- [X] Unix/Linux/Mac
- [X] Windows


## Install Binary
```
cargo install mproxy-client
cargo install mproxy-proxy
cargo install mproxy-reverseproxy
cargo install mproxy-server
```


## Using MPROXY as a library 
Include in Cargo.toml:
```
[dependencies]
mproxy-client = "0.1.0"
mproxy-proxy = "0.1.0"
mproxy-reverseproxy = "0.1.0"
mproxy-server = "0.1.0"
```


## Command Line Interface

### Client
```
MPROXY: UDP Client

Stream local data to logging servers via UDP

USAGE:
  mproxy-client [FLAGS] [OPTIONS] ...

OPTIONS:
  --path        [FILE_DESCRIPTOR]   Filepath, descriptor, or handle. Use "-" for stdin
  --server-addr [HOSTNAME:PORT]     Downstream UDP server address. May be repeated 

FLAGS:
  -h, --help    Prints help information
  -t, --tee     Copy input to stdout

EXAMPLE:
  mproxy-client --path /dev/random --server-addr '127.0.0.1:9920' --server-addr '[::1]:9921'
  mproxy-client --path - --server-addr '224.0.0.1:9922' --server-addr '[ff02::1]:9923' --tee >> logfile.log
```

### Proxy
```
MPROXY: Proxy

Forward TCP, UDP, or Multicast endpoints to a downstream UDP socket address. 

USAGE:
  mproxy-proxy  [FLAGS] [OPTIONS]

OPTIONS:
  --udp-listen-addr     [HOSTNAME:PORT]     UDP listening socket address. May be repeated
  --udp-downstream-addr [HOSTNAME:PORT]     UDP downstream socket address. May be repeated
  --tcp-connect-addr    [HOSTNAME:PORT]     Connect to TCP host, forwarding stream. May be repeated 

FLAGS:
  -h, --help    Prints help information
  -t, --tee     Copy input to stdout

EXAMPLE:
  mproxy-proxy --udp-listen-addr '0.0.0.0:9920' \
    --udp-downstream-addr '[::1]:9921' \
    --udp-downstream-addr 'localhost:9922' \
    --tcp-connect-addr 'localhost:9925' \
    --tee
```

### Reverse-Proxy
```
MPROXY: Reverse-proxy

Forward upstream TCP and/or UDP endpoints to downstream listeners.
Messages are routed via UDP multicast to downstream sender threads. 
Spawns one thread per listener.

USAGE:
  mproxy-reverseproxy  [FLAGS] [OPTIONS]

OPTIONS:
  --udp-listen-addr [HOSTNAME:PORT]     Spawn a UDP socket listener, and forward to --multicast-addr
  --tcp_listen_addr [HOSTNAME:PORT]     Reverse-proxy accepting TCP connections and forwarding to --multicast-addr
  --multicast-addr  [MULTICAST_IP:PORT] Defaults to '[ff02::1]:9918'
  --tcp-output-addr [HOSTNAME:PORT]     Forward packets from --multicast-addr to TCP downstream
  --udp_output_addr [HOSTNAME:PORT]     Forward packets from --multicast-addr to UDP downstream

FLAGS:
  -h, --help    Prints help information
  -t, --tee     Print UDP input to stdout

EXAMPLE:
  reverse_proxy --udp-listen-addr '0.0.0.0:9920' --tcp-output-addr '[::1]:9921' --multicast-addr '224.0.0.1:9922'
```

### Server
```
MPROXY: UDP Server

Listen for incoming UDP messages and log to file.

USAGE:
  mproxy-server [FLAGS] [OPTIONS] ...

OPTIONS: 
  --path        [FILE_DESCRIPTOR]   Filepath, descriptor, or handle.
  --listen-addr [SOCKET_ADDR]       Upstream UDP listening address. May be repeated 

FLAGS:
  -h, --help    Prints help information
  -t, --tee     Copy input to stdout

EXAMPLE:
  mproxy-server --path logfile.log --listen-addr '127.0.0.1:9920' --listen-addr '[::1]:9921'
```


## Motivation

- Complete yet barebones distributed networks framework for e.g. telemetry or sensor data
- Zero-configuration, simple operation and deployment
- Leverage benefits of UDP protocol:
  - Ability to merge data streams from many sources
  - Stream multiplexing and redistribution
  - UDP multicasting enables stateless, scaleable reverse-proxy
- Prioritizing cross-compatability, simplicity, security, and performance

## Alternatives

- cURL
- [Netcat](https://en.wikipedia.org/wiki/Netcat) Point-to-point communications with complete feature set 
- [Nginx](https://en.wikipedia.org/wiki/Nginx) Feature-rich proxy server with static file serving, file caching, and load balancing
- [Websocat](https://github.com/vi/websocat) Command-line client for websockets

