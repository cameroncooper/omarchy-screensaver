# Screensaver observation prototype

The native screensaver publishes read-only animation state on the user session bus. This prototype leaves its effect selection, engine clock, monitor layout, rendering cadence, input handling, and transition decisions in place. No observer is required for normal operation.

- Bus name: `org.omarchy.Screensaver` (owned only while the native overlay runs)
- Object: `/org/omarchy/Screensaver`
- Interface: `org.omarchy.Screensaver1`
- Method: `GetState() -> Snapshot`
- Signal: `StateChanged(Snapshot)`

`Snapshot` is a D-Bus struct with fields in this order:

| Field | Type | Meaning |
| --- | --- | --- |
| revision | `t` | Monotonically increasing within this bus owner; starts at zero before the first effect |
| active | `b` | Whether a reproducible effect description is available |
| engine | `s` | Initialization compatibility identifier, currently `ttfx-0.5.1` |
| effect | `s` | Selected effect name |
| input | `s` | Resolved input text, including its existing formatting |
| rng_state | `at` | Four xoshiro state words captured after selection and before `EngineCtx`/layout initialization |

Geometry and anchoring are deliberately local to each observer. TTY preparation/cleanup options are not part of this interface. The description reproduces the chosen initialization with the defaults of the identified engine version. It does not promise matching trajectories or completion times for different grids. There is no seek operation or per-frame timing stream.

Subscribe to owner changes and `StateChanged` before calling `GetState` on the current unique owner. Accept only that owner's signals; discard revisions already applied. Reset the revision when the owner changes. An owner disappearing terminates observation. There is no D-Bus activation file: a read must not launch the screensaver.

The animation exposes a consumable effect-start description. Only the publisher assigns revisions. It stores the latest snapshot and wakes a bus worker through a capacity-one channel using `try_send`; dropping the publisher closes the channel and ends the worker. There are no explicit shutdown flags or condition variables. It performs no bus I/O, never waits for an observer, and does not maintain an event backlog. The bus worker coalesces superseded transitions and serves the latest snapshot. Missing or failed session-bus access leaves normal rendering operational. Input over 64 KiB publishes an inactive snapshot so the observer channel remains bounded without truncating the actual input. Readers can inspect state but cannot start, stop, select, or control effects through this interface.

Initial construction, an actual geometry rebuild, and completion followed by a new effect produce new revisions. Ordinary frames do not. Late observers initialize the current effect from its beginning and adopt subsequent transitions. This is a current-state stream, not a durable history.

The RNG snapshot is necessary: publishing the original CLI seed with a pinned effect would omit random draws consumed while selecting that effect. The engine test reconstructs rain, matrix, print, and a random selection from the published state and compares their actual frame strings.

## Verification

`make check` covers baseline CLI/render behavior and RNG reconstruction. The companion Touch Bar checkout has `scripts/test-native-screensaver-observer.py` (private bus; snapshot, transition, publisher drop while the process remains alive, and owner restart) and `scripts/test-native-screensaver-pacing.py` (real GLES output under the 30 Hz policy). `examples/observer-fixture.rs` provides a desktop-free publisher for these tests.

This is a local development build, not an upstream release or submitted PR.
