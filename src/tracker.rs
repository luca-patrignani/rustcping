use std::{time::Duration, net::IpAddr};

use crate::user_input::UserInput;

pub struct Probe {
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
}

pub struct Info {
    pub user_input: UserInput,
    pub counter: u128,
    pub ip_addr: IpAddr,
}

impl Info {
    pub fn track(&mut self, probe: &Probe) {
        if probe.err.is_none() {
            self.counter += 1;
        } else {
            self.counter = 0;
        }
    }

}
