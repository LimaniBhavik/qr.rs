# Case Study: Healthcare

## The Challenge
Medical staff need quick access to a patient's emergency medical record or contact information from a physical ID bracelet.

## The Solution: vCard QRs
Using the VCard generation feature, hospitals can print bracelets containing a vCard QR. When scanned by an EMT, it immediately adds the emergency contact or primary care physician to their phone.

### CLI Example
```bash
qr-cli contact --first-name "John" --last-name "Doe" --phone "+15559110000" --organization "Mercy Hospital" -o patient_id.png
```
