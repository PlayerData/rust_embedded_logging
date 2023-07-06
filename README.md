# Rust embedded logging

A crate to provide logging for embedded applications.

This expects `void embedded_logging_log(uint8_t level, const char *msg)` to be defined for the library to call out to.

An example implementation that uses zephyrs logging framework is in example_wiring.c
