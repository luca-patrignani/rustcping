use std::{
    env,
    error::Error,
    net::{SocketAddr, ToSocketAddrs},
    sync::mpsc::channel,
    thread,
};

pub mod pinger;
pub mod printer;
mod tcping;
mod tracker;
mod user_input;

use pinger::{PingTimeout, PingWithoutTimeout, Pinger};
use printer::print_probe;
use tcping::tcping;
use tracker::Info;
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
    let conn_timeout = user_input.timeout;
    let (probe_sx, probe_rx) = channel();
    let (ctrlc_sx, ctrlc_rx) = channel();
    let mut info = Info::new(user_input.clone(), socket.ip());
    let tracker_handle = thread::spawn(move || {
        while let Ok(probe) = probe_rx.recv() {
            info.track(&probe);
            print_probe(&info, &probe);
        }
        print_final_stats(&info)
    });
    let tcping_th = std::thread::current();
    ctrlc::set_handler(move || {
        _ = ctrlc_sx.send(());
        tcping_th.unpark();
    })
    .expect("Error setting Ctrl-C handler");

    let pinger: Box<dyn Pinger> = if let Some(t) = conn_timeout {
        Box::new(PingTimeout {
            socket,
            conn_timeout: t.to_std().unwrap(),
        })
    } else {
        Box::new(PingWithoutTimeout { socket })
    };
    tcping(probe_sx, ctrlc_rx, pinger.as_ref(), user_input);
    _ = tracker_handle.join();
    // the threads close in this order: ctrlc => tcping => tracker => main
    Ok(())
}
