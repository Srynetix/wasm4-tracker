# Simple music tracker (code-only) for [WASM-4] (with [wasm4-sx])

Build music for the WASM-4 using code or a YAML file. Generates a binary blob to be parsed with [wasm4-sx]'s [TrackReader].  
It's a work in progress.

## Sample

Here is a sample as a YAML file.

```yml
tones:
  snare:
    frequency:
      first: 220
    duration:
      decay: 4
      sustain: 4
      release: 4
    volume:
      sustain: 8
    flags:
      channel: Noise

  synth2:
    frequency:
      first: 440
      second: 880
    duration:
      sustain: 7
    volume:
      sustain: 12
    flags:
      channel: Pulse2

  bass:
    frequency:
      first: 110
    duration:
      sustain: 12
    volume:
      sustain: 25
    flags:
      channel: Triangle

  kick:
    frequency:
      first: 110
      second: 55
    duration:
      sustain: 7
    volume:
      sustain: 50
    flags:
      channel: Triangle

  riser:
    frequency:
      first: 55
      second: 8000
    duration:
      sustain: 120
      release: 30
    volume:
      sustain: 25
    flags:
      channel: Pulse1

patterns:
  intro:
    - {frame: 0, notes: [{note: D2, tone: synth2}]}
    - {frame: 30, notes: [{note: E2, tone: synth2}]}
    - {frame: 60, notes: [{note: F2, tone: synth2}]}
    - {frame: 90, notes: [{note: G2, tone: synth2}]}
    - {frame: 120, notes: []}

  arp:
    - {frame: 0, notes: [{note: D2, tone: synth2}]}
    - {frame: 8, notes: [{note: D3, tone: synth2}]}
    - {frame: 15, notes: [{note: D4, tone: synth2}]}
    - {frame: 23, notes: [{note: D3, tone: synth2}]}
    - {frame: 30, notes: [{note: D2, tone: synth2}]}
    - {frame: 38, notes: [{note: D3, tone: synth2}]}
    - {frame: 45, notes: [{note: D4, tone: synth2}]}
    - {frame: 53, notes: [{note: D3, tone: synth2}]}
    - {frame: 60, notes: [{note: D2, tone: synth2}]}
    - {frame: 68, notes: [{note: D3, tone: synth2}]}
    - {frame: 75, notes: [{note: D4, tone: synth2}]}
    - {frame: 83, notes: [{note: D3, tone: synth2}]}
    - {frame: 90, notes: [{note: D2, tone: synth2}]}
    - {frame: 98, notes: [{note: D3, tone: synth2}]}
    - {frame: 105, notes: [{note: D4, tone: synth2}]}
    - {frame: 113, notes: [{note: D3, tone: synth2}]}
    - {frame: 120, notes: []}

  drums_intro:
    - {frame: 0, notes: [{tone: kick}]}
    - {frame: 30, notes: [{tone: snare}]}
    - {frame: 60, notes: [{tone: kick}]}
    - {frame: 90, notes: [{tone: snare}]}
    - {frame: 120, notes: []}

  drums_end:
    - {frame: 0, notes: [{tone: kick}]}
    - {frame: 30, notes: [{tone: snare}]}
    - {frame: 60, notes: [{tone: kick}]}
    - {frame: 90, notes: [{tone: snare}]}
    - {frame: 98, notes: [{tone: snare}]}
    - {frame: 106, notes: [{tone: snare}]}
    - {frame: 112, notes: [{tone: snare}]}
    - {frame: 120, notes: []}

  bass:
    - {frame: 15, notes: [{note: D2, tone: bass}]}
    - {frame: 45, notes: [{note: D2, tone: bass}]}
    - {frame: 75, notes: [{note: D2, tone: bass}]}
    - {frame: 105, notes: [{note: D2, tone: bass}]}

  riser:
    - {frame: 0, notes: [{tone: riser}]}

track:
  - {frame: 0, patterns: [intro]}
  - {frame: 120, patterns: [intro]}
  - {frame: 240, patterns: [intro, drums_intro]}
  - {frame: 360, patterns: [intro, drums_intro]}
  - {frame: 480, patterns: [arp]}
  - {frame: 600, patterns: [arp]}
  - {frame: 720, patterns: [arp, drums_intro]}
  - {frame: 840, patterns: [arp, drums_end, riser]}
  - {frame: 960, patterns: [intro, drums_intro, bass]}
  - {frame: 1080, patterns: [intro, drums_intro, bass]}
  - {frame: 1200, patterns: [intro, drums_intro, bass]}
  - {frame: 1320, patterns: [intro, drums_end, bass]}
  - {frame: 1440, patterns: [arp, drums_intro, bass]}
  - {frame: 1560, patterns: [arp, drums_intro, bass]}
  - {frame: 1680, patterns: [arp, drums_intro, bass]}
  - {frame: 1800, patterns: [arp, drums_end, bass, riser]}
```

## How to parse and play the music

Generate the binary blob.

```sh
cargo install --git https://github.com/Srynetix/wasm4-tracker
wasm4-tracker ./sample.yml ./sample.bin
```

```rust
use wasm4_sx::{W4RefCell, TrackReader, Engine};

static TRACK: W4RefCell<TrackReader> =
    W4RefCell::new(TrackReader::new(include_bytes!("./sample.bin")));

#[no_mangle]
fn update() {
    let frame_count = Engine::frame_count();
    TRACK.borrow_mut().play_tick(frame_count as usize);
}
```

[WASM-4]: https://wasm4.org/
[wasm4-sx]: https://github.com/Srynetix/wasm4-sx
[TrackReader]: https://srynetix.github.io/wasm4-sx/wasm4_sx/struct.TrackReader.html
