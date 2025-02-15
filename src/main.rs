use image::{save_buffer, imageops};
use std::{fs, env};


    /// The main function of the program.
    ///
    /// This function parses command line arguments, creates output directory if it doesn't exist,
    /// opens watermark image, and then for each image path provided:
    /// opens the image, resizes it if width and height are provided, overlays the watermark,
    /// and saves it to the output directory.
    ///
#[tokio::main]
async fn main() {
    let help: String = "
Usage:
    watermarker [--path <target_path>] [--target-resolution <width> <height>] [--filetype <filetype>] <watermark_path> <image_path> [image_path]...

Arguments:
    --path <target_path>         (Optional) Target directory to save watermarked images. Defaults to './output'.
    --res <width> <height>       (Optional) Target resolution for the watermarked images. reserves the aspect ratio. Defaults to the original resolution.
    --filetype <filetype>        (Optional) Target file type for the watermarked images. Defaults to the original file type.
    <watermark_path>             Path to the watermark image.
    <image_path>                 Path(s) to the image(s) to be watermarked.

Examples:
    watermarker watermark.png image1.jpg
    watermarker watermark.png image1.jpg image2.png
    watermarker watermark.png /home/user/images/
    watermarker --path ./target --res 300 500 --filetype jpg watermark.png image1.png image2.png image3.png
".to_string();

    let args = env::args().collect::<Vec<String>>();

    // env_index is used to iterate through command line arguments
    let mut env_index = 0;

    // default values
    let mut target_path=String::from("./output/");
    let mut target_resolution: Option<(u32, u32)> = None;
    let mut target_filetype: Option<String> = None;

    // parse first command line arguments if any
    loop {
        env_index += 1;
        match args.get(env_index) {
            Some(arg) => {
                if arg == "--path" { // change target path
                    env_index += 1;
                    target_path = args.get(env_index).expect("path is not specified.").to_string() + "/";
                }
                else if arg == "--res" { // change target resolution
                    env_index += 2;
                    target_resolution = Some((
                        args.get(env_index - 1)
                        .expect("width is not specified.")
                        .parse().expect("invalid width."),
                        args.get(env_index)
                        .expect("height is not specified.")
                        .parse().expect("invalid height."),
                    ));
                }
                else if arg == "--filetype" { // change target file type
                    env_index += 1;
                    target_filetype = Some(".".to_string() + args.get(env_index).expect("filetype is not specified.").trim_start_matches('.'));
                }
                else {
                    break
                }
            },
            None => { // no command line arguments
                println!("your need to provide a watermark and at least one image\r\n {}", help);
                return;
            }
        }
    }

    // create output directory if it doesn't exist
    fs::create_dir_all(&target_path).expect("failed to create output directory");

    let watermark_path = args.get(env_index).expect("watermark is not specified.").to_string();
    env_index += 1;

    // open watermark
    let watermark_img = match image::open(&watermark_path) {
        Ok(img) => img,
        Err(e) => { println!("failed to open watermark image: {}", e); return }
    };

    // let the tokio go wild
    for path in args.iter().skip(env_index) {

        // clone variables if necessary so they can be moved into the spawned thread
        let path_clone = path.clone();
        let watermark_img = watermark_img.clone();
        let target_path = target_path.clone();
        let target_filetype = target_filetype.clone();


        // spawn a new thread
        tokio::spawn(async move {
            watermarker(path_clone, watermark_img, target_path, target_resolution, target_filetype);
        });
    }
}

fn watermark_dir(path: String, watermark_img: image::DynamicImage, target_path: String, target_resolution: Option<(u32, u32)>, target_filetype: Option<String>) {
    println!("directory found, processing all files in this directory: {}", path);
    let dir_name = path.clone();
    let dir_name = dir_name.split("/").last().expect("failed to get directory name");
    fs::create_dir_all(target_path.to_owned() + dir_name + "/").expect("failed to create output directory");
    for entry in fs::read_dir(&path).expect("failed to read directory") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    let path = path.to_str().expect("failed to get path").to_string();
                    watermark(path, watermark_img.clone(), target_path.clone() + dir_name + "/", target_resolution, target_filetype.clone());
                }
                else if path.is_dir() {
                    let path = path.to_str().expect("failed to get path").to_string();
                    watermark_dir(path, watermark_img.clone(), target_path.clone() + dir_name + "/", target_resolution, target_filetype.clone());
                }
            },
            Err(e) => println!("failed to get entry for:{} {}\r\nthis will be skipped", path, e)
        }
    }
}

fn watermarker(path: String, watermark_img: image::DynamicImage, target_path: String, target_resolution: Option<(u32, u32)>, target_filetype: Option<String>) {
    match fs::metadata(&path) {
        Ok(metadata) => {

            if metadata.is_file() {
                watermark(path, watermark_img, target_path, target_resolution, target_filetype);
            }
            else if metadata.is_dir() {
                watermark_dir(path, watermark_img, target_path, target_resolution, target_filetype);
            }
            else {
                println!("failed to get metadata for: {}\r\nthis will be skipped", path);
            }

        },
        Err(e) => {
            println!("failed to get metadata for: {}: {}\r\nthis will be skipped.", path, e);
        }
    }
}

fn watermark(path: String, watermark_img: image::DynamicImage, target_path: String, target_resolution: Option<(u32, u32)>, target_filetype: Option<String>) {
    println!("watermarking: {}", path);
    // gets the filename from the path
    // if target_filetype is set, we need to change the file type            
    let filename = match target_filetype {
        Some(filetype) => {
            match path.split("/").last() {
                Some(filename) => {
                    match filename.split(".").next() {
                        Some(filename) => filename.to_string() + &filetype,
                        None => { println!("failed to get file type for: {}", path); return }
                    }
                },
                None => { println!("failed to get filename from path: {}", path); return }
            }
        }
        None => {
            match path.split("/").last() {
                Some(filename) => filename.to_string(),
                None => { println!("failed to get filename from path: {}", path); return }
            }
        }
    };

    // open image
    let mut img = match image::open(&path) {
        Ok(img) => img,
        Err(e) => { println!("failed to open image: {}\r\nthis will be skipped", e); return }
    };

    // resize image if target resolution is set
    if let Some((width, height)) = target_resolution {
        img = img.resize(width, height, image::imageops::Nearest);
    }

    // overlay watermark
    imageops::overlay(&mut img, &watermark_img, 0, 0);

    // save image
    match save_buffer(&(target_path.to_owned() + &filename), img.as_bytes(), img.width(), img.height(), img.color()) {
        Ok(_) => {},
        Err(e) => { println!("failed to save '{}' this will be skipped. {}", filename, e); /*no 'return' here because function ends*/ }
    }
}