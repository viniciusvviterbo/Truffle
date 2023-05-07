use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{
    env,
    io::{self, Write},
};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    n_threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f: String = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                n_threads: 4,
            });
        } else {
            let flag: String = args[1].clone();

            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Truffle Hog: A rust based port-sniffer
\nUsage: truffle [OPTIONS] <IP_ADDR>
\nOptions:
\t-h, --help            Print help
\t-j <NUMBER>           Number of threads to use. Default is 4
Arguments:
\t<IP_ADDR>             Target's IP address"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr: IpAddr = match IpAddr::from_str(&args[3]) {
                    Ok(ipaddr) => ipaddr,
                    Err(_) => return Err("Not a valid IP address. Must be IPv4 or IPv6"),
                };
                let n_threads: u16 = match args[2].parse() {
                    Ok(number) => number,
                    Err(_) => return Err("Failed to parse thread number"),
                };
                return Ok(Arguments {
                    n_threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}

fn scan(transmitter: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                transmitter.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name: String = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program_name, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.n_threads;
    let addr = arguments.ipaddr;
    let (transmitter, receiver) = channel();
    for i in 0..num_threads {
        let transmitter = transmitter.clone();

        thread::spawn(move || {
            scan(transmitter, i, addr, num_threads);
        });
    }

    let mut out = vec![];
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
