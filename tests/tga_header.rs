use rusty_renderer::tga::{Rgb888, TgaImage};
use std::fs::File;

#[test]
fn valid() {
    assert_eq!(1, 1);
}

#[test]
fn valid_set() {
    let mut fd = File::create("/tmp/test_2.tga").unwrap();
    let mut image: TgaImage<Rgb888> = TgaImage::new(1000, 1000).unwrap();
    for i in 0..500{
        image.set(i, i, Rgb888 { r: 255, g: 255, b: 255 });
    }
    image.write(& mut fd).unwrap();
}
