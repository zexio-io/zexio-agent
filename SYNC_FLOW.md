# Vectis Sync Flows

Berikut adalah rincian alur (flow) sinkronisasi antara **Dashboard** dan **Worker Node**.

## 1. Flow Registrasi Otomatis (Worker → Dashboard)
Alur ini terjadi saat pertama kali instalasi menggunakan `install.sh`.

```mermaid
sequenceDiagram
    participant W as Worker Node (Rust)
    participant D as Dashboard (NestJS)
    
    W->>W: Deteksi IP Publik (api.ipify.org)
    W->>W: Baca Secret (/etc/vectis/worker.secret)
    W->>D: POST /infra/workers/auto-register
    Note over W,D: Payload: { ip, secret }
    
    D->>D: Validasi Token & Simpan ke DB
    D->>D: Generate ID (random name + 4 hex)
    D->>D: Provisioning DNS (Subdomain)
    
    D-->>W: Response: { id, subdomain, ip }
    W->>W: Simpan Konfigurasi ke /etc/vectis/worker.conf
    W->>W: Set SYNC_STATUS = synced
```

---

## 2. Flow Sinkronisasi Manual (Dashboard → Worker)
Alur ini terjadi saat User menekan tombol "Sync" di Dashboard atau saat Dashboard melakukan pengecekan kesehatan berkala.

```mermaid
sequenceDiagram
    participant D as Dashboard (NestJS)
    participant W as Worker Node (Rust)
    
    D->>D: Ambil IP & Secret Worker dari DB
    D->>D: Generate HMAC Signature (Worker Auth)
    
    D->>W: POST /sync (Port 3000)
    Note over D,W: Auth: Bearer <HMAC_SIGNATURE>
    
    W->>W: Validasi Signature menggunakan Secret Lokal
    W->>W: Kumpulkan System Stats & Metadata Node
    
    W-->>D: Response: { status: "online", version: "x.y.z", stats: {...} }
    
    D->>D: Update Last Sync & Status di Dashboard
```

## Status Sinkronisasi

1.  **`pending`**: Worker sudah terinstall tapi belum berhasil registrasi ke Dashboard (biasanya karena `VECTIS_TOKEN` kosong).
2.  **`synced`**: Worker dan Dashboard sudah terhubung dan data sinkron.
3.  **`failed`**: Upaya sinkronisasi terakhir gagal (misal: Worker mati atau IP berubah).
4.  **`outdated`**: Ada perubahan konfigurasi di Dashboard yang belum diterapkan ke Worker.

## Perintah Penting di Worker Node

-   `vectis-status`: Melihat status sinkronisasi saat ini.
-   `vectis-sync`: Melakukan pendaftaran/update manual ke Dashboard.
