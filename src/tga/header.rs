use crate::tga::parse_error::ParseError;
use std::io::{Write};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Bpp {
    Bits8,
    Bits16,
    Bits24,
    Bits32,
}

impl Bpp {
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

fn parse_bpp(value: u8) -> Result<Bpp, ParseError> {
    match value {
        0x8 => Ok(Bpp::Bits8),
        0x10 => Ok(Bpp::Bits16),
        0x18 => Ok(Bpp::Bits24),
        0x20 => Ok(Bpp::Bits32),
        _ => Err(ParseError::MismatchedBpp(value)),
    }
}
fn from_bpp(bpp: Bpp) -> u8 {
    match bpp {
        Bpp::Bits8 => 0x8,
        Bpp::Bits16 => 0x10,
        Bpp::Bits24 => 0x18,
        Bpp::Bits32 => 0x20,
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
        _ => Err(ParseError::ColorMap),
    }
}
fn from_image_type(image_format_type: ImageFormatType, image_data_type: ImageDataType) -> u8 {
    match (image_format_type, image_data_type) {
        (ImageFormatType::Uncompressed, ImageDataType::NoData) => 0,
        (ImageFormatType::Uncompressed, ImageDataType::ColorMapped) => 1,
        (ImageFormatType::Uncompressed, ImageDataType::TrueColor) => 2,
        (ImageFormatType::Uncompressed, ImageDataType::BlackAndWhite) => 3,
        (ImageFormatType::RLE, ImageDataType::ColorMapped) => 9,
        (ImageFormatType::RLE, ImageDataType::TrueColor) => 10,
        (ImageFormatType::RLE, ImageDataType::BlackAndWhite) => 11,
        _ => 0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ImageOrigin {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

fn parse_image_origin(value: u8) -> ImageOrigin {
    // Take 5 and 4 bit
    match (value & 0x30) >> 4 {
        0 => ImageOrigin::BottomLeft,
        1 => ImageOrigin::BottomRight,
        2 => ImageOrigin::TopLeft,
        _ => ImageOrigin::TopRight,
    }
}

fn from_image_descriptor(image_origin: ImageOrigin, alpha_channel: bool ) -> u8 {
    let mut image_descriptor : u8 = match image_origin {
        ImageOrigin::BottomLeft => 0b0000_0000,
        ImageOrigin::BottomRight =>  1 << 4,
        ImageOrigin::TopLeft => 2 << 4,
        ImageOrigin::TopRight => 3 << 4, 
    };
    if alpha_channel {
        image_descriptor = image_descriptor | 1 << 7;
    }
    image_descriptor
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TgaHeader {
    pub id_length: u8,
    pub color_map_type: bool,
    pub image_format_type: ImageFormatType,
    pub image_data_type: ImageDataType,
    pub color_map_start: u16,
    pub color_map_len: u16,
    pub color_map_depth: Option<Bpp>,
    pub x_origin: u16,
    pub y_origin: u16,
    pub width: u16,
    pub height: u16,
    pub bpp: Bpp,
    pub image_origin: ImageOrigin,
    pub alpha_channel: bool,
}

impl TgaHeader {
    pub fn new(width: u16, height: u16) -> TgaHeader {
        TgaHeader {
            id_length: 0,
            color_map_type: false,
            image_format_type: ImageFormatType::Uncompressed,
            image_data_type: ImageDataType::TrueColor,
            color_map_start: 0,
            color_map_len: 0,
            color_map_depth: Option::None,
            x_origin: 0,
            y_origin: 0,
            width,
            height,
            bpp: Bpp::Bits24,
            image_origin: ImageOrigin::TopLeft,
            alpha_channel: false,
        }
    } 
    pub fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write(&[self.id_length])?;
        writer.write(&[self.color_map_type as u8])?;
        writer.write(&[from_image_type(self.image_format_type, self.image_data_type)])?;
        writer.write(self.color_map_start.to_le_bytes().as_ref())?;
        writer.write(self.color_map_len.to_le_bytes().as_ref())?;
        match self.color_map_depth {
            Some(bpp) => writer.write(&[from_bpp(bpp)])?,
            None => writer.write(&[0])?,
        };
        writer.write(self.x_origin.to_le_bytes().as_ref())?;
        writer.write(self.y_origin.to_le_bytes().as_ref())?;
        writer.write(self.width.to_le_bytes().as_ref())?;
        writer.write(self.height.to_le_bytes().as_ref())?;
        writer.write(&[from_bpp(self.bpp)])?;
        writer.write(&[from_image_descriptor(self.image_origin, self.alpha_channel)])?;
        Ok(())
    }

    pub fn set_bpp(&mut self, value: u8) -> Result<&Self, ParseError>{
        self.bpp = parse_bpp(value)?;
        Ok(self)
    }
}

fn has_color_map(value: u8) -> Result<bool, ParseError> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseError::Header),
    }
}
