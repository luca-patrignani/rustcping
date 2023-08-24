use std::{time::Duration, net::IpAddr};

use crate::user_input::UserInput;

pub struct Probe {
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
}

pub trait ProbePrinter {
    fn print_probe(&self, p: Probe);
}

pub struct StdoutPrinter {
    user_input: UserInput,
    current_ip_addr: IpAddr,
}

impl ProbePrinter for StdoutPrinter {
    fn print_probe(&self, p: Probe) {
        let elapsed = p.elapsed.as_millis();
        let url = &self.user_input.url;
        let port = self.user_input.port;
        let ip_addr = &self.current_ip_addr;
        println!("Reply from {url} ({ip_addr}) on port {port} TCP_conn=1 time={elapsed} ms")
    }
}

impl StdoutPrinter {
    pub fn new(user_input: UserInput, ip_addr: IpAddr) -> StdoutPrinter {
        StdoutPrinter {user_input, current_ip_addr: ip_addr }
    }
}
