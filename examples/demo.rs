use i18nx::t;

fn main() {
    // Demonstrating formatting
    println!("{}", t!("I'd rather be {1} than {0}", "right", "happy"));

    // Set language at runtime
    i18nx::locale!("ru");

    // Import a new translation dictionary
    i18nx::from_ron!(include_str!("assets/localization/demo.ron"));

    // Use with_ron if you prefer storing your localizations separately
    i18nx::with_ron!("cn", include_str!("assets/localization/demo.cn.ron"));
    i18nx::with_ron!("ru", include_str!("assets/localization/demo.ru.ron"));
    dbg!(&i18nx::global_dictionary().lock().unwrap().resource);

    // Use the `t` macro just like you would use `format`
    println!("{}", t!("Hello {name}!", name = "Rustaceans"));

    // Reset global dictionary and/or locale if you want
    i18nx::new!();
    i18nx::locale!();
    dbg!(&i18nx::global_dictionary().lock().unwrap().resource);
}

// $ cargo run --example demo -q
// I'd rather be happy than right
// Привет Rustaceans!
// [examples/demo.rs:22] &i18nx::global_dictionary().lock().unwrap().resource = {
//     "Hello {name}!": {
//         "ru": "Привет {name}!",
//         "fr": "Bonjour {name}!",
//         "cn": "你好 {name}！",
//         "de": "Hallo {name}!",
//     },
// }
// [examples/demo.rs:25] &i18nx::global_dictionary().lock().unwrap().resource = {}
