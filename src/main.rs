use std::{
    env::{self},
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{channel, Sender},
    thread,
};

const MAX: u16 = 65535;

struct Cli {
    flag: String,
    threads: u16,
    ipaddr: IpAddr,
}

impl Cli {
    fn new(args: &Vec<String>) -> Result<Cli, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f: String = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            Ok(Cli {
                flag: String::from(""),
                threads: 4,
                ipaddr,
            })
        } else {
            let flag: String = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \r\n -h or -help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr: IpAddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not valid IPADDR; Must be IPv4 OR IPv6"),
                };

                let threads: u16 = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread numbers"),
                };

                return Ok(Cli {
                    flag,
                    ipaddr,
                    threads,
                });
            } else {
                Err("Invalid syntax")
            }
        }
    }
}

fn scan(x_transmitter: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                x_transmitter.send(port).unwrap();
            }
            Err(_) => {}
        };

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    let arguments: Cli = Cli::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let (x_transmitter, x_receiver) = channel();

    for i in 0..num_threads {
        let x_transmitter = x_transmitter.clone();

        thread::spawn(move || {
            scan(x_transmitter, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(x_transmitter);

    for p in x_receiver {
        out.push(p);
    }

    println!(" ");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }

    println!("Args {:?}", args);
}
