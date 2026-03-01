# Case Study: Education

## The Challenge
Teachers want to share digital resources (Google Drive folders, YouTube videos, interactive quizzes) with students easily during a live lecture.

## The Solution: Web/GUI Generation
Using the `qr-web` WASM application, a teacher can instantly paste a link on their smartboard and display a QR code for the entire classroom to scan from their seats.

### CLI Example
```bash
qr-cli "https://classroom.google.com/folder-link" -s 50 -o smartboard_qr.png
```
