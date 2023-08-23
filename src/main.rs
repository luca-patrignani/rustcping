use std::{
    net::{SocketAddr, TcpStream},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub mod printer;
pub mod user_input;

use printer::{Probe, ProbePrinter, StdoutPrinter};
use user_input::UserInput;

fn main() {
    let user_input = UserInput{socket: SocketAddr::from(([93, 184, 216, 34], 443))}; // example.com port 443
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
