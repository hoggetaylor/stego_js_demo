#![recursion_limit="512"]
#[macro_use]
extern crate stdweb;
extern crate rand;
extern crate matrix_code;
extern crate maud;

use matrix_code::Stego;
use matrix_code::render::RenderOptions;
use matrix_code::render::spiral::spiral_path::Path;
use matrix_code::encode::Encoder;
use matrix_code::encode::encoding::Encoding;
use maud::Render;

fn main() {
    stdweb::initialize();

    js! {
        var random_code = @{random_code};
        var random_path = @{random_path};
        document.getElementById("stego-container").innerHTML = random_code();

        var toSvgString = function(shapes) {
            return shapes.map(function(shape){
                shape.forEach(function (point, idx) {
                    if (!idx) {
                        point.splice(2, 0, "C");
                        point.unshift("M");
                    } else {
                        point.splice(0, 2, "C");
                    }
                });
                return shape.map(function (point) {
                    return point.join(" ");
                }).join("");
            }).join("")
        };

        setInterval(function() {
            pasition.animate({
                from: document.getElementById("stegopath").attr("d"),
                to: random_path(),
                time: 500,
                easing: function() { },
                begin: function(shapes) { },
                progress: function(shapes, percent){
                    document.getElementById("stegopath").setAttribute("d", toSvgString(shapes));
                },
                end: function(shapes){ }
            });
        }, 1000);
    }
}

fn random_path() -> String {
    let id = ::rand::random();
    let bits = Encoder::new().encode(id);
    let path = Path::for_bits_at(bits.into_raw(), 100, 80);
    let mut path_str = String::new();
    path.render_to(&mut path_str);
    path_str
}

fn random_code() -> String {
    let id = ::rand::random();
    let stego = Stego::new();
    let bytes = stego.render(id, &RenderOptions::default());
    unsafe {
        String::from_utf8_unchecked(bytes)
    }
}
