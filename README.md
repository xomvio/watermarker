# Watermarker

Lightweight and efficient CLI tool designed to add watermarks to images. Especially designed for batch-editing multiple files but also works well if you processing single file.

## Key Features:
- Custom Output Directory: Save watermarked images to your preferred folder.
- Flexible Resolution: Resize images while maintaining their aspect ratio.
- File Type Conversion: Output images in a format of your choice (e.g., .jpg, .png).
- Batch Processing: Apply watermarks to multiple images in a single command.

## Usage:
```bash
watermarker [--path <target_path>] [--res <width> <height>] [--filetype <filetype>] <watermark_path> <image_path> [image_path]...
```

## Arguments:
```
--path <target_path>
(Optional) Specifies the target directory to save the watermarked images. Defaults to ./output.

--res <width> <height>
(Optional) Sets the resolution for the output images while preserving their aspect ratio. Defaults to the original resolution.

--filetype <filetype>
(Optional) Specifies the file type for the output images (e.g., jpg, png). Defaults to the original file type.

<watermark_path>
The file path to the watermark image.

<image_path>
One or more file paths to the images to be watermarked.
```

## Examples
Add watermark to single image:
```bash
watermarker watermark.png image1.jpg
```
Add watermark to multiple images:
```bash
watermarker watermark.png image1.jpg image2.png
```
With custom output directory, resolution, and file type:
```bash
watermarker --path ./target --res 300 500 --filetype jpg watermark.png image1.png image2.png image3.png
```

## Installation
Arch Linux (AUR):
```bash
paru -S watermarker
```
Other:<br />
You can compile yourself or directly download bin or exe from releases.

## Contributing

Contributions are welcome! Feel free to fork the repository, submit issues, or create pull requests to improve Watermarker.
