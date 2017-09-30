Rustworking Core
===========
Rustworking is a project that takes a few networking operations often used by
system administrators and implements them in Rust. This will allow these
operations to be performed in bulk which can allow for quick and easy testing
of multiple hosts in a network.

The Rustworking Core is the Rust crate that contains the implementation of
these tests.

You can find the CLI for this tool [here](https://github.com/timmonfette1/rustworking-cli)

Supported Functions
-------------
The following are the current operations this crate supports:

  - PING
  - Test an HTTP request (limited to single IP address right now)

In the future, you'll be able to do the following operations:

  - Test a TCP packet
  - Test a UDP packet

You can perform these operations on the following:

  - A single IP Address (v4 or v6)
  - A subnet of IP Addresses (v4 or v6)
  - A file of IP Addresses (1 per line, combination of v4 and v6)

How to Use
------------
Simply add this crate to your project to make use of these functions:

In your Cargo.toml

```
[dependencies]
rustworking-core = { path = "rustworking-core" }
```
