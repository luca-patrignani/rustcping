use std::{
    net::TcpStream,
    sync::mpsc,
    thread,
    time::{Duration, Instant}, env,
};

pub mod printer;
pub mod user_input;
mod tests;

use printer::{Probe, ProbePrinter, StdoutPrinter};
use user_input::parse;

fn main() {
    let user_input = parse(env::args());
    let socket = user_input.socket;
    let (sx, rx) = mpsc::channel::<Probe>();
    let printer = StdoutPrinter::new(user_input);
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
