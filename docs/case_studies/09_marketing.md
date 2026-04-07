# Case Study: Marketing

## The Challenge
Marketers run ad campaigns on billboards and transit stops. They need beautiful, branded QR codes (custom colors, embedded logos) to track physical engagement.

## The Solution: Branded QRs
`qr-scan-rs` supports custom foreground and background colors, and the core library/CLI support logo embedding.

### CLI Example
```bash
# Assuming an upcoming CLI feature or using library
qr-cli "https://campaign.com/summer" --fg "#ff6600" --bg "#ffffff" -l high -o poster.png
```
