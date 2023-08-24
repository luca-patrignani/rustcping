use std::{
    net::{TcpStream, ToSocketAddrs, SocketAddr},
    sync::mpsc,
    thread,
    time::{Duration, Instant}, env, error::Error,
};

pub mod printer;
pub mod user_input;
mod tests;

use printer::{Probe, ProbePrinter, StdoutPrinter};
use user_input::parse;

fn get_socket(url: &String, port: u16) -> Result<SocketAddr, std::io::Error> {
    format!("{url}:{port}")
        .to_socket_addrs()
        .map(|v| {v.as_ref()[0]})
}

fn main() -> Result<(), Box<dyn Error>> {
    let user_input = parse(env::args());
    let socket = get_socket(&user_input.url, user_input.port)?;
    let (sx, rx) = mpsc::channel::<Probe>();
    let printer = StdoutPrinter::new(user_input, socket.ip());
    thread::spawn(move || loop {
        let rcvd = rx.recv();
        if let Ok(probe) = rcvd {
            printer.print_probe(probe);
        }
    });
    loop {
        let start = Instant::now();
        let conn_res = TcpStream::connect_timeout(&socket, Duration::from_secs(1));
        let elapsed = start.elapsed();
        let err: Option<std::io::Error> = conn_res.err();
        _ = sx.send(Probe { elapsed, err });
        thread::sleep(Duration::from_secs(1) - elapsed)
    }
}
