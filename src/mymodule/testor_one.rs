#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

pub fn testor_one() {
    println!("Hello from println One!");
    private_func();
}

fn private_func() {
    println!("Hello from private function!");
}
