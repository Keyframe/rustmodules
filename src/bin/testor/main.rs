#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use testor::mymodule::{ testor_one, testor_two, bla };

fn main() {
    // mymodule::testor_one();
    // mymodule::testor_two();
    testor_one();
    testor_two();
    bla();
}
