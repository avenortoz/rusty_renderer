#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ParseError{
    ColorMap,
    Header,
    Footer,
    UnsupportedImageType(u8),
    UnsupportedBpp(u8),
    MismatchedBpp(u8),
    // FIX
    UnsupportedTgaType
}
