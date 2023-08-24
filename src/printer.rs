use std::time::Duration;

use crate::user_input::UserInput;

pub struct Probe {
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
}

pub trait ProbePrinter {
    fn print_probe(&self, p: Probe);
}

pub struct StdoutPrinter {
    user_input: UserInput
}

impl ProbePrinter for StdoutPrinter {
    fn print_probe(&self, p: Probe) {
        let elapsed = p.elapsed.as_millis();
        let ip = &self.user_input.url;
        let port = self.user_input.port;
        println!("Reply from 74.6.231.21 ({ip}) on port {port} TCP_conn=1 time={elapsed} ms")
    }
}

impl StdoutPrinter {
    pub fn new(user_input: UserInput) -> StdoutPrinter {
        StdoutPrinter {user_input}
    }
}
