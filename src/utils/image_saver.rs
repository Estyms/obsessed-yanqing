use std::fs;
use std::io::Write;

fn create_images_folder() {
    match fs::create_dir("./images") {
        Ok(_) => {}
        Err(_) => {}
    }
}
pub fn write_image_from_b64(name: String, b64_image: String) -> String {
    create_images_folder();
    let data = image_base64::from_base64(b64_image.into());
    let mut_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("./images/{}.webp", name));
    mut_file.unwrap().write_all(data.as_slice()).expect("Cannot write image");
    return format!("{}.webp", name)
}
