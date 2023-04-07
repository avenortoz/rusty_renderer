use crate::tga::{ParseError, TgaHeader};
use std::error::Error;
use std::marker::PhantomData;

struct RawTgaImage<'a> {
    header: TgaHeader,
    data: &'a Vec<u8>,
}

#[derive(Debug)]
pub struct Rgb888{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Debug)]
pub struct Rgb555{
    pub rgb: u16
}

#[derive(Debug)]
pub struct Gray8{
    g: u8
}

#[derive(Debug)]
pub enum Color {
    Rgb888(Rgb888),
    Gray8(Gray8),
    Rgb555(Rgb555),
}

pub struct TgaImage<'a, C = Rgb888 > {
    //raw_tga: RawTgaImage<'a>,
    raw_data: Vec<u8>,
    target_color_type: &'a PhantomData<C>,
}

impl<'a, Rgb888> TgaImage<'a, Rgb888> {
    pub fn set_rgb888(&self, x: u16, y: u16, pixel: Rgb888) -> Result<(), ParseError> {
        println!("Rgb888");
        Ok(())
    }
}

impl<'a, Rgb555> TgaImage<'a, Rgb555> {
    pub fn set_rgb555(&self, x: u16, y: u16, pixel: Rgb555) -> Result<(), ParseError> {
        println!("Rgb555 {:?}", pixel.rgb);
        Ok(())
    }
}

impl<'a, Gray8> TgaImage<'a, Gray8> {
    pub fn set_gray8(&self, x: u16, y: u16, pixel: Gray8) -> Result<(), ParseError> {
        println!("Gray8");
        Ok(())
    }
}

impl<'a> TgaImage<'a> {
    pub fn new() -> Self {
        //todo!()
        return TgaImage { raw_data: vec![23,23,23,23], target_color_type: &PhantomData::<Rgb888>}
    }
}
