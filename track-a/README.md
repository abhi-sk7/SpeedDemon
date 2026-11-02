# Track A · Ingest

PulseForge scans each record once using byte offsets into the original line. It interns endpoint names once, maps each `(hour, endpoint)` pair to a compact group, and appends only the latency values needed for exact nearest-rank percentiles. Each group's latency list is sorted once at the end. Groups are then emitted in UTC-hour and unsigned-byte endpoint order.

The solution uses only Rust's standard library. There are no runtime dependencies. Error counts include statuses 500–599; a 499 remains a client error. Integer arithmetic is used for percentile ranks and error parts per million.

## Build and run

The Dockerfile uses only the organizer allowlisted Rust and Debian images, both pinned by digest. The build is offline and the resulting executable reads `/data/requests.log`, then writes only `/out/summary.txt`.

For a local source build with Rust 1.99 or later:

```sh
cargo build --release --offline
```

For a local functional run, provide the same `/data` and `/out` mounts used by the harness. The competition harness remains the authority for correctness and performance.

## Next optimization work

Use actual board run history to compare allocation reuse and an integer-key sort against the current hash grouping. Keep changes only when the official output hash passes and the organizer machine shows a repeatable improvement.
