<h1 align="center">Truffle</h1>

A rust-based port-sniffer

### Setup

Clone this repository, and:
```bash
cargo build
```

### Execution

If there is just the source code on your machine, run:
```bash
cargo run --
```

If the executable is already installed, run:
```bash
truffle
```

### Usage

```bash
Usage: [-i Address] [-s ARG] [-e ARG]

Available options:
    -i, --ipaddr <Address>  The address to be sniffed. Must be a valid IPv4 address. Falls back to
                       localhost
    -s, --start <ARG>  The start port for the sniffer. Must be greater than 0
    -e, --end <ARG>    The end port for the sniffer. Must be less than or equal to MAX_PORT
    -h, --help         Prints help information
```

**[GNU GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.html)**
