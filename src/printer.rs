use chrono::{DateTime, Utc};

use crate::tracker::{Info, Probe};

pub fn print_probe(info: &Info, probe: &Probe) {
    if probe.err.is_none() {
        print_probe_success(info, probe)
    } else {
        print_probe_failure(info)
    }
}

pub fn print_final_stats(info: &Info) {
    let url = &info.user_input.url;
    let ip_addr = &info.ip_addr;
    let port = info.user_input.port;
    let succ_counter = info.succ_probes_counter;
    let fail_counter = info.fail_probes_counter;
    let total_probes = succ_counter + fail_counter;
    // awful workaround for showing two decimal figures.
    let packet_loss_perc = (fail_counter as f64 / total_probes as f64 * 10000.0).trunc() / 100.0;
    let last_succ_probe = to_string(info.last_succ_probe, "Never succeded".to_owned());
    let last_fail_probe = to_string(info.last_fail_probe, "Never failed".to_owned());
    let total_uptime = info.total_uptime.num_seconds();
    let total_downtime = info.total_downtime.num_seconds();
    println!(
"
--- {url} ({ip_addr}) TCPing statistics ---
{total_probes} probes transmitted on port {port} | {succ_counter} received, {packet_loss_perc}% packet loss
successful probes:   {succ_counter}
unsuccessful probes: {fail_counter}
last successful probe:   {last_succ_probe}
last unsuccessful probe: {last_fail_probe}
total uptime:   {total_uptime} seconds
total downtime: {total_downtime} seconds"
    );
    if succ_counter > 0 {
        let min = info.min_rtt.num_microseconds().unwrap_or_default() as f32 / 1000.0;
        let max = info.max_rtt.num_microseconds().unwrap_or_default() as f32 / 1000.0;
        let avg = info.sum_rtt.num_milliseconds() as f32 / succ_counter as f32;
        println!("rtt min/avg/max: {:.2}/{:.2}/{:.2} ms", min, max, avg);
    }
    println!(
        "
--------------------------------------
TCPing started at: 2023-08-25 09:42:40
TCPing ended at:   2023-08-25 09:42:48
duration (HH:MM:SS): 00:00:08
"
    );
}

fn to_string(time: Option<DateTime<Utc>>, default: String) -> String {
    time.map_or(default, |tt| {
        let mut t = tt.to_string();
        t.truncate(19);
        t
    })
}

fn print_probe_success(info: &Info, probe: &Probe) {
    let url = &info.user_input.url;
    let ip_addr = info.ip_addr;
    let port = info.user_input.port;
    let elapsed = probe.elapsed.num_milliseconds();
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
