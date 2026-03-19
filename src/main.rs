use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub const PROJECT_ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");
pub const IMAGES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images");
fn generate_ppm_file(){
    let mut file_output = String::new();
    let image_width = 512;
    let image_height = 512;
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

fn main() {
    generate_ppm_file();
}
