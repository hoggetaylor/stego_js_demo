#[macro_use]
extern crate stdweb;
extern crate rand;
extern crate matrix_code;

use matrix_code::Stego;
use matrix_code::render::RenderOptions;

fn main() {
    stdweb::initialize();

    js! {
        var random_code = @{random_code};
        setInterval(function() {
            document.getElementById("stego-container").innerHTML = random_code();
        }, 1000);
    }
}

fn random_code() -> String {
    let id = ::rand::random();
    let stego = Stego::new();
    let bytes = stego.render(id, &RenderOptions::default());
    unsafe {
        String::from_utf8_unchecked(bytes)
    }
}
