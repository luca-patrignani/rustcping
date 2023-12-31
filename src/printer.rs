use chrono::Local;

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
    let packet_loss_perc = format!("{:.2}", fail_counter as f64 / total_probes as f64 * 100.0);
    let last_succ_probe = info
        .last_succ_probe
        .map(|t| t.with_timezone(&Local))
        .map_or("Never succeded".to_owned(), |t| {
            format!("{}", t.format("%Y-%m-%d %H:%M:%S"))
        });
    let last_fail_probe = info
        .last_fail_probe
        .map(|t| t.with_timezone(&Local))
        .map_or("Never failed".to_owned(), |t| {
            format!("{}", t.format("%Y-%m-%d %H:%M:%S"))
        });
    let total_uptime = info.total_uptime.num_seconds();
    let total_downtime = info.total_downtime.num_seconds();
    let tcping_start = info
        .start_time
        .map(|t| t.with_timezone(&Local))
        .map_or("".to_owned(), |t| {
            format!("{}", t.format("%Y-%m-%d %H:%M:%S"))
        });
    let tcping_end = info
        .end_time
        .map(|t| t.with_timezone(&Local))
        .map_or("".to_owned(), |t| {
            format!("{}", t.format("%Y-%m-%d %H:%M:%S"))
        });
    let tcping_duration = info.end_time.unwrap() - info.start_time.unwrap();
    let tcping_hours = (tcping_duration.num_seconds() / 60) / 60;
    let tcping_minutes = (tcping_duration.num_seconds() / 60) % 60;
    let tcping_seconds = tcping_duration.num_seconds() % 60;
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
        println!("rtt min/avg/max: {:.2}/{:.2}/{:.2} ms", min, avg, max);
    }
    println!(
        "
--------------------------------------
TCPing started at: {tcping_start}
TCPing ended at:   {tcping_end}
duration (HH:MM:SS): {:0>2}:{:0>2}:{:0>2}",
        tcping_hours, tcping_minutes, tcping_seconds
    );
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
