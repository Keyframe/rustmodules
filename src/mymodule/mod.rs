#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

mod testor_one;
mod testor_two;

pub use crate::mymodule::testor_one::*;
pub use crate::mymodule::testor_two::*;

#[cfg(feature = "macos")]
mod mysubmodule;
#[cfg(feature = "macos")]
pub use crate::mymodule::mysubmodule::bla::*;

#[cfg(feature = "linux")]
mod mysubmodule2;
#[cfg(feature = "linux")]
pub use crate::mymodule::mysubmodule2::bla::*;
