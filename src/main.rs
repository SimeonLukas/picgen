#[macro_use]
extern crate rocket;
extern crate imageproc;

use ab_glyph::FontArc;
use image::imageops::blur;
use image::imageops::FilterType::Nearest;
use image::imageops::{overlay, rotate180_in};
use image::io::Reader as ImageReader;
use image::DynamicImage::*;
use image::GenericImage;
use image::{ImageBuffer, Pixel, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, Canvas};
use imageproc::geometric_transformations::*;
use rocket::fs::NamedFile;
use rocket::http::ext;
use rocket::response::content;
use std::fmt::Arguments;
use std::io::Read;
use std::path::Path;
use std::env;

#[get("/?<file>&<i1>&<i2>&<i3>&<i4>&<i5>&<i6>&<i7>&<i8>&<i9>&<i10>")]
async fn index(
    file: Option<String>,
    i1: Option<String>,
    i2: Option<String>,
    i3: Option<String>,
    i4: Option<String>,
    i5: Option<String>,
    i6: Option<String>,
    i7: Option<String>,
    i8: Option<String>,
    i9: Option<String>,
    i10: Option<String>,
) -> Option<NamedFile> {
    // Implement Fonts
    let roboto = FontArc::try_from_slice(include_bytes!("../fonts/roboto.ttf")).unwrap();
    let icons = FontArc::try_from_slice(include_bytes!("../fonts/icons.ttf")).unwrap();

    let file = file.unwrap_or("template1.json".to_string());
    let i1 = i1.unwrap_or("".to_string());
    let i2 = i2.unwrap_or("".to_string());
    let i3 = i3.unwrap_or("".to_string());
    let i4 = i4.unwrap_or("".to_string());
    let i5 = i5.unwrap_or("".to_string());
    let i6 = i6.unwrap_or("".to_string());
    let i7 = i7.unwrap_or("".to_string());
    let i8 = i8.unwrap_or("".to_string());
    let i9 = i9.unwrap_or("".to_string());
    let i10 = i10.unwrap_or("".to_string());

    // load json from static template
    let mut template = String::new();
    let mut file = std::fs::File::open("static/".to_owned() + &file).unwrap();
    file.read_to_string(&mut template).unwrap();
    let template: serde_json::Value = serde_json::from_str(&template).unwrap();
    // load background image and set width and height background
    let title = template["config"][0]["title"].as_str();
    let bottomlayer = template["config"][0]["background"].as_str().unwrap();
    let image = ImageReader::open("static/".to_owned() + &bottomlayer)
        .unwrap()
        .decode()
        .unwrap();
    let mut image = image.to_rgba8();
    let width = image.width();
    let height = image.height();
    let mut counter = 0;
    // get all content values in template
    for content in template["config"][0]["content"].as_array().unwrap() {
        counter += 1;
        let key = counter.to_string();
        let key = key.as_str();
        // create layer
        let mut layer = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 0]));
        // draw text by json values
        let scale: f32 = content["size"].to_string().parse().unwrap_or(50.0);
        let r: u8 = content["color"][0].to_string().parse().unwrap_or(255);
        let g: u8 = content["color"][1].to_string().parse().unwrap_or(255);
        let b: u8 = content["color"][2].to_string().parse().unwrap_or(255);
        let a: u8 = content["color"][3].to_string().parse().unwrap_or(255);
        let x = content["position"][0].to_string().parse().unwrap_or(0);
        let y = content["position"][1].to_string().parse().unwrap_or(0);
        let font = content["font"].to_string();
        let font = match font.as_str() {
            "Roboto" => roboto.clone(),
            "Icons" => icons.clone(),
            _ => roboto.clone(),
        };
        // get text of input
        let text = match key {
            "1" => i1.clone(),
            "2" => i2.clone(),
            "3" => i3.clone(),
            "4" => i4.clone(),
            "5" => i5.clone(),
            "6" => i6.clone(),
            "7" => i7.clone(),
            "8" => i8.clone(),
            "9" => i9.clone(),
            "10" => i10.clone(),
            _ => "".to_string(),
        };

        draw_text_mut(
            &mut layer,
            image::Rgba([r, g, b, a]),
            x,
            y,
            scale,
            &font,
            &text,
        );

        // blur layer
        if content["shadow"].is_array() {
            let mut shadowlayer = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 0]));

            let shadow = content["shadow"].as_array().unwrap();
            let xoffset = shadow[0].to_string().parse().unwrap_or(0);
            let yoffset = shadow[1].to_string().parse().unwrap_or(0);
            let sigma = shadow[2].to_string().parse().unwrap_or(0.0);
            let color = shadow[3].as_array().unwrap();
            let color = [
                color[0].to_string().parse().unwrap_or(0),
                color[1].to_string().parse().unwrap_or(0),
                color[2].to_string().parse().unwrap_or(0),
                color[3].to_string().parse().unwrap_or(0),
            ];
            draw_text_mut(
                &mut shadowlayer,
                image::Rgba(color),
                x + xoffset,
                y + yoffset,
                scale,
                &font,
                &text,
            );

            if content["rotate"].is_u64() {
                let theta: u64 = content["rotate"].to_string().parse().unwrap_or(0);
                let theta = theta as f32 * std::f32::consts::PI / 180.0;
                shadowlayer = rotate(
                    &shadowlayer,
                    (x as f32, y as f32),
                    theta,
                    Interpolation::Bicubic,
                    Rgba([0, 0, 0, 0]),
                );
            }

            let shadowlayer = blur(&shadowlayer, sigma);
            overlay(&mut image, &shadowlayer, 0, 0);
        }

        if content["rotate"].is_u64() {
            let theta: u64 = content["rotate"].to_string().parse().unwrap_or(0);
            let theta = theta as f32 * std::f32::consts::PI / 180.0;
            layer = rotate(&layer, (x as f32, y as f32), theta, Interpolation::Bicubic, Rgba([0, 0, 0, 0]));
        }

        overlay(&mut image, &layer, 0, 0);
    }

    if let Some(title) = title {
        let path = Path::new(&title);
        let path = path.with_extension("png");
        image.save(&path).unwrap();
        return Some(NamedFile::open(&path).await.unwrap());
    } else {
        image.save("output.png").unwrap();
        return Some(NamedFile::open("output.png").await.unwrap());
    }
}

#[launch]
fn rocket() -> _ {
    for argument in std::env::args() {
        // get port
        if argument == "--port" {
            let port: u16 = std::env::args().nth(2).unwrap().parse().unwrap();
            std::env::set_var("PORT", port.to_string());
        }
    }

    let port: u16 = std::env::var("PORT").unwrap_or("2468".to_string()).parse().unwrap();

    rocket::build()
    .configure(rocket::Config::figment().merge(("port", port)))
    .mount("/", routes![index])
}
