# Case Study: Logistics & Shipping

## The Challenge
Warehouses need to track packages rapidly. Standard barcodes are sometimes hard to read if damaged.

## The Solution: Package Tracking QRs
QR codes with High error correction (`-l high`) ensure that even if the shipping label is torn or scuffed, the tracking ID is readable.

### CLI Example
```bash
qr-cli "TRACKING:PKG-987654321" -l high -o label_987654.png
```
