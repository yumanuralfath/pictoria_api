# Pictoria API

Pictoria API adalah aplikasi web yang dibangun menggunakan Rust dan framework Rocket. Proyek ini bertujuan untuk menyediakan platform interaktif untuk berbagi dan berkolaborasi dalam gambar.

## Fitur

- **RESTful API**: Menggunakan Rocket untuk membangun API yang cepat dan efisien.
- **Database**: Menggunakan Diesel untuk interaksi dengan database PostgreSQL.
- **Asynchronous**: Memanfaatkan Tokio untuk operasi non-blok.

## Prerequisites

Sebelum memulai, pastikan Anda telah menginstal:

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Instalasi

1. Clone repositori ini:
   ```bash
   git clone https://github.com/yumanuralfath/pictoria_api
   cd pictoria_api
   ```

2. Install dependensi:
   ```bash
   cargo build
   ```

3. Buat file `.env` di root proyek dan tambahkan konfigurasi database Anda:
   ```env
   DATABASE_URL=postgres://username:password@localhost/pictoria_api
   ```

## Menjalankan Aplikasi

Untuk menjalankan aplikasi, gunakan perintah berikut:
```
cargo run
```
Aplikasi akan berjalan di `http://localhost:8000`.