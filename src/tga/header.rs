use nom::{
    combinator::{map, map_opt, map_res},
    number::complete::{le_u16, le_u8},
    IResult,
};
use crate::tga::parse_error::ParseError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Bpp {
    Bits8,
    Bits16,
    Bits24,
    Bits32,
}

impl Bpp{
    pub fn bits(&self) -> u8 {
        match self {
            Self::Bits8 => 8,
            Self::Bits16 => 16,
            Self::Bits24 => 24,
            Self::Bits32 => 32,
        }
    }
    pub fn bytes(&self) -> u8 {
        self.bits() / 8
    }
}


fn parse_bpp(value: u8) -> Result<Bpp, ParseError>{
    match value {
        0x8 => Ok(Bpp::Bits8),
        0x10 => Ok(Bpp::Bits16),
        0x18 => Ok(Bpp::Bits24),
        0x20 => Ok(Bpp::Bits32),
        _ => Err(ParseError::MismatchedBpp(value))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ImageFormatType {
    Uncompressed,
    RLE,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ImageDataType {
    NoData,
    ColorMapped,
    TrueColor,
    BlackAndWhite,
}

fn parse_image_type(image_type: u8) -> Result<(ImageFormatType, ImageDataType), ParseError> {
    match image_type {
        0 => Ok((ImageFormatType::Uncompressed, ImageDataType::NoData)),
        1 => Ok((ImageFormatType::Uncompressed, ImageDataType::ColorMapped)),
        2 => Ok((ImageFormatType::Uncompressed, ImageDataType::TrueColor)),
        3 => Ok((ImageFormatType::Uncompressed, ImageDataType::BlackAndWhite)),
        9 => Ok((ImageFormatType::RLE, ImageDataType::ColorMapped)),
        10 => Ok((ImageFormatType::RLE, ImageDataType::TrueColor)),
        11 => Ok((ImageFormatType::RLE, ImageDataType::BlackAndWhite)),
        _ => Err(ParseError::ColorMap)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ImageOrigin {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
}

fn parse_image_origin(value: u8) -> ImageOrigin {
    // Take 5 and 4 bit
    match (value & 0x30 ) >> 4 {
        0 => ImageOrigin::BottomLeft,
        1 => ImageOrigin::BottomRight,
        2 => ImageOrigin::TopLeft,
        _ => ImageOrigin::TopRight,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TgaHeader {
    pub id_length: u8,
    pub color_map_type: bool,
    pub image_format_type: ImageFormatType,
    pub image_data_type: ImageDataType,
    pub color_map_start: u16,
    pub color_map_len: u16,
    pub color_map_depth: Bpp,
    pub x_origin: u16,
    pub y_origin: u16,
    pub width: u16,
    pub height: u16,
    pub bpp: Bpp,
    pub image_origin: u8,
    pub alpha_channel: u8,
}

fn has_color_map(value: u8) -> Result<bool, ParseError>{
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseError::Header)
    }
}

impl TgaHeader{
    pub(crate) fn parse(input: &[u8]) -> Result<TgaHeader, ParseError>{
        let id_length: u8 = input[0];
        todo!()
    }
}
