# Ping in your tray

![screenshot](assets/screenshot.png)

# Releases

You can download .dmg file [from the latest release](https://github.com/iliabylich/tray-ping/releases) (built on CI). If you need a build for a platform other than Mac feel free to create an issue. The code is cross-platform, so it should be trivial to get a build for linux/windows.

# Code signing

Of course the dmg file is not signed and Mac OS can put the app in the quarantine.

```sh
$ xattr /Applications/tray-ping.app/Contents/MacOS/tray-ping
com.apple.quarantine
# Then
$ xattr -cr /Applications/tray-ping.app/Contents/MacOS/tray-ping
# and try running again
```

# Built with

1. [Tauri](https://tauri.app/)
2. [ping](https://crates.io/crates/ping)

# Internals

3 threads:

1. UI thread (main).
2. Thread that consecutively runs `ping` and stores results.
3. Thread that receives ping lines for the tray via mspc and renders them in the tray menu.

# Configuration

There's no dynamic configuration, however [there are a few top-level constants](/src-tauri/src/main.rs) that can be changed:

1. `DEFAULT_HOST` - set to `google.com:443` by default, has be to be in `<host>:<port>` format.
2. `TRAY_HEIGHT` - set to `15`, configures total number of rows that show sliding window of the `ping` output, i.e. by default it shows last 15 lines.

# License

MIT, do whatever you want.
