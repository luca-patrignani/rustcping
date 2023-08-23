use std::{
    net::{SocketAddr, TcpStream},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

struct Probe {
    elapsed: Duration,
    err: Option<std::io::Error>,
}

fn main() {
    let (sx, rx) = mpsc::channel::<Probe>();
    thread::spawn(move || loop {
        let rcvd = rx.recv();
        if rcvd.is_ok() {
            let probe = rcvd.unwrap();
            println!("{} {}", probe.err.is_some(), probe.elapsed.as_millis());
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
