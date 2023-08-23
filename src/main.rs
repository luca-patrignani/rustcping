use std::{
    net::{SocketAddr, TcpStream},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub mod printer;

use printer::{Probe, ProbePrinter};

fn main() {
    let (sx, rx) = mpsc::channel::<Probe>();
    // let printer = StdoutPrinter::new();
    thread::spawn(move || loop {
        let rcvd = rx.recv();
        if let Ok(probe) = rcvd {
            printer::StdoutPrinter::print_probe(probe);
        }
    });
    let addr = SocketAddr::from(([93, 184, 216, 34], 443)); // example.com port 443
    loop {
        let start = Instant::now();
        let conn_res = TcpStream::connect_timeout(&addr, Duration::from_secs(1));
        let elapsed = start.elapsed();
        let err: Option<std::io::Error> = conn_res.err();
        _ = sx.send(Probe { elapsed, err });
        thread::sleep(Duration::from_secs(1) - elapsed)
    }
}
