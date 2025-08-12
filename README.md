# 🖼️ Watermarker

CLI tool for adding watermarks to images, built with Rust.

## ✨ Features

- **Concurrent Processing**: Leverages async/await for batch processing
- **Smart Resizing**: Preserves aspect ratios with high-quality Lanczos3 filtering
- **Format Support**: PNG, JPEG, WebP, BMP, and TIFF formats
- **Directory Processing**: Recursively processes entire image directories
- **Robust Error Handling**: Graceful error recovery with detailed context

## 🚀 Installation

### From AUR (Arch Linux)
```bash
paru -S watermarker
```

### From Source
```bash
git clone https://github.com/xomvio/watermarker
cd watermarker
cargo build --release
```

## 📖 Usage

```bash
watermarker [OPTIONS] <WATERMARK_PATH> [IMAGE_PATHS]...
```

### Arguments
- `<WATERMARK_PATH>` - Path to the watermark image
- `[IMAGE_PATHS]...` - Path(s) to image files or directories to be watermarked

### Options
- `-t, --target-path <PATH>` - Output directory (default: `./output`)
- `--width <WIDTH>` - Target width in pixels (preserves aspect ratio)
- `--height <HEIGHT>` - Target height in pixels (preserves aspect ratio)
- `-f, --format <FORMAT>` - Output format: `png`, `jpg`, `webp`, `bmp`, `tiff`
- `-h, --help` - Show help information
- `-V, --version` - Show version information

## 📝 Examples

### Basic Usage
```bash
# Watermark a single image
watermarker watermark.png photo.jpg

# Watermark multiple images
watermarker watermark.png photo1.jpg photo2.png photo3.webp
```

### Advanced Usage
```bash
# Process entire directory with custom output location
watermarker -t ./watermarked watermark.png ./photos/

# Resize and convert format while watermarking
watermarker --width 1920 --format png watermark.png image.jpg

# Batch process with specific dimensions
watermarker -t ./output --width 800 --height 600 watermark.png ./images/
```

## 🎯 Supported Formats

| Format | Input | Output | Extension |
|--------|-------|--------|-----------|
| PNG    | ✅    | ✅     | `.png`    |
| JPEG   | ✅    | ✅     | `.jpg`, `.jpeg` |
| WebP   | ✅    | ✅     | `.webp`   |
| BMP    | ✅    | ✅     | `.bmp`    |
| TIFF   | ✅    | ✅     | `.tiff`, `.tif` |

## ⚡ Performance

- **Concurrent Processing**: Processes multiple images simultaneously
- **High-Quality Scaling**: Uses Lanczos3 algorithm for high image quality
- **Async I/O**: Non-blocking file operations for maximum throughput

## 📄 License

This project is licensed under the GPL-3.0-or-later License.
