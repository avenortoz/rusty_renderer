use crate::tga::parse_error::ParseError;

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
    pub image_origin: ImageOrigin,
    pub alpha_channel: u8,
}

fn has_color_map(value: u8) -> Result<bool, ParseError> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseError::Header),
    }
}

impl TgaHeader {
    pub(crate) fn parse(input: &[u8]) -> Result<TgaHeader, ParseError> {
        let mut offset = 0;
        let id_length: u8 = input[offset];
        offset += 1;
        let id_color_map: bool = has_color_map(input[offset])?;
        offset += 1;
        let (image_format_type, image_data_type) = parse_image_type(input[offset])?;
        offset += 1;
        let color_map_start = u16::from_le_bytes(input[offset..=offset + 1].try_into().unwrap());
        offset += 2;
        let color_map_len = u16::from_le_bytes(input[offset..=offset + 1].try_into().unwrap());
        offset += 2;
        let color_map_depth: Bpp = parse_bpp(input[offset])?;
        offset += 1;
        let x_origin = u16::from_le_bytes(input[offset..=offset + 1].try_into().unwrap());
        offset += 2;
        let y_origin = u16::from_le_bytes(input[offset..=offset + 1].try_into().unwrap());
        offset += 2;
        let width = u16::from_le_bytes(input[offset..=offset + 1].try_into().unwrap());
        offset += 2;
        let height = u16::from_le_bytes(input[offset..=offset + 1].try_into().unwrap());
        offset += 2;
        let bpp = parse_bpp(input[offset])?;
        offset += 1;
        let _descriptor = input[offset];
        let image_origin: ImageOrigin = parse_image_origin(_descriptor);
        let alpha_channel: u8 = _descriptor & 0xF;
        offset += 1;
        Ok(TgaHeader {
            id_length,
            color_map_type: id_color_map,
            image_format_type,
            image_data_type,
            color_map_start,
            color_map_len,
            color_map_depth,
            x_origin,
            y_origin,
            width,
            height,
            bpp,
            image_origin,
            alpha_channel,
        })
    }
}
