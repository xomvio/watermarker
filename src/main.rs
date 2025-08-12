use anyhow::{Context, Result};
use clap::Parser;
use image::{imageops, DynamicImage, ImageFormat};
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Parser)]
#[command(version = "0.3.0", author = "xomvio <xomvio at proton dot me>")]
#[command(about = "A CLI tool for adding watermarks to images")]
struct Cli {
    /// Path to the watermark image
    watermark_path: PathBuf,

    /// Path(s) to the image(s) to be watermarked
    image_paths: Vec<PathBuf>,

    /// Target directory to save watermarked images
    #[arg(short, long, default_value = "./output")]
    target_path: PathBuf,

    /// Target width for the watermarked images (preserves aspect ratio)
    #[arg(long)]
    width: Option<u32>,

    /// Target height for the watermarked images (preserves aspect ratio)
    #[arg(long)]
    height: Option<u32>,

    /// Target file format for the watermarked images
    #[arg(short, long)]
    format: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let watermarker = Watermarker::new(
        cli.watermark_path,
        cli.target_path,
        cli.width,
        cli.height,
        cli.format,
    ).await?;
    
    watermarker.process_paths(cli.image_paths).await?;
    
    Ok(())
}

struct Watermarker {
    watermark: DynamicImage,
    target_dir: PathBuf,
    width: Option<u32>,
    height: Option<u32>,
    format: Option<ImageFormat>,
}

impl Watermarker {
    async fn new(
        watermark_path: PathBuf,
        target_dir: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        format: Option<String>,
    ) -> Result<Self> {
        fs::create_dir_all(&target_dir)
            .await
            .context("Failed to create output directory")?;
            
        let watermark = image::open(&watermark_path)
            .context("Failed to open watermark image")?;
            
        let format = format.as_deref().map(Self::parse_format).transpose()?;
        
        Ok(Self {
            watermark,
            target_dir,
            width,
            height,
            format,
        })
    }
    
    fn parse_format(format_str: &str) -> Result<ImageFormat> {
        match format_str.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::WebP),
            "bmp" => Ok(ImageFormat::Bmp),
            "tiff" => Ok(ImageFormat::Tiff),
            _ => anyhow::bail!("Unsupported format: {}", format_str),
        }
    }
    
    async fn process_paths(&self, paths: Vec<PathBuf>) -> Result<()> {
        let tasks: Vec<_> = paths
            .into_iter()
            .map(|path| {
                let watermark = self.watermark.clone();
                let target_dir = self.target_dir.clone();
                let width = self.width;
                let height = self.height;
                let format = self.format;
                
                tokio::spawn(async move {
                    Self::process_path(path, watermark, target_dir, width, height, format).await
                })
            })
            .collect();
            
        for task in tasks {
            if let Err(e) = task.await? {
                eprintln!("Error processing image: {:#}", e);
            }
        }
        
        Ok(())
    }
    
    async fn process_path(
        path: PathBuf,
        watermark: DynamicImage,
        target_dir: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        format: Option<ImageFormat>,
    ) -> Result<()> {
        let metadata = fs::metadata(&path)
            .await
            .with_context(|| format!("Failed to read metadata for {}", path.display()))?;
            
        if metadata.is_file() {
            Self::process_image(path, watermark, target_dir, width, height, format).await
        } else if metadata.is_dir() {
            Self::process_directory(path, watermark, target_dir, width, height, format).await
        } else {
            anyhow::bail!("Path is neither file nor directory: {}", path.display())
        }
    }

    async fn process_directory(
        dir_path: PathBuf,
        watermark: DynamicImage,
        target_dir: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        format: Option<ImageFormat>,
    ) -> Result<()> {
        println!("Processing directory: {}", dir_path.display());
        
        let mut entries = fs::read_dir(&dir_path)
            .await
            .with_context(|| format!("Failed to read directory {}", dir_path.display()))?;
            
        let mut tasks = Vec::new();
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                let watermark = watermark.clone();
                let target_dir = target_dir.clone();
                
                let task = tokio::spawn(async move {
                    Self::process_image(path, watermark, target_dir, width, height, format).await
                });
                
                tasks.push(task);
            }
        }
        
        for task in tasks {
            if let Err(e) = task.await? {
                eprintln!("Error processing image in directory: {:#}", e);
            }
        }
        
        Ok(())
    }

    async fn process_image(
        image_path: PathBuf,
        watermark: DynamicImage,
        target_dir: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
        format: Option<ImageFormat>,
    ) -> Result<()> {
        let mut img = image::open(&image_path)
            .with_context(|| format!("Failed to open image {}", image_path.display()))?;
            
        // Resize if dimensions are specified
        if let (Some(w), Some(h)) = (width, height) {
            img = img.resize_exact(w, h, imageops::FilterType::Lanczos3);
        } else if let Some(w) = width {
            let aspect_ratio = img.height() as f32 / img.width() as f32;
            let h = (w as f32 * aspect_ratio) as u32;
            img = img.resize_exact(w, h, imageops::FilterType::Lanczos3);
        } else if let Some(h) = height {
            let aspect_ratio = img.width() as f32 / img.height() as f32;
            let w = (h as f32 * aspect_ratio) as u32;
            img = img.resize_exact(w, h, imageops::FilterType::Lanczos3);
        }
        
        // Apply watermark
        imageops::overlay(&mut img, &watermark, 0, 0);
        
        // Determine output format and path
        let output_format = format.unwrap_or_else(|| {
            Self::detect_format(&image_path).unwrap_or(ImageFormat::Png)
        });
        
        let file_stem = image_path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid filename")?;
            
        let extension = Self::format_to_extension(output_format);
        let output_path = target_dir.join(format!("{}.{}", file_stem, extension));
        
        img.save_with_format(&output_path, output_format)
            .with_context(|| format!("Failed to save watermarked image to {}", output_path.display()))?;
            
        println!("✓ Watermarked image saved to {}", output_path.display());
        Ok(())
    }
    
    fn detect_format(path: &Path) -> Option<ImageFormat> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext.to_lowercase().as_str() {
                "png" => Some(ImageFormat::Png),
                "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
                "webp" => Some(ImageFormat::WebP),
                "bmp" => Some(ImageFormat::Bmp),
                "tiff" | "tif" => Some(ImageFormat::Tiff),
                _ => None,
            })
    }
    
    fn format_to_extension(format: ImageFormat) -> &'static str {
        match format {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::WebP => "webp",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Tiff => "tiff",
            _ => "png",
        }
    }
}
