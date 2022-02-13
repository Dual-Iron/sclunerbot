macro_rules! add_file {
    ($name:ident, $fn:ident, $filename:literal) => {
        lazy_static::lazy_static! {
            pub static ref $name: Vec<&'static str> = {
                include_str!(concat!("../lang/", $filename)).lines().collect()
            };
        }

        pub fn $fn() -> &'static str {
            use rand::prelude::SliceRandom;

            $name.choose(&mut rand::thread_rng()).unwrap()
        }
    };
}

add_file!(QUIPS, quip, "quips.txt");
add_file!(MSG_LACKS_TEXT, lacks_text, "message_lacks_text.txt");
