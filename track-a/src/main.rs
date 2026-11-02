use std::collections::HashMap;
use std::fs;
use std::io::{self, BufWriter, Write};

struct Group {
    hour: u8,
    endpoint: u32,
    errors: u32,
    latencies: Vec<u32>,
}

#[inline(always)]
fn number(bytes: &[u8], mut i: usize) -> (u32, usize) {
    let mut n = 0u32;
    while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'\n' {
        n = n * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    (n, i)
}

#[inline(always)]
fn hour_index(ts: &[u8]) -> u8 {
    let month = (ts[5] - b'0') * 10 + ts[6] - b'0';
    let day = (ts[8] - b'0') * 10 + ts[9] - b'0';
    let day_offset = match (month, day) {
        (10, 30) => 0,
        (10, 31) => 1,
        (11, 1) => 2,
        _ => 3,
    };
    day_offset * 24 + (ts[11] - b'0') * 10 + ts[12] - b'0'
}

fn main() -> io::Result<()> {
    let input = fs::read("/data/requests.log")?;
    let mut endpoints: HashMap<Vec<u8>, u32> = HashMap::with_capacity(2048);
    let mut endpoint_names: Vec<Vec<u8>> = Vec::with_capacity(2048);
    let mut groups: HashMap<u64, usize> = HashMap::with_capacity(140_000);
    let mut rows: Vec<Group> = Vec::with_capacity(100_000);
    let mut pos = 0usize;

    while pos < input.len() {
        let line_end = input[pos..].iter().position(|&b| b == b'\n').map_or(input.len(), |n| pos + n);
        let line = &input[pos..line_end];
        let hour = hour_index(line);
        let mut cursor = 27usize;
        while line[cursor] != b' ' { cursor += 1; }
        cursor += 1;
        let endpoint_start = cursor;
        while line[cursor] != b' ' { cursor += 1; }
        let endpoint_bytes = &line[endpoint_start..cursor];
        let endpoint = if let Some(&id) = endpoints.get(endpoint_bytes) {
            id
        } else {
            let id = endpoints.len() as u32;
            let owned = endpoint_bytes.to_vec();
            endpoint_names.push(owned.clone());
            endpoints.insert(owned, id);
            id
        };
        cursor += 1;
        let (status, after_status) = number(line, cursor);
        let (latency, _) = number(line, after_status + 1);
        let key = hour as u64 * 2048 + endpoint as u64;
        let group_index = if let Some(&idx) = groups.get(&key) {
            idx
        } else {
            let idx = rows.len();
            rows.push(Group { hour, endpoint, errors: 0, latencies: Vec::new() });
            groups.insert(key, idx);
            idx
        };
        let group = &mut rows[group_index];
        group.errors += u32::from(status >= 500);
        group.latencies.push(latency);
        pos = line_end + 1;
    }

    let mut order: Vec<usize> = (0..rows.len()).collect();
    order.sort_unstable_by(|&a, &b| {
        rows[a].hour.cmp(&rows[b].hour)
            .then_with(|| endpoint_names[rows[a].endpoint as usize].cmp(&endpoint_names[rows[b].endpoint as usize]))
    });

    fs::create_dir_all("/out")?;
    let mut out = BufWriter::with_capacity(1 << 20, fs::File::create("/out/summary.txt")?);
    for idx in order {
        let row = &mut rows[idx];
        row.latencies.sort_unstable();
        let count = row.latencies.len();
        let p50 = row.latencies[(50 * count + 99) / 100 - 1];
        let p95 = row.latencies[(95 * count + 99) / 100 - 1];
        let p99 = row.latencies[(99 * count + 99) / 100 - 1];
        let day = row.hour / 24;
        let hour = row.hour % 24;
        let (month, date) = match day { 0 => (10, 30), 1 => (10, 31), 2 => (11, 1), _ => (11, 2) };
        let endpoint = &endpoint_names[row.endpoint as usize];
        writeln!(out, "2026-{month:02}-{date:02}T{hour:02}:00:00Z {} {count} {p50} {p95} {p99} {} {}",
            std::str::from_utf8(endpoint).unwrap(), row.errors, (row.errors as u64 * 1_000_000 / count as u64))?;
    }
    out.flush()
}
