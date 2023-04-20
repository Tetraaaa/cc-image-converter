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
    let mut image_copy = original_image.clone();
    let dim: (u32, u32) = original_image.dimensions();
    for x in 0..dim.0 {
        for y in 0..dim.1 {
            image_copy.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
        }
    }
    image_copy.save("./out1.png").expect("Impossible d'enregistrer l'image");
    return image_copy;
}

