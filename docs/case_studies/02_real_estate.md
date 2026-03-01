# Case Study: Real Estate

## The Challenge
Real estate agents place physical "For Sale" signs outside properties. Passersby often want immediate access to listing details, interior photos, and the agent's contact info without typing a long URL.

## The Solution: Dynamic Property Listing QRs
By printing a QR code on the yard sign that points to a dynamic URL (e.g., `https://agency.com/property/123`), the agency can update the listing status (Active, Pending, Sold) without reprinting the physical sign.

### CLI Example
```bash
qr-cli "https://agency.com/property/12345" --fg "#1a365d" -o listing_12345.png
```
