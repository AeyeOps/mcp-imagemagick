# Known Issues

## ImageMagick DNG Support

### Issue
ImageMagick 7 (`convert7`) may fail to process DNG files with the following error:
```
magick: no images for write '-write' '/path/to/output.webp' at CLI arg 4 @ error/operation.c/CLINoImageOperator/4985.
```

### Cause
This appears to be related to ImageMagick's DNG format delegate configuration or missing dependencies for RAW format support.

### Workaround
Use the `darktable` converter instead:
```json
{
  "tool": "convert_dng_to_webp",
  "arguments": {
    "input_path": "/path/to/image.dng",
    "output_path": "/path/to/output.webp",
    "converter": "darktable"
  }
}
```

### Recommendation
For DNG files, we recommend using darktable as it:
- Has native RAW format support
- Provides better color management
- Handles camera-specific color profiles correctly

## Auto Converter Selection

The auto converter currently prioritizes ImageMagick over darktable for performance reasons. However, if you're working primarily with DNG files, you may want to explicitly specify `"converter": "darktable"` in your requests.