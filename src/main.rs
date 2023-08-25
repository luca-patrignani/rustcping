use std::{
    net::{TcpStream, ToSocketAddrs, SocketAddr},
    sync::mpsc,
    thread,
    time::{Duration, Instant}, env, error::Error,
};

pub mod printer;
pub mod user_input;
mod tests;
mod tracker;

use printer::print_probe;
use tracker::{Probe, Info};
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
    let mut info = Info{user_input, succ_probes_counter: 0, fail_probes_counter: 0, ip_addr: socket.ip() };
    thread::spawn(move || {
        loop {
            if let Ok(probe) = rx.recv() {
                info.track(&probe);
                print_probe(&info, &probe);
            }
    }
    });
    let conn_timeout = Duration::from_secs(1);
    loop {
        let start = Instant::now();
        let conn_res = TcpStream::connect_timeout(&socket, conn_timeout);
        let elapsed = start.elapsed();
        let err: Option<std::io::Error> = conn_res.err();
        if elapsed < conn_timeout {
            thread::sleep(conn_timeout - elapsed)
        }
        _ = sx.send(Probe { elapsed, err });
    }
}
