use image::{save_buffer, imageops};
use tokio::task::JoinHandle;
use std::fs::{self, FileType};
use clap::{self, Parser};

#[derive(clap::Parser)]
#[clap(version = "0.2.4", author = "Xomvio <xomvio at proton dot me>")]
struct Cli {
    /// Path to the watermark image.
    #[clap(value_parser)]
    watermark_path: String,

    /// Path(s) to the image(s) to be watermarked.
    #[clap(value_parser)]
    image_paths: Vec<String>,

    /// Target directory to save watermarked images. Defaults to './output'.
    #[clap(short, long, default_value = "./output/")]
    target_path: String,

    /// Target resolution for the watermarked images. reserves the aspect ratio. Defaults to the original resolution.
    #[clap(long)]
    width: Option<u32>,
    #[clap(long)]
    height: Option<u32>,

    /// Target file type for the watermarked images. Defaults to the original file type.
    #[clap(short, long)]
    filetype: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let target_path = cli.target_path.trim_end_matches('/').to_string() + "/";
    let watermark_path = cli.watermark_path.clone();
    let image_paths = cli.image_paths.clone();
    let width = cli.width;
    let height = cli.height;
    let filetype = cli.filetype.clone();

    // create output directory if it doesn't exist
    fs::create_dir_all(&target_path).expect("failed to create output directory");

    // open watermark
    let watermark_img = match image::open(&watermark_path) {
        Ok(img) => img,
        Err(e) => { println!("failed to open watermark image: {}", e); return }
    };

    let mut tasks = vec![];

    for image_path in image_paths {
        // clone variables for the async task
        let watermark_img = watermark_img.clone();
        let target_path = target_path.clone();
        let filetype = filetype.clone();

        // spawn a new task for each image to be watermarked        
        let fut = tokio::spawn(async move {
            watermarker(image_path, watermark_img, target_path, width, height, filetype).await 
        });

        tasks.push(fut);
    }
    
    // wait for all tasks to complete
    for task in tasks {
        if let Err(e) = task.await {
            println!("failed to join task: {}", e);
        }
    }
}

async fn watermarker(
    image_path: String,
    watermark_img: image::DynamicImage,
    target_path: String,
    width: Option<u32>,
    height: Option<u32>,
    filetype: Option<String>,
) {
    match fs::metadata(&image_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                // apply watermark to the image file
                tokio::spawn(async move {
                    watermark(image_path, watermark_img, target_path, width, height, filetype);
                });
            } else if metadata.is_dir() {
                watermark_dir(image_path, watermark_img, target_path, width, height, filetype).await;
            } else {
                println!("failed to get metadata for: {}\r\nthis will be skipped", image_path);
            }
        }
        Err(e) => {
            println!("failed to read metadata for {}: {}", image_path, e);
        }
    }
}

/// Processes a directory to apply a watermark to all images within it.
async fn watermark_dir(
    dir_path: String,
    watermark_img: image::DynamicImage,
    target_path: String,
    width: Option<u32>,
    height: Option<u32>,
    filetype: Option<String>,
) {
    println!("Processing directory: {}", dir_path);

    //let dir_name = dir_path.split('/').last().unwrap_or("unknown");

    fs::create_dir_all(&target_path).expect("failed to create output directory");

    for entry in fs::read_dir(&dir_path).expect("failed to read directory") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_file() {
                    continue; // skip non-file entries
                }
                let path_str = path.to_str().expect("failed to convert path to string").to_string();

                // clone variables for the async task
                let path_clone = path_str.clone();
                let watermark_img = watermark_img.clone();
                let target_path = target_path.clone();
                let target_filetype = filetype.clone();

                // spawn a new task for each image to be watermarked
                watermarker(path_clone, watermark_img, target_path, width, height, target_filetype).await;
            }
            Err(e) => {
                println!("failed to read entry in directory {}: {}", dir_path, e);
                continue;
            }
        }
    }
}

/// Applies a watermark to a single image file.
fn watermark(
    image_path: String,
    watermark_img: image::DynamicImage,
    target_path: String,
    width: Option<u32>,
    height: Option<u32>,
    filetype: Option<String>,
) {
    // open image
    let mut img = match image::open(&image_path) {
        Ok(img) => img,
        Err(e) => { println!("failed to open image {}: {}", image_path, e); return }
    };

    // resize if width or height is specified
    if width.is_some() || height.is_some() {
        let width = width.unwrap_or(img.width());
        let height = height.unwrap_or(img.height());
        img = img.resize(width, height, imageops::FilterType::Nearest);
    }

    // apply watermark
    imageops::overlay(&mut img, &watermark_img, 0, 0);

    // determine output file type
    let ext = filetype.unwrap_or_else(|| {
        image_path.split('.').next_back().unwrap_or("png").to_string()
    });

    // save the watermarked image
    let output_path = format!("{}{}.{}", target_path, image_path.split('/').next_back().unwrap().split('.').next().unwrap(), ext);
    match save_buffer(output_path.clone(), img.as_bytes(), img.width(), img.height(), img.color()) {
        Ok(_) => println!("Watermarked image saved to {}", output_path),
        Err(e) => println!("Failed to save watermarked image: {}", e),
    }
}