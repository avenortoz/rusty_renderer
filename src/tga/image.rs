use crate::tga::{ParseError, TgaHeader};
use std::io::Write;
use std::marker::PhantomData;

pub struct Rgb888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
pub struct Rgb555 {
    pub rgb: u16,
}

pub struct Gray8 {
    pub gray: u8,
}

pub enum Color {
    Rgb888(Rgb888),
    Rgb555(Rgb555),
    Gray8(Gray8),
}

pub trait Pixel {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn bytes() -> u8;
    fn bits() -> u8;
}

impl Pixel for Rgb888 {
    fn from_bytes(bytes: &[u8]) -> Rgb888 {
        Rgb888 {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.r, self.g, self.b]
    }

    fn bytes() -> u8 {
        3
    }

    fn bits() -> u8 {
        24
    }
}

impl Pixel for Rgb555 {
    fn from_bytes(bytes: &[u8]) -> Rgb555 {
        Rgb555 {
            rgb: u16::from_le_bytes([bytes[0], bytes[1]]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.rgb.to_le_bytes().to_vec()
    }

    fn bytes() -> u8 {
        2
    }

    fn bits() -> u8 {
        15
    }
}

impl Pixel for Gray8 {
    fn from_bytes(bytes: &[u8]) -> Gray8 {
        Gray8 { gray: bytes[0] }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.gray]
    }

    fn bytes() -> u8 {
        1
    }

    fn bits() -> u8 {
        8
    }
}

pub struct TgaImage<P>
where
    P: Pixel,
{
    header: TgaHeader,
    data: Vec<u8>,
    color_marker: PhantomData<P>,
}

impl<P> TgaImage<P>
where
    P: Pixel,
{
    pub fn new(width: u16, height: u16) -> Result<Self, ParseError> {
        let mut header: TgaHeader = TgaHeader::new(width, height);
        //header.set_bpp(P::bits())?;
        let size = (width as usize) * (height as usize) * (P::bytes() as usize);
        let mut data: Vec<u8> = Vec::with_capacity(size.into());
        data.resize(size.into(), 0);
        Ok(Self {
            header,
            data,
            color_marker: PhantomData::<P>,
        })
    }

    pub fn set(&mut self, x: u16, y: u16, pixel: &P) {
        let offset = ((y as usize * self.header.width as usize) + x as usize) * P::bytes() as usize;
        let (_, suffix) = self.data.split_at_mut(offset.into());
        let index : usize = P::bytes() as usize;
        suffix[..index].copy_from_slice(&pixel.to_bytes());
    }

    pub fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        self.header.write(writer)?;
        writer.write(&self.data)?;
        Ok(())
    }
}
