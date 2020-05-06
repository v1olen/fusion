# Fusion
Fusion is a simple wlan management tool with unix-like commands syntax. The main assumption of Fusion it to resemble in usage unix-like commands like `ls` and `cat` to provide an intuitive tool with a low entry threshold.

Currently Fusion is available only for Linux but a version for Windows is planned.

# Requirements

For this moment the only dependency is `networkmanager` as on Linux Fusion works as a wrapper for `nmcli`.

# Building

## Requirements

 * `libffi`
 * `cargo`

## Command

### Linux
```
cargo build --release
```

Run inside main or child directory. Output binary should be available inside `/target/release` directory.

## Repositories
On Arch Linux you can install with `pacaur`:

```
pacaur -S fusion
```

Automated install for other distributions is planned.

# Documentation

Soon.
