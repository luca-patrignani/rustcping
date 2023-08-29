use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};

use crate::user_input::UserInput;

pub struct Probe {
    pub start: DateTime<Utc>,
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
    pub cycle_duration: Duration,
}

pub struct Info {
    pub user_input: UserInput,
    pub succ_probes_streak: u128,
    pub fail_probes_streak: u128,
    pub succ_probes_counter: u128,
    pub fail_probes_counter: u128,
    pub ip_addr: IpAddr,
    pub last_succ_probe: Option<DateTime<Utc>>,
    pub last_fail_probe: Option<DateTime<Utc>>,
    pub total_uptime: Duration,
    pub total_downtime: Duration,
    pub min_rtt: Duration,
    pub max_rtt: Duration,
    pub sum_rtt: Duration,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Info {
    pub fn new(user_input: UserInput, ip_addr: IpAddr) -> Info {
        Info {
            user_input,
            ip_addr,
            succ_probes_streak: 0,
            fail_probes_streak: 0,
            succ_probes_counter: 0,
            fail_probes_counter: 0,
            last_succ_probe: None,
            last_fail_probe: None,
            total_uptime: Duration::zero(),
            total_downtime: Duration::zero(),
            min_rtt: Duration::max_value(),
            max_rtt: Duration::min_value(),
            sum_rtt: Duration::zero(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn track(&mut self, probe: &Probe) {
        if probe.err.is_none() {
            self.succ_probes_streak += 1;
            self.fail_probes_streak = 0;
            self.succ_probes_counter += 1;
            self.last_succ_probe = Some(probe.start);
            self.total_uptime = self.total_uptime + probe.cycle_duration;
            self.min_rtt = Duration::min(self.min_rtt, probe.elapsed);
            self.max_rtt = Duration::max(self.max_rtt, probe.elapsed);
            self.sum_rtt = self.sum_rtt + probe.elapsed;
        } else {
            self.succ_probes_streak = 0;
            self.fail_probes_streak += 1;
            self.fail_probes_counter += 1;
            self.last_fail_probe = Some(probe.start);
            self.total_downtime = self.total_downtime + probe.cycle_duration;
        }
        if self.start_time.is_none() {
            self.start_time = Some(probe.start)
        }
        self.end_time = Some(probe.start + probe.elapsed);
    }
}
