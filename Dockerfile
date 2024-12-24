# Menggunakan image Rust sebagai base image untuk build
FROM rust:1.83.0 as builder

# Set direktori kerja di dalam container
WORKDIR /usr/src/app

# Menyalin Cargo.toml dan Cargo.lock untuk caching dependensi
COPY Cargo.toml Cargo.lock ./

# Menyalin semua kode sumber
COPY . .

# Build aplikasi dalam mode release
RUN cargo build --release

# Menggunakan image minimal dengan GLIBC yang lebih baru
FROM debian:bookworm-slim

# Menginstal library runtime yang diperlukan
RUN apt-get update && apt-get install -y libpq-dev && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set direktori kerja di dalam container
WORKDIR /app

# Menyalin binary aplikasi dari tahap builder
COPY --from=builder /usr/src/app/target/release/pictoria_api /usr/local/bin/pictoria_api

# Menyalin file konfigurasi seperti .env dan diesel.toml
COPY .env ./ 
COPY diesel.toml ./ 

# Mengekspos port yang digunakan aplikasi
EXPOSE 8000

# Menjalankan aplikasi
CMD ["pictoria_api"]
