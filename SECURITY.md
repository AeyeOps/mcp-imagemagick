# Security Policy

## Overview

The mcp-imagemagick server processes image files using ImageMagick and darktable. As with any system that processes user-provided files, there are inherent security considerations to be aware of.

## Inherited Security Risks

### ImageMagick Security Concerns

ImageMagick is a powerful image processing tool that supports numerous file formats. Some of these formats can contain embedded code or references to external resources, which historically have been exploited for:

1. **Remote Code Execution (RCE)** - Malicious files could execute arbitrary commands
2. **Server-Side Request Forgery (SSRF)** - Files could trigger requests to internal resources
3. **Denial of Service (DoS)** - Crafted images could consume excessive resources
4. **Information Disclosure** - Specially crafted files could read arbitrary files

The most notable example was "ImageTragick" (CVE-2016-3714), where ImageMagick's processing of MVG and SVG files could be exploited for remote code execution.

### Our Specific Context

While mcp-imagemagick currently focuses on DNG to WebP conversion (lower risk formats), we still inherit ImageMagick's attack surface because:

- ImageMagick performs file type detection by content, not extension
- A malicious file disguised as a DNG could potentially be processed as a different format
- We execute ImageMagick with the same permissions as the MCP server

## Security Recommendations

### 1. Configure ImageMagick Security Policy

Ensure your system's ImageMagick has a properly configured `policy.xml` file. Common locations:
- `/etc/ImageMagick-7/policy.xml`
- `/etc/ImageMagick-6/policy.xml`
- `/usr/local/etc/ImageMagick-7/policy.xml`

Recommended restrictions for web-facing services:

```xml
<policymap>
  <!-- Disable vulnerable coders -->
  <policy domain="coder" rights="none" pattern="EPHEMERAL" />
  <policy domain="coder" rights="none" pattern="URL" />
  <policy domain="coder" rights="none" pattern="HTTPS" />
  <policy domain="coder" rights="none" pattern="HTTP" />
  <policy domain="coder" rights="none" pattern="FTP" />
  <policy domain="coder" rights="none" pattern="MVG" />
  <policy domain="coder" rights="none" pattern="MSL" />
  <policy domain="coder" rights="none" pattern="TEXT" />
  <policy domain="coder" rights="none" pattern="LABEL" />
  
  <!-- Allow only specific safe formats -->
  <policy domain="coder" rights="read|write" pattern="PNG" />
  <policy domain="coder" rights="read|write" pattern="JPEG" />
  <policy domain="coder" rights="read|write" pattern="WEBP" />
  <policy domain="coder" rights="read" pattern="DNG" />
  
  <!-- Set resource limits -->
  <policy domain="resource" name="memory" value="1GB"/>
  <policy domain="resource" name="map" value="2GB"/>
  <policy domain="resource" name="width" value="16KP"/>
  <policy domain="resource" name="height" value="16KP"/>
  <policy domain="resource" name="time" value="120"/>
</policymap>
```

### 2. Run with Limited Permissions

Consider running the MCP server:
- As a dedicated user with minimal privileges
- In a containerized environment (Docker, Podman)
- With filesystem restrictions (only access to necessary directories)
- Behind appropriate network isolation

### 3. Input Validation

While our server performs basic validation, consider additional measures:
- Verify file extensions match content
- Limit file sizes before processing
- Scan files with antivirus if processing untrusted sources
- Implement rate limiting to prevent DoS

### 4. Keep Software Updated

Regularly update:
- ImageMagick to the latest version
- darktable to the latest version
- This MCP server
- System packages and dependencies

Check for updates:
```bash
# Check ImageMagick version
convert -version

# Check for known vulnerabilities
# Visit: https://cve.mitre.org/cgi-bin/cvekey.cgi?keyword=imagemagick
```

### 5. Monitoring and Logging

Monitor for suspicious activity:
- Unusual resource consumption during conversions
- Failed conversion attempts with strange errors
- Access to unexpected file paths

The MCP server logs to stderr, so ensure these logs are captured and reviewed.

## Deployment Recommendations

### Development Environment
- Use default ImageMagick policies
- Process only your own test files
- Run with your user permissions

### Production Environment
- Implement strict ImageMagick policy.xml
- Run in isolated container/VM
- Use dedicated service account
- Process files from trusted sources only
- Implement proper authentication on MCP client

### Public-Facing Service
- All production recommendations plus:
- Run behind a reverse proxy
- Implement file upload scanning
- Use temporary directories with automatic cleanup
- Consider using cloud-based image processing services instead

## Security Resources

- [ImageMagick Security Policy](https://imagemagick.org/script/security-policy.php)
- [ImageMagick Security Policy Evaluator](https://imagemagick-secevaluator.doyensec.com/)
- [ImageTragick Vulnerability Info](https://imagetragick.com/)
- [CVE Database - ImageMagick](https://cve.mitre.org/cgi-bin/cvekey.cgi?keyword=imagemagick)

## Reporting Security Issues

If you discover a security vulnerability in mcp-imagemagick, please:

1. **Do not** create a public GitHub issue
2. Send details to the repository maintainers
3. Allow reasonable time for a fix before public disclosure

## Disclaimer

This software is provided "as is" without warranty. Users are responsible for evaluating security risks in their specific deployment context. Processing untrusted image files always carries inherent risks.