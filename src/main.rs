use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::net::UdpSocket;

/// Simple command line tool to try out UDP hole punching.
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// IP address of the other peer to connect to.
    addr: String,

    /// The UDP source port (the destination port of the other peer).
    #[clap(long)]
    src_port: u16,

    /// The UDP destination port (the source port of the other peer).
    #[clap(long)]
    dest_port: u16,

    /// The message to send to the other peer.
    #[clap(long, default_value = "Hello World")]
    msg: String,
}

const PUNCH_INTERVAL_IN_MILLIS: f64 = 10_000.0; // the punch time is every 10,000 milliseconds

fn main() {
    let args = Args::parse();

    // Prepare UDP socket:
    let socket = UdpSocket::bind(("0.0.0.0", args.src_port)).expect("couldn't bind to address");

    // Calculate and print punch time:
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let punch_time = Duration::from_millis((((now.as_millis() as f64)/PUNCH_INTERVAL_IN_MILLIS).ceil() * PUNCH_INTERVAL_IN_MILLIS) as u64);
    println!("Punching in {:.3} seconds...", ((punch_time-now).as_millis() as f64)/1000.0);
    
    // Wait until punch time:
    while SystemTime::now().duration_since(UNIX_EPOCH).unwrap() < punch_time {
        // wait...
    }

    // Punch:
    socket.send_to(args.msg.as_ref(), (args.addr, args.dest_port)).expect("couldn't send data");

    let mut buf = [0; 64];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    let filled_buf = &mut buf[..number_of_bytes];

    println!("Punch succeeded! Received {} bytes '{}' from {}",
        number_of_bytes,
        String::from_utf8(filled_buf.to_vec()).unwrap_or("<UTF-8 error>".to_string()),
        src_addr
    );
}
