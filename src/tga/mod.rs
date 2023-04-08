mod parse_error;
mod header;
mod image;
pub use crate::{
    tga::parse_error::ParseError,
    tga::header::*,
    tga::image::*,
};
