# MPROXY: Multicast Network Dispatcher and Proxy
Streams files and raw socket data over the network. MPROXY includes client, forward proxy, 
reverse proxy, and server packages, each with binary and library targets. Provides a complete network stack 
using [UDP Multicast](https://en.wikipedia.org/wiki/Multicast) as an intermediate 
route, enabling simple deployment of scalable stream multiplexing and aggregate 
stream processing.

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
  - Stateless: no shared resources between threads. Communications between threads are routed via UDP multicast


### Compatability
- [X] UDP
- [X] TCP (via `proxy` or `reverse_proxy`)
- [X] TLS (partial support for client TLS via `mproxy-forward`. Requires feature `tls` enabled)
- [X] IPv4
- [X] IPv6
- [X] Unix/Linux/Mac
- [X] Windows


## Docs
 - [mproxy-client](https://docs.rs/mproxy-client/)
 - [mproxy-server](https://docs.rs/mproxy-server/)
 - [mproxy-forward](https://docs.rs/mproxy-forward/)
 - [mproxy-reverse](https://docs.rs/mproxy-reverse/)


## Motivation

- Minimal set of networking tools needed for complete encapsulation of distributed systems
- Zero-configuration, simple operation and deployment
- Leverage benefits of UDP protocol:
  - Dead simple stream aggregation
  - Performant stream multiplexing and redistribution
  - UDP multicasting enables stateless, scalable reverse-proxy
- Prioritizing cross-compatability, simplicity, security, and performance

