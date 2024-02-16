# Ping in your tray

![screenshot](assets/screenshot.png)

# Releases

You can download .dmg file from the latest release (built on CI).

# Built with

1. [Tauri](https://tauri.app/)
2. [ping](https://crates.io/crates/ping)

# Internals

3 threads:

1. UI thread.
2. Thread that consecutively runs `ping` and stores results.
3. Synchronization thread that updates tray menu based on the state of the ping thread.

No cross-thread communication, just a global shared value behind a mutex. 
