// -j thread_number -h 

use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::mpsc;
use std::process;

struct Args {
    flag: String,
    ipaddr: IpAddr,
    threads: i64,
}

const DEFAULT_THREAD:i64 = 4;

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("Too few arguments");
        }
        if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Args {flag: String::from(""), ipaddr, threads: DEFAULT_THREAD})
        } else {
            let flag = args[1].clone();
            if flag.contains("h") || flag.contains("--help") {
                if(args.len() == 2) {
                    println!("Usage: -t Number of threads 
                \r\n              -h Printing this help message ");
                return Err("info message");
                } else  {
                    return Err("Too many arguments")
                }
            } 
            else if flag.contains("t") || flag.contains("--thread") {g
                let threads = match args[2].parse::<i64>() {
                    Ok(t) => t,
                    Err(_) => return Err("Invalid thread counts")
                };

                let ipaddr = match   IpAddr::from_str(&args[3]) {
                Ok(addr) => addr,
                Err(_) => return Err("Invalid IP address")
             };
             
             return Ok(Args {flag, threads, ipaddr })
            } else {
                return Err("Invalid arguments")
            }


        }
    } 
}


fn scan(tx, port: u64, addr: IpAddr, num_threads: u64) {
    let p = port + 1;
    loop {
        match Tcp::Stream {
            Ok(_) => {
                print!(".")
                std::io::flush().unwrap();
                tx.transmit(p)
            }
            Err() => {}
        }

        if MAX - p <= DEFAULT_THREAD {
            break
        }
        p = p+1
    }
}


fn main() {
    let args  :Vec<String>= env::args().collect();
    let program = args[0].clone();

    let arguments =  Args::new(&args).unwrap_or_else( |err|  {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{}: An error occured, {}", program, err);
            process::exit(0)
        }
    })

    let num_threads = arguments.num_threads;
    let addr = arguments.addr;

    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(&tx, i, &addr, num_thread);
        })

        drop(tx)
        
        let out_list = vec![]

        for m in rx {
            out_list.push(m);
        }
        
        out_list.sort()

        for p in out_list {
            println!("Port {} is open", p);
        }
    }
}