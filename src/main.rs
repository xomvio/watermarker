use photon_rs::{multiple::watermark, native::save_image};
//use chrono;
use std::env;

    /// watermarker [--path <target_path>] <watermark_path> <image_path> [image_path]...
    ///
    /// Adds a watermark to a list of images. The watermark is the first argument
    /// and the images are the rest of the arguments
    ///
    /// If --path is specified, the following argument is the target path for the
    /// watermarked images. Otherwise, the watermarked images are written to the
    /// same directory as the original images, with "watermarked_" prefixed to the
    /// filename.
    ///
    /// If the watermark is not found, the program will crash. If any of the images
    /// are not found, the program will crash.
    ///
    ///
#[tokio::main]
async fn main() {

    let help: String = "
Usage:
    watermarker [--path <target_path>] <watermark_path> <image_path> [image_path]...

Arguments:
    --path <target_path>  (Optional) Target directory to save watermarked images. Defaults to './output'.
    <watermark_path>      Path to the watermark image.
    <image_path>          Path(s) to the image(s) to be watermarked.
".to_string();

    let args = env::args().collect::<Vec<String>>();

    let mut env_index = 1;
    let mut target_path = String::new();
    let watermark_path;
    
    match args.get(1) {
        Some(arg) => {
            if arg == "--path" {
                target_path = String::from(args.get(env_index + 1).expect(&help).to_string() + "/");
                watermark_path = args.get(env_index + 2).expect(&help).to_string();
                env_index += 3;
            } else {
                target_path = String::from("./output/");
                watermark_path = arg.to_string();
                env_index += 1;
                //env_index += 2;
            }
        },
        None => {
            println!("your need to provide a watermark and at least one image\r\n {}", help);
            return;
        }
    }

    //create output directory if it doesn't exist
    std::fs::create_dir_all(&target_path).unwrap();

    //let mut timer = chrono::Local::now();
    let watermark_img = photon_rs::native::open_image(&watermark_path).unwrap();
    //println!("get watermark: {} ms", chrono::Local::now().timestamp_millis() - timer.timestamp_millis());

    //println!("watermarking {} images", args.len() - env_index);
    for index in env_index..args.len() {
        let args_clone = args.clone();
        let watermark_img = watermark_img.clone();
        let target_path = target_path.clone();
        //timer = chrono::Local::now();
        tokio::spawn(async move {
            let mut img = photon_rs::native::open_image(&args_clone[index]).unwrap();
            watermark(&mut img, &watermark_img, 0,0);
            save_image(img, &(target_path.to_owned() + &args_clone[index])).unwrap();
            //println!("watermarked {} in {} ms", args_clone[index], chrono::Local::now().timestamp_millis() - timer.timestamp_millis());
        });
    }
}
