use rusty_renderer::core::Point2D;
use rusty_renderer::tga::{Color, Rgb888, TgaImage};
use std::fs::File;
use std::mem;

#[test]
fn valid() {
    assert_eq!(1, 1);
}

#[test]
fn valid_set() {
    let mut fd = File::create("/tmp/renderer/test_2.tga").unwrap();
    let mut image: TgaImage<Rgb888> = TgaImage::new(1000, 1000).unwrap();
    for i in 0..500 {
        image.set(
            i,
            i,
            &Rgb888 {
                r: 255,
                g: 255,
                b: 255,
            },
        );
    }
}

// #[test]
fn check_line_drawing_v1() {
    let mut image: TgaImage<Rgb888> = TgaImage::new(256, 256).unwrap();
    let mut fd = File::create("/tmp/renderer/test_line_v1.tga").unwrap();
    let a = Point2D { x: 10, y: 10 };
    let b = Point2D { x: 20, y: 50 };
    // draw_line(a, b, &mut image);
    // let a = Point2D { x: 90, y: 90 };
    // let b = Point2D { x: 10, y: 50 };
    draw_line(a, b, &mut image);
    image.write(&mut fd).unwrap();
}

fn draw_line(a: Point2D, b: Point2D, image: &mut TgaImage<Rgb888>) {
    if a.x > b.x {
        draw_line(b, a, image);
        return;
    }
    // Check where we get less steep angle
    let line_color = Rgb888 {
        r: 255,
        g: 255,
        b: 255,
    };
    let k: f32 = (b.y - a.y) as f32 / (b.x - a.x) as f32;
    for i in 0..=b.x - a.x {
        image.set(
            (i + a.x) as u16,
            f32::floor(i as f32 * k + a.y as f32) as u16,
            &line_color,
        );
    }
}

// #[test]
fn check_line_drawing_v2() {
    let mut image: TgaImage<Rgb888> = TgaImage::new(256, 256).unwrap();
    let mut fd = File::create("/tmp/renderer/test_line_v2.tga").unwrap();
    let a = Point2D { x: 10, y: 10 };
    let b = Point2D { x: 20, y: 50 };
    // draw_line(a, b, &mut image);
    // let a = Point2D { x: 90, y: 90 };
    // let b = Point2D { x: 10, y: 50 };
    draw_line_v2(a, b, &mut image);
    image.write(&mut fd).unwrap();
}
fn draw_line_v2(a: Point2D, b: Point2D, image: &mut TgaImage<Rgb888>) {
    if a.x > b.x {
        draw_line(b, a, image);
        return;
    }
    // Check where we get less steep angle
    let line_color = Rgb888 {
        r: 255,
        g: 255,
        b: 255,
    };
    if b.y - a.y < b.x - a.x {
        let k: f32 = (b.y - a.y) as f32 / (b.x - a.x) as f32;
        for i in 0..=b.x - a.x {
            image.set(
                (i + a.x) as u16,
                f32::floor(i as f32 * k + a.y as f32) as u16,
                &line_color,
            );
        }
    } else {
        let k: f32 = (b.x - a.x) as f32 / (b.y - a.y) as f32;
        for i in 0..=b.y - a.y {
            image.set(
                f32::floor(i as f32 * k + a.x as f32) as u16,
                (i + a.y) as u16,
                &line_color,
            );
        }
    }
}

// #[test]
fn check_line_drawing_v3() {
    let mut image: TgaImage<Rgb888> = TgaImage::new(256, 256).unwrap();
    let mut fd = File::create("/tmp/renderer/test_line_v3.tga").unwrap();
    let a = Point2D { x: 10, y: 10 };
    let b = Point2D { x: 20, y: 50 };
    // draw_line(a, b, &mut image);
    // let a = Point2D { x: 90, y: 90 };
    // let b = Point2D { x: 10, y: 50 };
    rusty_renderer::core::draw_line(a.clone(), b.clone(), &mut image, &Rgb888{r:190, g:190, b:190});
    // draw_line_v3(a, b, &mut image);
    image.write(&mut fd).unwrap();
}

fn draw_line_v3(mut a: Point2D, mut b: Point2D, image: &mut TgaImage<Rgb888>) {
    let mut steep = false;
    if b.y - a.y > b.x - a.x {
        mem::swap(&mut a.x, &mut a.y);
        mem::swap(&mut b.x, &mut b.y);
        steep = true;
    }
    if a.x > b.x {
        mem::swap(&mut a, &mut b)
    }

    let line_color = Rgb888 {
        r: 255,
        g: 255,
        b: 255,
    };
    let dx: i32 = (b.x - a.x) as i32;
    let dy: i32 = (b.y - a.y) as i32;
    let derror2: u16 = (dy * 2) as u16;
    let mut error2: i32 = 0;
    let mut y = a.y;
    for x in a.x..=b.x {
        if steep {
            image.set(y as u16, x as u16, &line_color);
        } else {
            image.set(x as u16, y as u16, &line_color);
        }
        error2 += derror2 as i32;
        if error2 > dx {
            y += if b.y > a.y { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}
