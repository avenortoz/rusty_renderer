use crate::core::Point2D;
use crate::tga::{TgaImage, Pixel};
use std::mem;

pub fn draw_line<P:Pixel>(
    mut a: Point2D,
    mut b: Point2D,
    image: &mut TgaImage<P>,
    line_color: &P,
) {
    let mut steep = false;
    if b.y - a.y > b.x - a.x {
        mem::swap(&mut a.x, &mut a.y);
        mem::swap(&mut b.x, &mut b.y);
        steep = true;
    }
    if a.x > b.x {
        mem::swap(&mut a, &mut b)
    }

    let dx: i32 = (b.x - a.x) as i32;
    let dy: i32 = (b.y - a.y) as i32;
    let derror2: u16 = i32::abs(dy * 2) as u16;
    let mut error2: i32 = 0;
    let mut y = a.y;
    for x in a.x..=b.x {
        if steep {
            image.set(y as u16, x as u16, line_color);
        } else {
            image.set(x as u16, y as u16, line_color);
        }
        error2 += derror2 as i32;
        if error2 > dx {
            y += if b.y > a.y { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}
