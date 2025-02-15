# Watermarker

Watermarker is a CLI tool for adding watermark to images.

## Usage:
```
watermarker [--path <target_path>] [--res <width> <height>] [--filetype <filetype>] <watermark_path> <image_path> [image_path]...
```

## Arguments:
```
--path <target_path>         (Optional) Target directory to save watermarked images. Defaults to './output'.
--res <width> <height>       (Optional) Target resolution for the watermarked images. reserves the aspect ratio. Defaults to the original resolution.
--filetype <filetype>        (Optional) Target file type for the watermarked images. Defaults to the original file type.
<watermark_path>             Path to the watermark image.
<image_path>                 Path(s) to the image(s) or folders that contain image(s) to be watermarked.
```

## Examples:
```bash
watermarker watermark.png image1.jpg
watermarker watermark.png image1.jpg image2.png
watermarker watermark.png /home/user/images/
watermarker --path ./target --res 300 500 --filetype jpg watermark.png image1.png image2.png image3.png
```

### Available on AUR:
```
paru -S watermarker
```