#!/bin/bash
set -e

# Menunggu database siap
until diesel setup; do
  echo "Waiting for database to be ready..."
  sleep 2
done

# Menjalankan migrasi
diesel migration run

# Menjalankan aplikasi
exec yumana_api