# Case Study: IT & Networking

## The Challenge
Offices and cafes want to allow guests to join their WiFi network without having to read out or type complex passwords.

## The Solution: WiFi QRs
`qr.rs` natively supports the WiFi format, creating codes that modern smartphones instantly recognize to connect to the network.

### CLI Example
While the CLI currently defaults to text mode for raw strings, the library supports this. A custom CLI tool can easily wrap this:
```bash
# Concept via script
qr-cli "WIFI:T:WPA;S:GuestNetwork;P:supersecret;;" -o guest_wifi.png
```
