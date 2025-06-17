# ImageMagick 7 CLI Reference

## Command Access
- Available as `convert7` (not standard `convert`)
- Alternative syntax: `magick` (IM7 unified command)

## DNG to WebP Conversion

### Basic Lossless Conversion
```bash
convert7 input.dng -define webp:lossless=true output.webp
```

### Maximum Quality Lossless
```bash
convert7 input.dng \
  -define webp:lossless=true \
  -define webp:exact=true \
  -define webp:method=6 \
  -define webp:partition-limit=0 \
  output.webp
```

## WebP Options

### Core Options
- `webp:lossless=true` - Enable lossless compression
- `webp:exact=true` - Preserve exact pixel values
- `webp:method=6` - Maximum compression (0-6, slower but smaller)
- `webp:quality=100` - Quality level (ignored in lossless mode)

### Advanced Options
- `webp:partition-limit=0` - Disable partition limit
- `webp:alpha-quality=100` - Alpha channel quality
- `webp:alpha-compression=1` - Alpha compression
- `webp:target-size=<bytes>` - Target file size
- `webp:thread-level=1` - Enable threading

## DNG Handling

### ColorSpace Considerations
- DNG files contain RAW sensor data
- May need color profile conversion
- Default sRGB conversion might occur

### Preserve Original Data
```bash
convert7 input.dng \
  -colorspace RGB \
  -define webp:lossless=true \
  output.webp
```

## Error Handling

### Common Issues
1. **Format Not Supported**
   - Check IM delegates: `convert7 -list format | grep -E 'DNG|WebP'`
   
2. **Memory Limits**
   - Large DNG files may hit limits
   - Adjust with: `-limit memory 2GB -limit disk 4GB`

3. **Color Shifts**
   - Use `-colorspace` carefully
   - Consider `-profile` for ICC profiles

## Performance Tips

### Speed Optimization
```bash
convert7 input.dng \
  -define webp:lossless=true \
  -define webp:method=0 \  # Fastest
  output.webp
```

### Memory Optimization
```bash
convert7 input.dng \
  -limit memory 512MB \
  -limit map 1GB \
  -define webp:lossless=true \
  output.webp
```

## Validation Commands

### Check Supported Formats
```bash
convert7 -list format
```

### Verify Installation
```bash
convert7 -version
```

### Get WebP Delegate Info
```bash
convert7 -list delegate | grep webp
```

## Batch Processing
```bash
# Convert all DNG files in directory
for f in *.dng; do
  convert7 "$f" -define webp:lossless=true "${f%.dng}.webp"
done
```

## Exit Codes
- 0: Success
- 1: General error
- 2: Missing/invalid arguments
- Other: Specific errors (check stderr)