use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

const MAX_PORT: u16 = 65535;
const IP_FALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Arguments {
    #[bpaf(long, short, argument("Address"), fallback(IP_FALLBACK))]
    /// The address to be sniffed. Must be a valid IPv4 address. Falls back to localhost
    pub ipaddr: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        fallback(1u16),
        guard(start_port_guard, "Must be greater than 0")
    )]
    /// The start port for the sniffer. Must be greater than 0  
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        fallback(MAX_PORT),
        guard(end_port_guard, "Must be less than or equal to 65535")
    )]
    /// The end port for the sniffer. Must be less than or equal to MAX_PORT
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX_PORT
}

async fn scan(transmitter: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            transmitter.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts: Arguments = arguments().run();

    let (transmitter, receiver) = channel();
    for i in opts.start_port..opts.end_port {
        let transmitter = transmitter.clone();

        task::spawn(async move { scan(transmitter, i, opts.ipaddr).await });
    }

    let mut out: Vec<u16> = vec![];
    drop(transmitter);
    for p in receiver {
        out.push(p);
    }

    println!("");

    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
