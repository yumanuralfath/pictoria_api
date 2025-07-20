# 🌐 Yumana API

API backend untuk aplikasi Todo dan Manajemen Akun, dibangun dengan **Rust + Rocket + Diesel** dan menggunakan PostgreSQL sebagai database utama. Mendukung autentikasi JWT dan integrasi Cloudinary untuk upload gambar profil.

> 🌍 Production URL: **https://api.yumana.my.id**

---

## 🚀 Tech Stack

- **Rust**
- **Rocket** (web framework)
- **Diesel** (ORM)
- **PostgreSQL**
- **JWT** (untuk autentikasi)
- **Cloudinary** (upload image profil)

---

## 🛠️ Cara Menjalankan Secara Lokal

### 1. Clone repository
```bash
git clone https://github.com/username/yumana_api.git
cd yumana_api
```

### 2. Buat file .env
Salin dari env.example:

```bash
cp .env.example .env
```

### 3. Isi variabel di .env:

```
DATABASE_URL=postgres://username:password@localhost:5432/yumana_db
CLOUDINARY_API_KEY=your_api_key
CLOUDINARY_API_SECRET=your_api_secret
CLOUDINARY_CLOUD_NAME=your_cloud_name
JWT_SECRET=your_jwt_secret
```

### 4. Setup database
```
diesel setup
diesel migration run
```
### 5. Jalankan server Rocket
```
cargo run
```

📂 Struktur Direktori (opsional)
```
src/
├── controllers/        # Logika handler endpoint
├── models/             # Model Diesel & Schema
├── routes/             # Route groupings
├── utils/              # Fungsi utilitas (auth, db, dll)
├── main.rs             # Entry point Rocket
└── ...
```
### ⚙️ Environment Variables

Semua variabel penting tersedia di file .env.example:

    DATABASE_URL — Koneksi ke PostgreSQL

    JWT_SECRET — Secret untuk generate JWT token

    CLOUDINARY_API_KEY, CLOUDINARY_API_SECRET, CLOUDINARY_CLOUD_NAME — untuk upload foto