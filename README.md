# Pomodoro App

A minimalist Pomodoro timer built with Rust + Tauri 2.x and SvelteKit.
This is a personal side project—Rust is just for fun, not for career ambitions. Be gentle; I’m not trying to become a
Rust guru overnight!

<img src="imgs/img.png" alt="img" width="300"/>
<img src="imgs/img_1.png" alt="img_1" width="300"/>

## Features

- Simple Pomodoro timer with configurable work and break durations
- Works in simple pomodoro states (working, short and long breaks) but when time is over, instead of stop or go to next
  state, counter still works
- Cross-platform desktop app (macOS, Windows, Linux)
- Rust backend handling timer logic

## Future Plans

- Add statistics module to track Pomodoro sessions over time
- Add support of projects and associated tasks

## Installing & running (for developers)

To start application:

```shell
cd src-tauri
cargo tauri dev
```

### Coverage report:

```shell
cargo tarpaulin -o Html
```

### Clippy:

```shell
cargo clippy -- -D warnings
```
