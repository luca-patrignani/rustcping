use crate::tracker::{Info, Probe};

pub fn print_probe(info: &Info, probe: &Probe) {
    if probe.err.is_none() {
        print_probe_success(info, probe)
    } else {
        print_probe_failure(info)
    }
}

fn print_probe_success(info: &Info, probe: &Probe) {
    let url = &info.user_input.url;
    let ip_addr = info.ip_addr;
    let port = info.user_input.port;
    let elapsed = probe.elapsed.as_millis();
    let counter = info.succ_probes_counter;
    println!("Reply from {url} ({ip_addr}) on port {port} TCP_conn={counter} time={elapsed} ms")
}

fn print_probe_failure(info: &Info) {
    let url = &info.user_input.url;
    let ip_addr = info.ip_addr;
    let port = info.user_input.port;
    let counter = info.fail_probes_counter;
    println!("No reply from {url} ({ip_addr}) on port {port} TCP_conn={counter}")
}
