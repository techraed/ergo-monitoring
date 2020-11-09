# Ergo - monitoring
Debug service printing out useful for developers and managers information about ergo blockchain state.

## Run
## System requirements
Make sure you have OpenSSL installed on your system. For more info, [read](https://docs.rs/openssl/0.10.30/openssl/#building).
### Binary from ~/.cargo/bin
1. Install [rust toolchain](https://github.com/rust-lang/rustup)
2. `cargo install --git https://github.com/SabaunT/ergo-monitoring.git --tag [latest_version]`
3. `ergo-monitoring path/to/config/file.yml`. You can provide no path - an example config file (`example.yml`) will be used then.
### From source
1. Install [rust toolchain](https://github.com/rust-lang/rustup)
2. Download repo using `git clone -b [latest_version] https://github.com/SabaunT/ergo-monitoring.git && cd ergo_monitoring`
3. `cargo run path/to/config/file.yml`. You can provide no path - an example config file (`example.yml`) will be used then.


## Changelog
### Version 0.1.1 - 09.11.2020
Prints out peers number data which is supplied by public nodes stated in config file.
