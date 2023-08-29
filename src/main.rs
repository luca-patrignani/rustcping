use std::{
    env,
    error::Error,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::mpsc::channel,
    thread,
};

pub mod printer;
mod tests;
mod tracker;
pub mod user_input;

use chrono::{Duration, Utc};
use printer::print_probe;
use tracker::{Info, Probe};
use user_input::parse;

use crate::printer::print_final_stats;

fn get_socket(url: &String, port: u16) -> Result<SocketAddr, std::io::Error> {
    format!("{url}:{port}")
        .to_socket_addrs()
        .map(|v| v.as_ref()[0])
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
    let conn_timeout = Duration::seconds(1);
    let tcping_handle = thread::spawn(move || {
        while ctrlc_rx.try_recv().is_err() {
            let start = Utc::now();
            let conn_res = TcpStream::connect_timeout(&socket, conn_timeout.to_std().unwrap());
            let elapsed = Utc::now() - start;
            let err: Option<std::io::Error> = conn_res.err();
            if elapsed < conn_timeout {
                thread::park_timeout((conn_timeout - elapsed).to_std().unwrap());
            }
            _ = probe_sx.send(Probe {
                elapsed,
                err,
                start,
                cycle_duration: conn_timeout,
            });
        }
    });
    ctrlc::set_handler(move || {
        _ = ctrlc_sx.send(());
        tcping_handle.thread().unpark();
    })
    .expect("Error setting Ctrl-C handler");
    _ = tracker_handle.join();
    // the threads close in this order: ctrlc => tcping => tracker => main
    Ok(())
}
