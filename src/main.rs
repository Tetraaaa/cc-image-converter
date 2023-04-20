use clap::Parser;
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
    // let image_content: Result<image::DynamicImage, image::ImageError> = ImageReader::open(&args.image_path).expect("Fichier introuvable").decode();
    let resized_image = resize(&original_image, 30, 30, FilterType::Nearest);
    resized_image.save("./test.jpg").expect("Impossible d'enregistrer l'image");
}
