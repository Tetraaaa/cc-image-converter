use clap::Parser;
use image::{DynamicImage, ImageBuffer, GenericImageView, GenericImage};
use image::io::Reader as ImageReader;
use image::imageops::{resize as resize, FilterType};

#[derive(Parser)]
struct CliArguments {
    // #[arg(short = 'i', long="input")]
    image_path: std::path::PathBuf
}

fn main() {
    let args: CliArguments = CliArguments::parse();
    let original_image: image::DynamicImage  = image::open(&args.image_path).expect("Fichier introuvable");
    let recolored_image = restrict_color_space(original_image);
    // let resized_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = resize_to_ccscreen_resolution(original_image);

}

fn resize_to_ccscreen_resolution(original_image: DynamicImage) -> ImageBuffer<image::Rgba<u8>, Vec<u8>>
{
    let resized_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = resize(&original_image, 64, 64, FilterType::Triangle);
    resized_image.save("./out2.png").expect("Impossible d'enregistrer l'image");
    return resized_image;
}

fn restrict_color_space(original_image: DynamicImage) -> DynamicImage
{
    let mut image_copy: DynamicImage = original_image.clone();
    let dim: (u32, u32) = original_image.dimensions();
    for x in 0..dim.0 {
        for y in 0..dim.1 {
            image_copy.put_pixel(x, y, convert_to_cc_colorspace(image_copy.get_pixel(x, y)));
        }
    }
    image_copy.save("./out1.png").expect("Impossible d'enregistrer l'image");
    return image_copy;
}

fn convert_to_cc_colorspace(original_pixel:image::Rgba<u8>) -> image::Rgba<u8>
{
    let colors: [image::Rgba<u8>; 16] = [
        image::Rgba([240, 240, 240, 255]),
        image::Rgba([242, 178, 51, 255]),
        image::Rgba([229, 127, 216, 255]),
        image::Rgba([153, 178, 242, 255]),
        image::Rgba([222, 222, 108, 255]),
        image::Rgba([127, 204, 25, 255]),
        image::Rgba([242, 178, 204, 255]),
        image::Rgba([76, 76, 76, 255]),
        image::Rgba([153, 153, 153, 255]),
        image::Rgba([76, 153, 178, 255]),
        image::Rgba([178, 102, 229, 255]),
        image::Rgba([51, 102, 204, 255]),
        image::Rgba([127, 102, 76, 255]),
        image::Rgba([87, 166, 78, 255]),
        image::Rgba([204, 76, 76, 255]),
        image::Rgba([25, 25, 25, 255])
    ];
    let mut minimal_distance = 255;
    let mut closest_color = colors[0];
    
    for color in colors {
        let distance: i16 = color_distance(original_pixel, color);
        if distance < minimal_distance {
            minimal_distance = distance;
            closest_color = color;
        }
    }
    return closest_color;
}

fn color_distance(color1:image::Rgba<u8>, color2:image::Rgba<u8>) -> i16
{
    // println!("{:?}", color1.0[0] - color2.0[0]);
    // return 1;
    let result:i16 = (color1.0[0] as i16 - color2.0[0] as i16).abs() + (color1.0[1] as i16 - color2.0[1] as i16).abs() + (color1.0[2] as i16 - color2.0[2] as i16).abs();
    return result;
}