# i18nx

i18nx is a runtime localization library for Rust. It is designed to be simple and easy to use.

It supports Rusty Object Notation (RON) files for translation data. Refer to the [RON documentation](https://docs.rs/ron) for more information.

It exports a single macro `t!` that can be used to translate strings at runtime.

For formatting, it uses the same syntax as the `format!` macro. Refer to the [formatx documentation](https://docs.rs/formatx) for more information.

## Usage

```rust
use i18nx::t;

// Create a new translation dictionary
// Tip: use `include_str` macro to embed translation files
i18nx::from_ron!(r#"{
  "Hello {name}!": {
    "de": "Hallo {name}!",
    "fr": "Bonjour {name}!",
  },
}"#);

// If you prefer storing your localizations separately
i18nx::with_ron!("cn", r#"{
  "Hello {name}!": "你好 {name}！",
}"#);
i18nx::with_ron!("ru", r#"{
  "Hello {name}!": "Привет {name}!",
}"#);

// Set locale anytime
i18nx::locale!("fr");

// Use the `t` macro just like you would use `format`
assert_eq!(
    t!("Hello {name}!", name = "Rustaceans"),
    "Bonjour Rustaceans!"
);
assert_eq!(
    t!("No translation for this string, so it will be printed and formatted as-is."),
    "No translation for this string, so it will be printed and formatted as-is."
);
```

See also: [integration example](examples/demo.rs) and [integration test](tests/integration.rs).


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
