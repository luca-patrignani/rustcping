use crate::tracker::{Info, Probe};

pub fn print_probe(info: &Info, probe: &Probe) {
    if probe.err.is_none() {
        print_probe_success(info, probe)
    } else {
        print_probe_failure(info)
    }
}

pub fn print_final_stats(info: &Info) {
    let succ_counter = info.succ_probes_counter;
    let fail_counter = info.fail_probes_counter;
    let total_probes = succ_counter + fail_counter;
    println!(
"
--- example.com (93.184.216.34) TCPing statistics ---
{total_probes} probes transmitted on port 443 | {succ_counter} received, 62.50% packet loss
successful probes:   {succ_counter}
unsuccessful probes: {fail_counter}
last successful probe:   2023-08-25 09:42:47
last unsuccessful probe: 2023-08-25 09:42:46
total uptime:   3 seconds
total downtime: 5 seconds
longest consecutive uptime:   2 seconds from 2023-08-25 09:42:40 to 2023-08-25 09:42:42
longest consecutive downtime: 5 seconds from 2023-08-25 09:42:42 to 2023-08-25 09:42:47
retried to resolve hostname 0 times
rtt min/avg/max: 131.306/132.387/133.411 ms
--------------------------------------
TCPing started at: 2023-08-25 09:42:40
TCPing ended at:   2023-08-25 09:42:48
duration (HH:MM:SS): 00:00:08
"
        )
}

fn print_probe_success(info: &Info, probe: &Probe) {
    let url = &info.user_input.url;
    let ip_addr = info.ip_addr;
    let port = info.user_input.port;
    let elapsed = probe.elapsed.as_millis();
    let counter = info.succ_probes_streak;
    println!("Reply from {url} ({ip_addr}) on port {port} TCP_conn={counter} time={elapsed} ms")
}

fn print_probe_failure(info: &Info) {
    let url = &info.user_input.url;
    let ip_addr = info.ip_addr;
    let port = info.user_input.port;
    let counter = info.fail_probes_streak;
    println!("No reply from {url} ({ip_addr}) on port {port} TCP_conn={counter}")
}
