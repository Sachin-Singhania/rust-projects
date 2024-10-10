use std::env;
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;
struct Arguments{
    flag: String,
    ipaddress:IpAddr,
    threads:u16,
} 

const MAX:u16=65535;

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {

        if args.len()<2 {
            return Err("Less than expected Arguments");
        }else if args.len()> 4 {
            return Err("More than expected Arguments");
            
        };
        let ipadd=match args[1].parse::<IpAddr>(){
            Ok(ip)=>ip,
            Err(_)=>return Err("Invalid Ip address"),
        };
        let threads=if args.len()==4{
            match args[3].parse::<u16>() {
                Ok(t)=>t,
                Err(_)=> return Err("Invalid Prompt"),
            }
        }else{
            4
        };
        let f=args[2].clone();
        if f.contains("-h") || f.contains("-help") && args.len() == 2{
            println!("Usuage -j how to select threads you want \n\r -h or -help for help");
            return Err("help");
        }else if f.contains("-j") {
           
        }
        
        return Ok(Arguments{flag:f,ipaddress:ipadd,threads});
    }

}

fn scan(tx: Sender<u16>, start_port: u16, peer: IpAddr, num_threads: u16){
    let mut port=start_port+1;
    loop {
        match TcpStream::connect((peer,port)) {
            Ok(e)=>{
                println!("Successfully Connected");
                println!("{:?}",e);

                tx.send(port).unwrap();
            },
            Err(_)=>{
            }
        }
        if MAX-port<=num_threads {
            break;
        }
        port+=num_threads;
    }
}

fn main() {
    let args:Vec<String>=env::args().collect();
    for i in &args{
        println!("{i}");
    }
    let arguments=Arguments::new(&args).unwrap_or_else(|err|{
        eprint!("Error Passing Arguments {}",err);
        std::process::exit(1);
    });
    println!("Flag : {} ,IP Address: {}, Threads: {}",arguments.flag, arguments.ipaddress, arguments.threads);
    println!("{args:?}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    let (tx, rx) = channel();
    for i in 0..arguments.threads{
        let txn=tx.clone();
        thread::spawn(move||{
            scan(txn,i,arguments.ipaddress,arguments.threads);
        });
    }
    let mut handles = vec![];
    drop(tx);
    for p in rx {
        handles.push(p);
    }
    handles.sort();
    for v in handles{
        println!("{v} is open");
    }
}
