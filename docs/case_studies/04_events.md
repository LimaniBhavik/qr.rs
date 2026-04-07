# Case Study: Events & Ticketing

## The Challenge
An event organizer needs to issue thousands of secure, unique tickets. Each ticket needs a scannable QR code containing a cryptographically signed payload to prevent forgery.

## The Solution: High-Throughput Batch Generation
Using the `qr-scan-rs` core library, the ticketing backend can generate tens of thousands of QR codes per second concurrently.

### Core Library Approach
See `examples/events_ticketing.rs` for a simulation of generating batch tickets with unique identifiers.
