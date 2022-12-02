const EXAMPLE: &'static str = r#"{
  "Hello": {
    "de": "Hallo",
    "fr": "Bonjour",
  },
  "Hello {name}!": {
    "de": "Hallo {name}!",
    "fr": "Bonjour {name}!",
  },
}
"#;

const EXAMPLE_RU: &'static str = r#"{
  "Hello": "Привет",
  "Hello {name}!": "Привет {name}!",
}
"#;

const EXAMPLE_CN: &'static str = r#"{
  "Hello": "你好",
  "Hello {name}!": "你好 {name}!",
}
"#;


#[cfg(test)]
mod test {
    use i18nx;
    use i18nx::{t, Dictionary};
    use crate::{EXAMPLE, EXAMPLE_RU, EXAMPLE_CN};

    #[test]
    fn global() {
        fn missing_dict() {
            assert_eq!(t!("Hello"), "Hello");
        }

        fn unset_locale() {
            i18nx::from_ron!(EXAMPLE);
            i18nx::locale!();
            assert_eq!(t!("Hello"), "Hello");
        }

        fn missing_translation() {
            i18nx::from_ron!(EXAMPLE);
            i18nx::locale!("cn");
            assert_eq!(t!("Hello"), "Hello");
        }

        fn from_ron() {
            i18nx::from_ron!(EXAMPLE);
            i18nx::locale!("de");
            assert_eq!(t!("Hello"), "Hallo");
            assert_eq!(t!("Hello"), "Hallo");
        }

        fn with_ron() {
            i18nx::with_ron!("ru", EXAMPLE_RU);
            i18nx::locale!("ru");
            assert_eq!(t!("Hello"), "Привет");
        }

        fn with_ron_multi() {
            i18nx::with_ron!("ru", EXAMPLE_RU);
            i18nx::with_ron!("cn", EXAMPLE_CN);
            i18nx::locale!("ru");
            assert_eq!(t!("Hello"), "Привет");
        }

        fn fuzz() {
            i18nx::locale!("ru");
            i18nx::from_ron!(EXAMPLE);

            i18nx::new!();
            i18nx::with_ron!("ru", EXAMPLE_RU);

            assert_eq!(t!("Hello {name}!", name="Cargo"), "Привет Cargo!");
        }

        missing_dict();
        unset_locale();
        missing_translation();
        from_ron();
        with_ron();
        with_ron_multi();
        fuzz();
    }

    #[test]
    fn dictionary_api() {
        let mut dict = Dictionary::from_ron(EXAMPLE).unwrap();
        dict.with_ron("ru", EXAMPLE_RU).unwrap()
            .with_ron("cn", EXAMPLE_CN).unwrap();

        dict.locale = Some("fr");
        assert_eq!(dict.get("Hello").unwrap(), "Bonjour");
    }
}
