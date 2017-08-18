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

    let mut options = RenderOptions::default();
    if let Some(location) = document().location() {
        let url = Url::parse(&location.href()).unwrap();
        let mut query_map = HashMap::new();
        for (key, value) in url.query_pairs().into_owned() {
            query_map.insert(key, value);
        }
        
        if let Some(fg) = query_map.remove("fg") {
            options.fg_color = fg;
        }
        if let Some(bg) = query_map.remove("bg") {
            options.bg_color = bg;
        }
    }

    js! {
        var update_div = function() {
            document.getElementById("stego-container").innerHTML = @{
                random_code(&options)
            };
        };
        update_div();
        setInterval(update_div, 1000);
    }
}

fn random_code(options: &RenderOptions) -> String {
    let id = ::rand::random();
    let stego = Stego::new();
    let bytes = stego.render(id, options);
    unsafe {
        String::from_utf8_unchecked(bytes)
    }
}
