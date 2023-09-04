use std::{
    env,
    error::Error,
    net::{SocketAddr, ToSocketAddrs},
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

pub mod pinger;
pub mod printer;
mod tests;
mod tracker;
pub mod user_input;

use chrono::Utc;
use pinger::{PingTimeout, PingWithoutTimeout, Pinger};
use printer::print_probe;
use tracker::{Info, Probe};
use user_input::{parse, UserInput};

use crate::printer::print_final_stats;

fn get_socket(url: &String, port: u16) -> Result<SocketAddr, std::io::Error> {
    format!("{url}:{port}")
        .to_socket_addrs()
        .map(|v| v.as_ref()[0])
}

fn tcping<P: Pinger + ?Sized>(
    probe_sx: Sender<Probe>,
    closer_rx: Receiver<()>,
    pinger: &P,
    user_input: UserInput,
) {
    let mut i = 0;
    while closer_rx.try_recv().is_err() && user_input.probes_count.map_or(true, |c| {i < c}) {
        let start = Utc::now();
        let err = pinger.ping();
        let elapsed = Utc::now() - start;
        if elapsed < user_input.interval_between_probes {
            thread::park_timeout((user_input.interval_between_probes - elapsed).to_std().unwrap());
        }
        _ = probe_sx.send(Probe {
            elapsed,
            err,
            start,
            cycle_duration: Utc::now() - start,
        });
        i += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let user_input = parse(env::args());
    let socket = get_socket(&user_input.url, user_input.port)?;
    let conn_timeout = user_input.timeout;
    let (probe_sx, probe_rx) = channel::<Probe>();
    let (ctrlc_sx, ctrlc_rx) = channel::<()>();
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
