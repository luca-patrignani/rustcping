use std::{
    net::{TcpStream, ToSocketAddrs, SocketAddr},
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant}, env, error::Error
};

pub mod printer;
pub mod user_input;
mod tests;
mod tracker;

use printer::print_probe;
use tracker::{Probe, Info};
use user_input::parse;

use crate::printer::print_final_stats;

fn get_socket(url: &String, port: u16) -> Result<SocketAddr, std::io::Error> {
    format!("{url}:{port}")
        .to_socket_addrs()
        .map(|v| {v.as_ref()[0]})
}

fn main() -> Result<(), Box<dyn Error>> {
    let user_input = parse(env::args());
    let socket = get_socket(&user_input.url, user_input.port)?;
    let (probe_sx, probe_rx) = channel::<Probe>();
    let (ctrlc_sx, ctrlc_rx) = channel::<()>();
    let mut info = Info::new(user_input, socket.ip());
    let tracker_handle = thread::spawn(move || {
        while let Ok(probe) = probe_rx.recv() {
            info.track(&probe);
            print_probe(&info, &probe);
        }
        print_final_stats(&info)
    });
    ctrlc::set_handler(move || {
        _ = ctrlc_sx.send(());
    }).expect("Error setting Ctrl-C handler");
    let conn_timeout = Duration::from_secs(1);
    let tcping_handle = thread::spawn(move || {
        while ctrlc_rx.try_recv().is_err() {
            let start = Instant::now();
            let conn_res = TcpStream::connect_timeout(&socket, conn_timeout);
            let elapsed = start.elapsed();
            let err: Option<std::io::Error> = conn_res.err();
            if elapsed < conn_timeout {
                thread::sleep(conn_timeout - elapsed);
            }
            _ = probe_sx.send(Probe { elapsed, err });
    }
    });
    _ = tracker_handle.join();
    _ = tcping_handle.join();
    Ok(())
}
