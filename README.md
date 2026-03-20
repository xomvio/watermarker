# watermarker

A fast, minimal CLI tool for batch image watermarking. written in Rust.

## Features

- Concurrent processing via async/await — handles large batches efficiently
- Recursive directory traversal
- Output resizing with aspect-ratio preservation (Lanczos3)
- Format conversion on output: PNG, JPEG, WebP, BMP, TIFF
- Graceful error recovery. one failed image doesn't abort the batch

## Installation

**AUR (Arch Linux)**
```sh
paru -S watermarker
```

**From source**
```sh
git clone https://github.com/xomvio/watermarker
cd watermarker
cargo build --release
```

## Usage

```
watermarker [OPTIONS] <WATERMARK_PATH> [IMAGE_PATHS]...
```

| Argument / Option | Description |
|---|---|
| `<WATERMARK_PATH>` | Path to the watermark image |
| `[IMAGE_PATHS]...` | Image files or directories to process |
| `-t, --target-path <PATH>` | Output directory (default: `./output`) |
| `--width <WIDTH>` | Resize output to this width (preserves aspect ratio) |
| `--height <HEIGHT>` | Resize output to this height (preserves aspect ratio) |
| `-f, --format <FORMAT>` | Output format: `png` `jpg` `webp` `bmp` `tiff` |

## Examples

```sh
# Single image
watermarker watermark.png photo.jpg

# Entire directory, custom output path
watermarker -t ./watermarked watermark.png ./photos/

# Resize and convert format
watermarker --width 1920 --format webp watermark.png ./photos/
```

## License

GPL-3.0-or-later
