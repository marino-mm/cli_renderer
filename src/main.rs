use std::fs::File;
use std::io::{stdout, Write};
use std::path::PathBuf;

pub const PROJECT_ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");
pub const IMAGES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images");

struct  Vec2{
    x:i32,
    y:i32
}
struct  Vec3{
    x:i32,
    y:i32,
    z:i32
}
#[derive(Copy, Clone)]
struct Color{
    r:u8,
    g:u8,
    b:u8
}
#[derive(Copy, Clone)]
struct PixelWithColor {
    brightness:u8,
    color:Color
}



fn generate_ppm_file(){
    let mut file_output = String::new();
    let image_width = 25;
    let image_height = 25;
    file_output.push_str("P3\n");
    file_output.push_str(format!("{0} {1}\n255\n", image_width, image_height).as_str());
    for y in 0..image_height {
        for x in 0..image_width {
            let r = x as f32 / image_width as f32;
            let g = y as f32 / image_height as f32;
            // let b = 0.25;
            let b = 0.0;

            let r = (r * 256f32) as i32;
            let g = (g * 256f32) as i32;
            let b = (b * 256f32) as i32;

            file_output.push_str(format!("{0} {1} {2} ", r, g, b).as_str());
        }
        file_output.push_str("\n");
    }

    // print!("{}", file_output);
    let file_path = PathBuf::from(IMAGES_DIR).join("out_2.ppm");
    let mut file = File::create(file_path).unwrap();
    file.write(&file_output.as_bytes()).unwrap();
}

fn draw_2_point_in_terminal(){

    let width = 10;
    let height = 10;

    // let mut draw_matrix:Vec<PixelWithColor> = vec![PixelWithColor { brightness: 0, color: Color{ r: 0, g: 0, b: 0 } }; width * height];
    // let middle = (width * height) / 2;
    // draw_matrix[middle - 2] = PixelWithColor { brightness: 1, color: Color{ r: 255, g: 0, b: 0 } };
    // draw_matrix[middle ] = PixelWithColor { brightness: 1, color: Color{ r: 0, g: 255, b: 0 } };
    // draw_matrix[middle + 2] = PixelWithColor { brightness: 1, color: Color{ r: 0, g: 0, b: 255 } };

    let mut draw_matrix = Vec::<PixelWithColor>::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.50;

            let r = (r * 256f32) as u8;
            let g = (g * 256f32) as u8;
            let b = (b * 256f32) as u8;

            draw_matrix.push(
                PixelWithColor { brightness: 1, color: Color { r, g, b } }
            )
        }
    }

    for line in 0..height{
        for col in 0..width{
            let pixel_pos = col + line * width;
            let pixel = &draw_matrix[pixel_pos];
            if pixel.brightness == 0{
                stdout().write_all(b".").unwrap();
            }
            else {
                let out_buff = format!("\x1B[38;2;{};{};{}m0\x1B[0m", pixel.color.r, pixel.color.g, pixel.color.b);
                stdout().write_all(out_buff.as_bytes()).unwrap();
            }
        }
        stdout().write(b"\n").unwrap();
    }
}

fn draw_braille(){

    let char_width: usize = 25;
    let char_height: usize = 25;

    let pixel_width = char_width * 2;
    let pixel_height = char_height * 4;

    let center_x = (pixel_width / 2) as i32;
    let center_y = (pixel_height / 2) as i32;
    let circle_size = char_width as i32 / 2;

    let mut pixel_matrix = vec![0; pixel_width * pixel_height];

    for y in 0..pixel_height {
        for x in 0..pixel_width {
            let dx = x as i32 - center_x;
            let dy = y as i32 - center_y;

            // Standard circle equation: x^2 + y^2 < r^2
            if (dx * dx) + (dy * dy) < circle_size * circle_size {
                pixel_matrix[y * pixel_width + x] = 1;
            }
        }
    }

    let braille_map = [
        (0,0),
        (0,1),
        (0,2),
        (1,0),
        (1,1),
        (1,2),
        (0,3),
        (1,3),
    ];
    let mut buf = [0; 4];
    for y in 0..char_height {
        for x in 0..char_width {
            let mut unicode_number = 0;
            for (potential, (x_off, y_off)) in braille_map.iter().enumerate(){
                let real_pixel_x = x * 2 + x_off;
                let real_pixel_y = y * 4 + y_off;
                let pixel_pos = real_pixel_x + real_pixel_y * pixel_width;
                if pixel_matrix[pixel_pos] == 1{
                    unicode_number += 1 << potential;
                }
            }
            let c = char::from_u32(0x2800 + unicode_number as u32).unwrap();
            let encoded_bytes = c.encode_utf8(&mut buf).as_bytes();
            stdout().write_all(encoded_bytes).unwrap();
        }
        stdout().write(b"\n").unwrap();
    }
}

macro_rules! time_if {
    ($name:ident($($arg:expr),*)) => {
        use std::time::Instant;
        let start = Instant::now();
        let result = $name($($arg),*);
        let duration = start.elapsed();
        println!("{}: {:?}", stringify!($name), duration);
        result
    };
}

fn main() {

    // generate_ppm_file();
    draw_2_point_in_terminal();
    // time_if!(draw_braille());
}
