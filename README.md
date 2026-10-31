# speedDemon · PulseForge

**Fast, private HTTP log analysis.** PulseForge turns structured access logs into hourly endpoint summaries with exact nearest-rank p50/p95/p99 latency, 5xx counts and error rates.

The project has two ways to use it:

- **Local web workbench:** open [`app/index.html`](app/index.html), choose a log file, analyze it in a browser worker, inspect the results, and download `summary.txt`. The file is processed locally and is never uploaded.
- **Competition runner:** the zero-dependency Rust implementation in [`track-a/`](track-a/) reads `/data/requests.log` and writes the byte-stable `/out/summary.txt` required by Speed Demon 2026 Track A.

## Why it is useful

Raw logs are hard to scan when an incident spans many endpoints and hours. This gives an engineer a compact, sortable view of traffic volume, tail latency and server-error rates without sending production data to a hosted analyzer. Percentiles use the exact nearest-rank definition; 499 responses do not count as 5xx errors.

## Run the web workbench

Open `app/index.html` in a current browser. Drop in an ASCII, space-delimited request log with timestamp, method, endpoint, status and latency in microseconds. Analysis runs in a Web Worker to keep the interface responsive. The first 1,000 output groups are previewed; the download contains every group.

The analyzer expects the Speed Demon Track A log shape and fixed 72-hour date span. For other formats or date ranges, adapt the timestamp and parser rules before using it.

## Competition entry

See [`track-a/README.md`](track-a/README.md) for the offline Docker build and execution contract. `track-a/PERF.md` records optimization evidence without inventing timings or board run IDs; add the actual board results as they arrive.

## License

MIT. See [`LICENSE`](LICENSE).

