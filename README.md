# Kaizen

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="resources/samurai-dark.png">
  <img src="resources/samurai.png" alt="Samurai silhouette" width="180" align="right">
</picture>

**One small thing, every day.**

Kaizen is a small, local habit tracker. Pick the activities you want to get better at, run a timer while you work on them, and watch your progress build up on a heatmap and a weekly streak.

It runs on your own machine as a single binary with a browser UI. No account, no cloud.

## Features

- **Activities** - track anything: learning, programming, reading, exercise
- **Timer** - start, pause, resume and stop; the time is logged to today when you stop
- **Week streak** - a week counts when you log at least once between Monday and Sunday. The current week only breaks the streak once it's over, so you always have until Sunday night.
- **Heatmap** - a year of activity at a glance, one square per day
- **Statistics** - time logged today, this week, and your best day

## Installation

Download the archive for your platform from the [latest release](https://github.com/Soskar1/kaizen/releases/latest):

## Usage

Start Kaizen from the folder where you want your data to live:

```sh
cd ~/kaizen-data
kaizen
```

Then open **http://127.0.0.1:3000** in your browser.

The server only listens on `127.0.0.1`, so it is reachable from your machine only. Stop it with `Ctrl+C`.

### Where your data is stored

Kaizen stores data in the **folder you start it from**. Every subfolder is an activity, and every day you log time gets one JSON file:

```
kaizen-data/
├── Learning/
│   ├── 2026-10-14.json
│   └── 2026-10-15.json
└── Reading/
    └── 2026-10-15.json
```

```json
{"date":"2026-10-15","activity_duration_in_seconds":1500}
```

## Building from source

You need:

- [Rust](https://rustup.rs) (stable) with the WebAssembly target:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev):
  ```sh
  cargo install trunk --locked
  ```

The web app is embedded into the `kaizen` binary at compile time, so build it first:

```sh
cd crates/kaizen-web
trunk build --release
cd ../..
cargo build --release -p kaizen
```

The binary ends up in `target/release/kaizen`. If you skip the Trunk step the build still succeeds, but the binary has no UI and warns about it on startup.

## Development

Run the server and the web app's dev server side by side:

```sh
# Terminal 1: API on http://127.0.0.1:3000
cargo run -p kaizen

# Terminal 2: web app with live reload on http://127.0.0.1:8080
cd crates/kaizen-web
trunk serve
```

Open http://127.0.0.1:8080. Trunk forwards API requests to the server (see [`Trunk.toml`](crates/kaizen-web/Trunk.toml)).

In debug builds, the server reads the web app from `crates/kaizen-web/dist` on disk instead of embedding it, so `cargo run -p kaizen` also serves whatever you last built with `trunk build`.