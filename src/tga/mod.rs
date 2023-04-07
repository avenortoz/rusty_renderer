mod parse_error;
mod header;
mod tga_image;
pub use crate::{
    tga::parse_error::ParseError,
    tga::header::*,
    tga::tga_image::*,
};
