#[macro_use]
extern crate stdweb;
extern crate rand;
extern crate matrix_code;
extern crate url;

use matrix_code::Stego;
use matrix_code::render::RenderOptions;
use stdweb::web::document;
use url::Url;
use std::collections::HashMap;

fn main() {
    stdweb::initialize();

    let mut fg = None;
    let mut bg = None;
    if let Some(location) = document().location() {
        let url = Url::parse(&location.href()).unwrap();
        let mut query_map = HashMap::new();
        for (key, value) in url.query_pairs().into_owned() {
            query_map.insert(key, value);
        }
        fg = query_map.remove("fg");
        bg = query_map.remove("bg");
    }

    js! {
        var random_code = @{random_code};

        var update_div = function() {
            document.getElementById("stego-container").innerHTML = random_code(@{fg}, @{bg});
        };
        update_div();
        setInterval(update_div, 1000);
    }
}

fn random_code(fg: Option<String>, bg: Option<String>) -> String {
    let id = ::rand::random();
    let stego = Stego::new();
    let mut options = RenderOptions::default();
    if let Some(fg) = fg {
        options.fg_color = fg;
    }
    if let Some(bg) = bg {
        options.bg_color = bg;
    }
    let bytes = stego.render(id, &options);
    unsafe {
        String::from_utf8_unchecked(bytes)
    }
}
