extern crate stdweb;
extern crate rand;
extern crate matrix_code;

use stdweb::web::{document, INode};
use std::time::Duration;
use matrix_code::Stego;
use matrix_code::render::RenderOptions;

fn main() {
    stdweb::initialize();

    let div = document().query_selector("stego-container").unwrap();
    loop {
        div.set_text_content(&random_code());
        std::thread::sleep(Duration::from_secs(1));
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
