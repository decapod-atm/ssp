# SSP

This is an implementation of the messages needed for the Smiley Secure Protocol (SSP), and its encrypted variant Encrypted Smiley Secure Protocol (eSSP).

A base set of messages are implemented, and additions are always welcome through pull requests :)

## Usage

By default, `ssp` is a `no-std` compatible library. To use in your project:

```
# Cargo.toml
ssp = "0.3"
```

If you would like to use `std`-only features:

```
# Cargo.toml
ssp = { version = "0.3", features = ["std"] }
```

## CAUTION

While this library has undergone testing against real hardware, it is still in early development.

Please use caution, and report issues if you encounter a problem.
