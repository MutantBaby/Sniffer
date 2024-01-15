use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};

use tokio::net::TcpStream;
use tokio::task;

use bpaf::Bpaf;

const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Cli {
    #[bpaf(short, long, fallback(IPFALLBACK))]
    /// The address that you want to sniff. Must be a valid IPv4 address. Falls back to 127.0.0.1
    pub address: IpAddr,

    #[bpaf(
        long("start"),
        short('s'),
        fallback(1u16),
        guard(start_port_guard, "Must be greater than 0")
    )]
    /// The start port for the sniffer. (Must be greater than 0)
    pub start_port: u16,

    #[bpaf(
        long("end"),
        short('e'),
        fallback(MAX),
        guard(end_port_guard, "Must be less than 65535")
    )]
    /// The end port for the sniffer. (Must be less than 65535)
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= 65535
}

// fn scan(x_transmitter: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
//     let mut port = start_port + 1;
//     loop {
//         match TcpStream::connect((addr, port)) {
//             Ok(_) => {
//                 print!(".");
//                 io::stdout().flush().unwrap();
//                 x_transmitter.send(port).unwrap();
//             }
//             Err(_) => {}
//         };

//         if (MAX - port) <= num_threads {
//             break;
//         }

//         port += num_threads;
//     }
// }

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let program: String = args[0].clone();

    // let arguments: Cli = Cli::new(&args).unwrap_or_else(|err| {
    //     if err.contains("help") {
    //         process::exit(0);
    //     } else {
    //         eprintln!("{} problem parsing arguments: {}", program, err);
    //         process::exit(0);
    //     }
    // });

    // let num_threads = arguments.threads;
    // let (x_transmitter, x_receiver) = channel();

    // for i in 0..num_threads {
    //     let x_transmitter = x_transmitter.clone();

    //     thread::spawn(move || {
    //         scan(x_transmitter, i, arguments.ipaddr, num_threads);
    //     });
    // }

    // let mut out = vec![];
    // drop(x_transmitter);

    // for p in x_receiver {
    //     out.push(p);
    // }

    // println!(" ");
    // out.sort();
    // for v in out {
    //     println!("{} is open", v);
    // }

    // println!("Args {:?}", args);
}
