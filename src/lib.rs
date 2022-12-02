//! # i18nx
//!
//! i18nx is a runtime localization library for Rust. It is designed to be simple and easy to use.
//!
//! It supports Rusty Object Notation (RON) files for translation data. Refer to the [RON documentation](https://docs.rs/ron) for more information.
//!
//! It exports a single macro `t!` that can be used to translate strings at runtime.
//!
//! For formatting, it uses the same syntax as the `format!` macro. Refer to the [formatx documentation](https://docs.rs/formatx) for more information.
//!
//! ## Usage
//!
//! ```rust
//! use i18nx::t;
//!
//! // Create a new translation dictionary
//! // Tip: use `include_str` macro to embed translation files
//! i18nx::from_ron!(r#"{
//!   "Hello {name}!": {
//!     "de": "Hallo {name}!",
//!     "fr": "Bonjour {name}!",
//!   },
//! }"#);
//!
//! // If you prefer storing your localizations separately
//! i18nx::with_ron!("cn", r#"{
//!   "Hello {name}!": "你好 {name}！",
//! }"#);
//! i18nx::with_ron!("ru", r#"{
//!   "Hello {name}!": "Привет {name}!",
//! }"#);
//!
//! // Set locale anytime
//! i18nx::locale!("fr");
//!
//! // Use the `t` macro just like you would use `format`
//! assert_eq!(
//!     t!("Hello {name}!", name = "Rustaceans"),
//!     "Bonjour Rustaceans!"
//! );
//! assert_eq!(
//!     t!("No translation for this string, so it will be printed and formatted as-is."),
//!     "No translation for this string, so it will be printed and formatted as-is."
//! );
//! ```
//!
//! ## Alternatives
//!
//! * [locales](https://crates.io/crates/locales)
//! * [fluent-bundle](https://crates.io/crates/fluent_bundle)
//! * [fluent-syntax](https://crates.io/crates/fluent-syntax)
//! * [rust-i18n](https://crates.io/crates/rust-i18n)
//! * [i18n-rust](https://crates.io/crates/i18n-rust)
//! * [i18n-embed](https://crates.io/crates/i18n-embed)
//! * [cargo-i18n](https://crates.io/crates/cargo-i18n)
//! * [gettext](https://docs.rs/gettext/latest/gettext/)
//! * [gettextrs](https://docs.rs/gettext-rs/latest/gettextrs/)
//!

use std::sync::Mutex;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

/// Dictionary holds current locale and a map of translations for each locale.
///
/// Example:
/// ```rust
/// use i18nx::Dictionary;
///
/// let mut dict = Dictionary::from_ron(r#"{
///   "Hello {name}!": {
///     "de": "Hallo {name}!",
///     "fr": "Bonjour {name}!",
///   },
/// }"#).unwrap();
/// dict.locale = Some("fr");
/// assert_eq!(
///     dict.get("Hello {name}!").unwrap(),
///     "Bonjour {name}!"
/// );
/// ```
#[derive(Default, Debug)]
pub struct Dictionary {
    /// Locale is a string that holds the current language.
    pub locale: Option<&'static str>,
    /// The resource is a HashMap of translations, where the key is the message and the value is a HashMap of translations for each locale.
    pub resource: HashMap<&'static str, HashMap<&'static str, &'static str>>,
}

impl Dictionary {
    /// Constructs empty dictionary.
    pub fn new() -> Self {
        Dictionary::default()
    }

    /// Constructs dictionary from RON string.
    pub fn from_ron(ron: &'static str) -> Result<Dictionary, ron::Error> {
        let dict = ron::from_str(ron)?;
        Ok(Dictionary {
            locale: None,
            resource: dict,
        })
    }

    /// Adds translations from RON string to the dictionary.
    pub fn with_ron(&mut self, locale: &'static str, ron: &'static str) -> Result<&mut Self, ron::Error> {
        let dict: HashMap<&'static str, &'static str> = ron::from_str(ron)?;
        for (key, translation) in dict.iter() {
            self.resource.entry(key).or_default().insert(locale, *translation);
        }
        Ok(self)
    }

    /// Lookup a translation for the given key and locale.
    pub fn get(&self, key: &'static str) -> Option<&'static str> {
        self.resource.get(key).and_then(move |translations| {
            translations.get(self.locale.unwrap())
        }).copied()
    }
}

#[doc(hidden)]
pub fn global_dictionary() -> &'static Mutex<Dictionary> {
    static INSTANCE: OnceCell<Mutex<Dictionary>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(Dictionary::new())
    })
}

/// Same as [Dictionary::new](struct.Dictionary.html#method.new) but uses global dictionary.
#[macro_export]
macro_rules! new {
    () => {{
        let mut dict = $crate::global_dictionary().lock().unwrap();
        let locale = dict.locale;
        *dict = $crate::Dictionary::new();
        dict.locale = locale;
    }}
}

/// Same as [Dictionary::from_ron](struct.Dictionary.html#method.from_ron) but uses global dictionary.
#[macro_export]
macro_rules! from_ron {
    ($ron:expr) => {{
        let mut dict = $crate::global_dictionary().lock().unwrap();
        let locale = dict.locale;
        *dict = $crate::Dictionary::from_ron($ron).unwrap();
        dict.locale = locale;
    }}
}

/// Same as [Dictionary::with_ron](struct.Dictionary.html#method.with_ron) but uses global dictionary.
#[macro_export]
macro_rules! with_ron {
    ($locale:expr, $ron:expr) => {{
        $crate::global_dictionary().lock().unwrap().with_ron($locale, $ron).unwrap();
    }}
}

/// Same as [Dictionary::locale](struct.Dictionary.html#method.locale) but uses global dictionary.
#[macro_export]
macro_rules! locale {
    () => {{
        $crate::global_dictionary().lock().unwrap().locale = None;
    }};

    ($locale:expr) => {{
        $crate::global_dictionary().lock().unwrap().locale = Some($locale);
    }};
}

/// Same as [Dictionary::get](struct.Dictionary.html#method.get) but uses global dictionary.
#[macro_export]
macro_rules! t {
    ($template:literal) => {{
        let dictionary = $crate::global_dictionary().lock().unwrap();
        dictionary.locale.and_then(|locale| {
            dictionary.get($template)
        }).unwrap_or($template)
    }};

    ($template:expr, $($values:tt)*) => {{
        let translated = $crate::t!($template);
        formatx::formatx!(translated, $($values)*).unwrap()
    }};
}
