use std::{
    env::{self},
    net::IpAddr,
    process,
    str::FromStr,
};

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

    println!("Args {:?}", args);
}
