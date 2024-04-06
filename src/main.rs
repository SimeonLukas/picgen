#[macro_use]
extern crate rocket;
extern crate imageproc;

use image::{ImageBuffer, RgbImage};
use image::GenericImage;
use image::io::Reader as ImageReader;
use rocket::fs::NamedFile;
use std::path::Path;
use imageproc::drawing::draw_text_mut;
use ab_glyph::FontArc;



#[get("/?<title>&<ort>&<bg>&<zeit>")]
async fn index(
    title: Option<String>,
    ort: Option<String>,
    bg: Option<String>,
    zeit: Option<String>

) -> Option<NamedFile> {
    let font = FontArc::try_from_slice(include_bytes!("../fonts/Roboto-Regular.ttf")).unwrap();
    let ort = ort.unwrap_or("St. Martin Garmisch-Partenkirchen".to_string());
    let zeit = zeit.unwrap_or("19:00 - 21:00 Uhr".to_string());
    let bg = bg.unwrap_or("white.png".to_string());
    let img = ImageReader::open("static/".to_owned() + &bg).unwrap().decode().unwrap();
    let bg = img.into_rgb8();
    let width = bg.width();
    let height = bg.height();
    let mut image: RgbImage = ImageBuffer::new(width, height);
    image.copy_from(&bg, 0, 0).unwrap();


    draw_text_mut(&mut image, image::Rgb([255, 255, 255]), 130, 1000, 50.0, &font , &ort);
    draw_text_mut(&mut image, image::Rgb([255, 255, 255]), 130, 1110, 50.0, &font , &zeit);
    if let Some(title) = title {
        let path = Path::new(&title);
        let path = path.with_extension("png");
        image.save(&path).unwrap();
        return Some(NamedFile::open(&path).await.unwrap());
    }
    else {
        image.save("output.png").unwrap();
        return Some(NamedFile::open("output.png").await.unwrap());
    }
    
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

