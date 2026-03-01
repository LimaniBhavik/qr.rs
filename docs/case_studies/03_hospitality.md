# Case Study: Hospitality & Restaurants

## The Challenge
Post-pandemic, restaurants prefer contactless menus to reduce printing costs and improve hygiene.

## The Solution: Table Menu QRs
Restaurants can generate specific QR codes for different table locations or menu types (Drinks, Dinner, Desserts). Using the `qr-gui`, a restaurant manager can easily create these without needing command-line knowledge, picking colors that match their interior decor.

### CLI Automation
For a chain, automating this per-store:
```bash
qr-cli "https://menu.example.com/store/42/table/12" --bg "#f8f9fa" -o table_12.png
```
