use std::{time::Duration, net::IpAddr};

use crate::user_input::UserInput;

pub struct Probe {
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
}

pub struct Info {
    pub user_input: UserInput,
    pub succ_probes_counter: u128,
    pub fail_probes_counter: u128,
    pub ip_addr: IpAddr,
}

impl Info {
    pub fn track(&mut self, probe: &Probe) {
        if probe.err.is_none() {
            self.succ_probes_counter += 1;
            self.fail_probes_counter = 0;
        } else {
            self.succ_probes_counter = 0;
            self.fail_probes_counter += 1;
        }
    }

}
