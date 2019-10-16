#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

pub fn bla() {
    println!("Hello from Bla 2!");
    bla_private();
}

fn bla_private() {
    println!("Hello from private Bla 2!");
}
