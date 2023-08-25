use std::net::IpAddr;

use chrono::{Utc, Duration, DateTime};

use crate::user_input::UserInput;

pub struct Probe {
    pub time: DateTime<Utc>,
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
}

pub struct Info {
    pub user_input: UserInput,
    pub succ_probes_streak: u128,
    pub fail_probes_streak: u128,
    pub succ_probes_counter: u128,
    pub fail_probes_counter: u128,
    pub ip_addr: IpAddr,
    pub last_succ_probe: Option<DateTime<Utc>>,
    pub last_fail_probe: Option<DateTime<Utc>>
}

impl Info {
    pub fn new(user_input: UserInput, ip_addr: IpAddr) -> Info {
        Info{user_input, ip_addr, 
            succ_probes_streak: 0, 
            fail_probes_streak: 0,
            succ_probes_counter: 0,
            fail_probes_counter: 0,
            last_succ_probe: None,
            last_fail_probe: None
        }
    }

    pub fn track(&mut self, probe: &Probe) {
        if probe.err.is_none() {
            self.succ_probes_streak += 1;
            self.fail_probes_streak = 0;
            self.succ_probes_counter += 1;
            self.last_succ_probe = Some(probe.time)
        } else {
            self.succ_probes_streak = 0;
            self.fail_probes_streak += 1;
            self.fail_probes_counter += 1;
            self.last_fail_probe = Some(probe.time)
        }
    }

}
