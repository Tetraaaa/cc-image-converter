use std::fs;

use clap::Parser;
use image::{DynamicImage, ImageBuffer, GenericImageView, GenericImage};
use image::imageops::{resize as resize, FilterType};

#[derive(Parser)]
struct CliArguments {
    #[arg(short = 'i', long="input")]
    image_path: std::path::PathBuf,
    #[arg(short = 'o', long="output")]
    output_file: std::path::PathBuf,
    #[arg(long="width")]
    width: u32,
    #[arg(long="height")]
    height: u32,
}

fn main() {
    let args: CliArguments = CliArguments::parse();

    let original_image: image::DynamicImage  = image::open(&args.image_path).expect("Fichier introuvable");
    let resized_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = resize_to_ccscreen_resolution(original_image, &args.width, &args.height);
    let image_as_cc_string = convert_to_cc_string(image::DynamicImage::ImageRgba8(resized_image));
    fs::write(&args.output_file, image_as_cc_string).expect("Impossible de générer le fichier CC.");

}

fn resize_to_ccscreen_resolution(original_image: DynamicImage, new_width:&u32, new_height:&u32) -> ImageBuffer<image::Rgba<u8>, Vec<u8>>
{
    let resized_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = resize(&original_image, *new_width, *new_height, FilterType::Triangle);
    // resized_image.save("./out1.png").expect("Impossible d'enregistrer l'image");
    return resized_image;
}

fn convert_to_cc_string(original_image: DynamicImage) -> String 
{
    let mut image_string = String::new();
    let mut image_copy: DynamicImage = original_image.clone();
    let dim: (u32, u32) = original_image.dimensions();
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            let new_color = convert_to_cc_colorspace(image_copy.get_pixel(x, y));
            image_string.push_str(new_color.paint.to_string().as_str());
            image_copy.put_pixel(x, y, new_color.color);
        }
        image_string.push_str("\n");
    }
    // image_copy.save("./out2.png").expect("Impossible d'enregistrer l'image");
    return image_string;
    // return image_copy;
}

fn convert_to_cc_colorspace(original_pixel:image::Rgba<u8>) -> CcColor
{
    const CC_COLORS: [CcColor; 16] = [
        CcColor {paint: '0', color: image::Rgba([240, 240, 240, 255])},
        CcColor {paint: '1', color: image::Rgba([242, 178, 51, 255])},
        CcColor {paint: '2', color: image::Rgba([229, 127, 216, 255])},
        CcColor {paint: '3', color: image::Rgba([153, 178, 242, 255])},
        CcColor {paint: '4', color: image::Rgba([222, 222, 108, 255])},
        CcColor {paint: '5', color: image::Rgba([127, 204, 25, 255])},
        CcColor {paint: '6', color: image::Rgba([242, 178, 204, 255])},
        CcColor {paint: '7', color: image::Rgba([76, 76, 76, 255])},
        CcColor {paint: '8', color: image::Rgba([153, 153, 153, 255])},
        CcColor {paint: '9', color: image::Rgba([76, 153, 178, 255])},
        CcColor {paint: 'a', color: image::Rgba([178, 102, 229, 255])},
        CcColor {paint: 'b', color: image::Rgba([51, 102, 204, 255])},
        CcColor {paint: 'c', color: image::Rgba([127, 102, 76, 255])},
        CcColor {paint: 'd', color: image::Rgba([87, 166, 78, 255])},
        CcColor {paint: 'e', color: image::Rgba([204, 76, 76, 255])},
        CcColor {paint: 'f', color: image::Rgba([25, 25, 25, 255])}
    ];
    let mut minimal_distance = 255;
    let mut closest_color: CcColor= CC_COLORS[0].clone();  

    for color in CC_COLORS {
        let distance: i16 = color_distance(original_pixel, color.color);
        if distance < minimal_distance {
            minimal_distance = distance;
            closest_color = color;
        }
    }
    return closest_color;
    
}

fn color_distance(color1:image::Rgba<u8>, color2:image::Rgba<u8>) -> i16
{
    let result:i16 = (color1.0[0] as i16 - color2.0[0] as i16).abs() + (color1.0[1] as i16 - color2.0[1] as i16).abs() + (color1.0[2] as i16 - color2.0[2] as i16).abs();
    return result;
}

#[derive(Clone)]
struct CcColor {
    paint: char,
    color: image::Rgba<u8>
}