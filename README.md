# bitcoin-handshake

[![Rust](https://github.com/luke-lorenzini/bitcoin-handshake/actions/workflows/rust.yml/badge.svg)](https://github.com/luke-lorenzini/bitcoin-handshake/actions/workflows/rust.yml)

This library implements a simple Bitcoin handshake based on the protocol specified [here](<https://en.bitcoin.it/wiki/Protocol_documentation>).

The repo contains several sample binaries along with the library. The binaries demonstrate execution of the handshake for both the local node and mimicking a simplified remote node. There is also a simple demonstration of ping-pong.

## Table of Contents

- [bitcoin-handshake](#bitcoin-handshake)
  - [Table of Contents](#table-of-contents)
  - [Quick Start](#quick-start)
  - [Structure](#structure)
  - [Binaries Explained](#binaries-explained)
    - [simple-handshake](#simple-handshake)
    - [node](#node)
    - [ping-pong](#ping-pong)
  - [Library Explained](#library-explained)
  - [Tests](#tests)

## Quick Start

From a terminal, start the "remote" node to receive the handshake:

```bash
cargo run --bin node
```

From a separate terminal, start the "local" node to initiate the handshake:

```bash
cargo run 127.0.0.1 8333
```

You should see acknowledgements printed to both terminals indicating status.

```bash
...
Received 'verack'
Handshake successful!
```

## Structure

The program is structured in the following hierarcy. Binaries are located in the *bin* folder. *message.rs* is the main building block for messages consisting of headers, *header.rs* and payloads, *payload.rs*.

```tree
|_src
|   |_bin
|   |   |_node.rs
|   |   |_ping-pong.rs
|   |   |_simple-handshake.rs
|   |_payload
|   |   |_ping.rs
|   |   |_pong.rs
|   |   |_verack.rs
|   |   |_version.rs
|   |_header.rs
|   |_lib.rs
|   |_magic_bytes.rs
|   |_message.rs
|   |_payload.rs
|   |_tests
|   |   |_test.rs
```

## Binaries Explained

### simple-handshake

The binary *simple-handshake* executes the handshake with fixed, internal parameters. It can be run with either the hard-coded IP and ports (`169.120.69.82:8333`) by running:

```bash
cargo run
```

or by running with your own custom IP and port:

```bash
cargo run 127.0.0.1 8333
```

If the binary is unable to parse the input parameters, it will default to the ones mentioned above.

### node

*node* simulates how a remote node "in the wild" would respond to an incoming handshake. It can be run locally to test the above *simple-handshake*. *node* has been compiled with parameters for IP address `127.0.0.1`, and port `8333`. *node* runs an infinite loop waiting for incoming requests. If it receives a `version` command, it will respond accordingly with its `version` followed by a `verack`

To run *node*, open a new terminal and:

```bash
cargo run --bin node
```

### ping-pong

*ping-pong* builds on *simple-handshake*. Once the handshake has been completed, a loop will begin. The loop does two things:

1) reply appropriately to an incoming 'ping' command
2) forward an 'inventory' message to the main thread for processing

*ping-pong* can be run similarly to *simple-handshake* in that it can either be run with default parameters:

```bash
cargo run --bin ping-pong
```

or with custom parameters

```bash
cargo run --bin ping-pong 127.0.0.1 8333
```

## Library Explained

The main entry to use the program is via the lib. The lib exposes the underlying command hierarchy. There is a parent `Message` type, which is built up from two sub-types; `Header`, and `Payload`.

```rust
let payload = PayloadVersion::new(protocol_version, remote_ip, remote_port);
let header = Header::new(
        MagicBytes::Mainnet,
        payload.get_command_string(),
        mem::size_of::<PayloadVersion>().try_into().expect("No surprises here"),
        calculate_checksum(&payload.get_payload()),
    );

let message = Message::new(header, Box::new(payload));
```

Currently, the only commands which have been implemented are, `version`, `ping`, `pong`, and `verack`; each of which is contained in its own *.rs file.

This lib has been designed to be easily extensible via an exposed trait, `PayloadTrait`. You can simply implement the trait for your own commands, build a message and then use in your own program. See docs for complete example. The lib uses trait objects `Box<dyn PayloadTrait>` to dynamically add new and different commands as needed.

## Tests

Several tests have been created which can be run by:

```bash
cargo test
```
