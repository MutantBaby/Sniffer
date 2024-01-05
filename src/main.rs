use std::{
    env::{self, args},
    net::{AddrParseError, IpAddr},
    str::FromStr,
};

struct Cli {
    flag: String,
    threads: u16,
    ipadder: IpAddr,
}

impl Cli {
    // fn new(args: &[String]) -> Result<Cli, &'static str> {
    //     if args.len() < 2 {
    //         return Err("Not enough arguments");
    //     } else if args.len() > 4 {
    //         return Err("Too many arguments");
    //     }

    //     let ip: String = args[1].clone();
    //     if let Ok(ipadder) = IpAddr::from_str(&ip) {
    //         Ok(Cli {
    //             flag: String::from(""),
    //             threads: 4,
    //             ipadder,
    //         })
    //     } else {
    //         Err("Error in handling arguments")
    //     }
    // }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Args {:?}", args);
}
