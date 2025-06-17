# darktable-cli Reference

## Overview
- Command-line interface for darktable RAW processor
- Specifically designed for RAW formats like DNG
- Better color management than ImageMagick for RAW files
- No GUI required - pure console mode

## Basic Usage

### Simple Conversion
```bash
darktable-cli input.dng output.webp
```

### With XMP Sidecar
```bash
darktable-cli input.dng input.xmp output.webp
```

## Command Structure
```bash
darktable-cli [options] <input> [<xmp>] <output>
```

## Key Options

### Format Control
- `--out-ext <ext>` - Force output format (webp, jpg, png, etc.)
- Output format auto-detected from filename extension

### Size Control
- `--width <width>` - Max output width
- `--height <height>` - Max output height
- `--hq true` - High quality resampling (default)

### Processing Options
- `--apply-custom-presets false` - Skip custom presets
- `--style <style>` - Apply darktable style
- `--style-overwrite` - Let style overwrite existing

### Performance
- `--core` - Use specific darktable core options
- `--conf <key>=<value>` - Set configuration options

## WebP Specific Configuration

### Quality Settings
```bash
darktable-cli input.dng output.webp \
  --core \
  --conf plugins/imageio/format/webp/quality=100
```

### WebP Options via --conf
- `plugins/imageio/format/webp/quality=<1-100>`
- `plugins/imageio/format/webp/compression=<0-6>`
- `plugins/imageio/format/webp/hint=<default|picture|photo|graph>`

## Color Management

### ICC Profile Handling
```bash
# Embed specific output profile
darktable-cli input.dng output.webp \
  --core \
  --conf plugins/imageio/format/webp/icc_profile=embedded
```

### Common Color Spaces
- sRGB (default for web)
- Adobe RGB
- ProPhoto RGB
- Linear RGB

## Metadata Handling

### Known Issues
- WebP export may not preserve all EXIF/XMP metadata
- ICC profiles might not be embedded properly
- Consider post-processing with exiftool if needed

## Advanced Usage

### Batch Processing
```bash
# Process folder of DNGs
darktable-cli /path/to/dngs/ /path/to/output/ --out-ext webp
```

### Using Styles
```bash
# Apply custom processing style
darktable-cli input.dng output.webp \
  --style "my_style" \
  --style-overwrite
```

### Debug Output
```bash
# Verbose output for troubleshooting
darktable-cli input.dng output.webp \
  --core --configdir /tmp/dt \
  -d all
```

## Performance Considerations

### OpenCL Acceleration
- Automatically uses OpenCL if available
- Disable with: `--core --conf opencl=false`

### Memory Usage
- Large DNG files use significant RAM
- Tiling used automatically for huge images

## Exit Codes
- 0: Success
- 1: General error
- 2: Missing input file
- Other: Various processing errors

## Advantages Over ImageMagick

1. **Better RAW Processing**
   - Proper demosaicing algorithms
   - Advanced noise reduction
   - Highlight recovery

2. **Color Accuracy**
   - Full color management pipeline
   - Camera-specific color matrices
   - Proper white balance

3. **Non-Destructive**
   - Applies XMP edits if present
   - Preserves RAW data integrity

## Limitations
- Slower than ImageMagick
- Larger memory footprint
- WebP metadata support incomplete
- May require GUI for initial preset creation