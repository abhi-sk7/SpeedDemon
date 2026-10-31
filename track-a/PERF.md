# Performance log

The official live-board run history is the source of performance measurements. No organizer run ID or timing has been recorded in this log yet; none are fabricated.

## Current design

- One sequential read into a contiguous byte buffer; timestamps and numeric fields are parsed directly from bytes.
- Endpoint strings are interned once. A compact integer key groups records by hour and endpoint.
- Latencies are retained per group because exact nearest-rank percentiles must be values present in the input. Each list is sorted once.
- Output groups are ordered by UTC hour and endpoint bytes, then formatted into a buffered writer.
- Runtime dependencies: zero.

## Measurement ledger

| Revision | Change | Correctness hash | Board run ID | Result |
| --- | --- | --- | --- | --- |
| Initial | Custom byte parser, interned endpoints, grouped exact percentile sort | Pending | Pending | Pending |

After each board run, add the run ID and measured time/memory/source-size/dependency axes here. Keep failed and reverted experiments with their actual results so the optimization history remains auditable.

## Next experiments

1. Compare current `HashMap` grouping against sorting compact `(hour, endpoint, latency, error)` records before aggregation.
2. Measure output sort and allocator cost separately with representative generated data.
3. Check whether reusing latency storage or a small-group selection strategy improves wall time without increasing memory or risking percentile correctness.
