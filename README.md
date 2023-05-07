<h1 align="center">Truffle</h1>

A rust-based port-sniffer

### Setup

Clone this repository, and:
```bash
cargo build
```

### Usage

```bash
truffle [OPTIONS] <IP_ADDR>
```
or
```bash
cargo run -- [OPTIONS] <IP_ADDR>
```

Inform the number of threads to use during execution and the target's address:

Example:
```
truffle -h
```

```
truffle 192.168.x.x
```

```
truffle -j 1000 192.168.x.x
```

**[GNU GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.html)**
