# Network Dispatcher
Streams data from a file-like descriptor or Unix socket over the network to one or more logging servers

Compatible with:

- [ ] TCP (Planned)
- [X] UDP
- [X] IPv4
- [X] IPv6 
- [X] Unix
- [X] Windows (untested)

Features
- [X] Network data transfer
- [X] Log data from multiple clients simultaneously
- [X] Stream to multiple logging servers simultaneously 
- [X] Minimal (2 dependencies, compiled size ~300kb)
- [X] Extremely fast




### Server

Start the logging server
```
cargo run --bin server --release -- --port 9920 --client_addr 127.0.0.1 --client_addr 127.0.0.2 --path logfile.log
```

### Client

Stream data from the client to the logging server:
```
cargo run --bin client --release -- --port 9920 --listen_addr 0.0.0.0 --path /dev/random
```

