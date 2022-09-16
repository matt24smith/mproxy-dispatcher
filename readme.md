# Network Dispatcher
Streams data from a file-like descriptor or Unix socket over the network to one or more logging servers


### Goals

- [ ] TCP 
- [X] UDP
- [X] IPv4
- [X] IPv6 
- [X] Unix Support
- [X] Windows Support (untested)
- [X] Multicast Support (currently IPv4 only)
- [X] Minimal (2 dependencies, compiled size ~300kb)
- [ ] Log data from multiple clients simultaneously
- [X] Stream to multiple logging servers simultaneously 



### Server

Start the logging server
```
cargo run --bin server --release -- --port 9920 --client_addr 127.0.0.1
```

### Client

Stream data from the client to the logging server:
```
cargo run --bin client --release -- --port 9920 --listen_addr 0.0.0.0 --path /dev/random
```


#### Multicasting

To enable [multicasting](http://https://en.wikipedia.org/wiki/Multicast), use the multicast address range e.g. `224.0.0.1` or `FF02::1`. Not to be confused with point-to-multipoint logging.

