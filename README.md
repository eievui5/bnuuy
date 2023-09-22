# bnuuy

[![dependency status](https://deps.rs/repo/github/eievui5/bnuuy/status.svg)](https://deps.rs/repo/github/eievui5/bnuuy)
[![Latest Version](https://img.shields.io/crates/v/bnuuy.svg)](https://crates.io/crates/bnuuy)
[![Docs](https://docs.rs/bnuuy/badge.svg)](https://docs.rs/bnuuy)

bnuuy is a fork of bnuuy, a RabbitMQ client written in pure Rust.

This fork was made because the bnuuy repo seems to have been abandoned for years, despite new PRs and issues being opened.

Alternatives like amqprs lack a nice api and good documentation, despite having more recent maintainence, which is why this project is worth reviving.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bnuuy = "1.0.0"
```

For usage, see the [documentation](https://docs.rs/bnuuy/) and
[examples](https://github.com/eievui5/bnuuy/tree/master/examples).

## Minimum Support Rust Version

The minimum supported Rust version for bnuuy 1.0.0 is currently Rust 1.56.0,
but that may change with a patch release (and could change with a patch release
to a dependency without our knowledge).

## TLS Support

By default, bnuuy enables TLS support via the
[native-tls](https://crates.io/crates/native-tls) crate. You can switch to using [rusttls](https://github.com/rustls/rustls) through the `rusttls-tls` feature. You can disable

support for TLS by turning off default features:

```toml
[dependencies]
bnuuy = { version = "1.0.0", default-features = false }
```

If you disable TLS support, the methods `Connection::open`,
`Connection::open_tuned`, and `Connection::open_tls_stream` will no longer be
available, as all three only allow secure connections. The methods
`Connection::insecure_open`, `Connection::insecure_open_tuned`, and
`Connection::insecure_open_stream` will still be available; these methods
support unencrypted connections.

## Integration Tests

bnuuy contains integration tests that require a RabbitMQ server. To run these,
set the `AMIQUIP_TEST_URL` environment variable to an `amqp://` or `amqps://` URL
before running `cargo test`. For example, if you have a RabbitMQ instance running
with the default guest account on your development machine:

```
bash$ AMIQUIP_TEST_URL=amqp://guest:guest@localhost cargo test
```

If the `AMIQUIP_TEST_URL` environment variable is not set, all integration tests
will be skipped (and silently pass). If you run with `--nocapture`, you will see
a warning printed on the first such skipped test:

```
bash$ cargo test -- nocapture
...
test integration_tests::exchange::test_declare ... AMIQUIP_TEST_URL not defined - skipping integration tests
...
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in bnuuy by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
