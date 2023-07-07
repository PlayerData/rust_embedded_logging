# Rust embedded logging

A crate to provide logging for embedded applications.

This expects `void embedded_logging_log(uint8_t level, const char *msg)` to be defined for the library to call out to.

An example implementation that uses zephyrs logging framework is in `zephyr/wiring.c`

This can be auto added if you include this as a zephyr module
```
manifest:
  
  remotes:
    - name: playerdata
      url-base: https://github.com/PlayerData
  
  projects:
    - name: rust_embedded_logging
      remote: playerdata
      revision: 0.1.0
      path: modules/rust_embedded_logging
```
