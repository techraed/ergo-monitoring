# Ergo - monitoring
Debug service printing out useful for developers and managers information about ergo blockchain state.

## Run
## System requirements
Make sure you have OpenSSL installed on your system. For more info - [read](https://docs.rs/openssl/0.10.30/openssl/#building).
### Binary from ~/.cargo/bin
1. Install [rust toolchain](https://github.com/rust-lang/rustup)
2. `cargo install --git https://github.com/SabaunT/ergo-monitoring.git --tag [version]`
3. `ergo-monitoring path/to/config/file.yml`. You can provide no path - an example config file (`example.yml`) will be used then.
### From source
1. Install [rust toolchain](https://github.com/rust-lang/rustup)
2. Download repo using `git clone -b [version] https://github.com/SabaunT/ergo-monitoring.git && cd ergo_monitoring`
3. `cargo run path/to/config/file.yml`. You can provide no path - an example config file (`example.yml`) will be used then.


## Changelog
### Version 0.1.2 - 10.11.2020
More information about blockchain state is provided. Rust openssl crate now tries to statically link to a copy of
OpenSSL. For more info - [read](https://docs.rs/openssl/0.10.30/openssl/#vendored).  

Example output
```
Peers number monitoring
--------------------------------------------------------------------------------------------------------
|                Peer                | Peers number | Headers height | Full height | Unconfirmed count |
--------------------------------------------------------------------------------------------------------
|   http://88.198.13.202:9053/info   |      38      |     357533     |   357533    |        19         |
|   http://159.65.11.55:9053/info    |      40      |     357533     |   357533    |        19         |
|  http://165.227.26.175:9053/info   |      35      |     357533     |   357533    |        27         |
|   http://159.89.116.15:9053/info   |      34      |     357533     |   357533    |        19         |
--------------------------------------------------------------------------------------------------------

```
### Version 0.1.1 - 09.11.2020
Prints out peers number data which is supplied by public nodes stated in config file.
