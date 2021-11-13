# Rust Packet Filtering
## Setup
```sh
sudo iptables -A OUTPUT -d 127.0.0.1 -p tcp --dport 9999 -j NFQUEUE --queue-num 0
ping 127.0.0.1
```

```sh
cargo build
sudo ./target/debug/rust-fw # setcap isn't working currently
```

### Cleanup
```
sudo iptables -D OUTPUT -d 127.0.0.1 -p tcp --dport 9999 -j NFQUEUE --queue-num 0
```
