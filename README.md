
# Aplikasi Chat Broadcast Asinkron (Modul 10)

Aplikasi ini merupakan sistem obrolan berbasis protokol WebSocket yang dibangun menggunakan ekosistem asinkron `tokio` dan `tokio_websockets` di Rust. Sistem ini mengimplementasikan server pusat yang mendengarkan koneksi masuk dan mendistribusikan pesan secara real-time ke seluruh klien yang terhubung menggunakan saluran *broadcast*.

### Cara Menjalankan Aplikasi

1. **Jalankan Server:**
   Buka terminal utama Anda, pastikan berada di dalam direktori `chat-async`, lalu jalankan server terlebih dahulu:
   ```bash
   cargo run --bin server
   ```

2. **Jalankan Client:**
    Buka tiga jendela terminal baru secara terpisah, lalu jalankan perintah berikut pada masing-masing terminal untuk membuka tiga klien:
    ```bash
    cargo run --bin client
    ```

Ketika salah satu klien mengetik sebuah pesan di terminalnya lalu menekan tombol Enter, fungsi asinkron tokio::select! pada klien akan langsung menangkap baris teks tersebut melalui stdin. Pesan ini kemudian dibungkus menjadi frame WebSocket dan dikirimkan menuju server pusat.

Seketika setelah server menerima pesan tersebut, server akan memformat teks dengan menyertakan alamat IP asal (SocketAddr) dan menyebarkannya (broadcast) ke seluruh instans klien yang sedang aktif. Berdasarkan modifikasi opsional yang telah diimplementasikan, server secara cerdas akan mengecek identitas pengirim terlebih dahulu; pesan hanya akan diteruskan ke terminal klien-klien lain dan tidak akan dipantulkan kembali ke terminal pengirim asli demi menjaga kebersihan log obrolan.

![Server](screenshots/2_1_server.png)
![Client 1](screenshots/2_1_client1.png)
![Client 2](screenshots/2_1_client2.png)
![Client 3](screenshots/2_1_client3.png)

---

### Eksperimen 2.2: Modifikasi Port WebSocket

Untuk mengubah port aplikasi menjadi `8080`, perubahan harus dilakukan pada kedua sisi sistem yang saling terhubung:
1. **Sisi Server (`src/bin/server.rs`):** Mengubah parameter alamat pengikatan (*binding*) pada fungsi `TcpListener::bind` dari `"127.0.0.1:2000"` menjadi `"127.0.0.1:8080"`. Modifikasi ini menginstruksikan server untuk membuka soket TCP baru dan mendengarkan lalu lintas masuk pada port 8080.
2. **Sisi Client (`src/bin/client.rs`):** Mengubah string URI target pada fungsi `ClientBuilder::from_uri` dari `"ws://127.0.0.1:2000"` menjadi `"ws://127.0.0.1:8080"`. Hal ini memastikan klien mengarahkan jabat tangan (*handshake*) koneksinya ke port server yang baru.

Kedua file tersebut **menggunakan protokol WebSocket yang sama**. Skema protokol ini didefinisikan secara eksplisit di dalam kode sumber klien saat melakukan inisialisasi koneksi melalui `ClientBuilder`, yang ditandai dengan penggunaan prefix `ws://` (WebSocket) pada string URI statis yang dimasukkan. Di sisi server, protokol ini didefinisikan saat membungkus aliran data TCP mentah menggunakan pustaka `ServerBuilder::new().accept(socket).await` untuk mengekstrak dan memproses bingkai (*frame*) WebSocket.

![Server Port 8080](screenshots/2_2_server.png)
![Client 1](screenshots/2_2_client1.png)
![Client 2](screenshots/2_2_client2.png)
![Client 3](screenshots/2_2_client3.png)


### Eksperimen 2.3: Perubahan Kecil (Menambahkan IP dan Port)

Pada eksperimen ini, modifikasi dilakukan untuk menyertakan informasi identitas pengirim berupa alamat IP dan nomor port asal pada setiap pesan yang disiarkan (*broadcast*). Hal ini bertujuan agar setiap klien dapat mengetahui dengan jelas dari mana pesan tersebut berasal tanpa perlu sistem registrasi nama pengguna (*username*) yang kompleks.

#### Alur Pengiriman Pesan dan Alasan Perubahan:
1. **Penerimaan di Server:** Ketika fungsi `ws_stream.next()` di sisi server menerima sebuah pesan dari klien, server juga memegang variabel `addr` yang bertipe `SocketAddr`. Variabel ini secara otomatis merekam alamat IP lokal beserta port dinamis yang dialokasikan oleh sistem operasi untuk koneksi klien tersebut.
2. **Formatisasi String (`String Formatting`):** Alasan perubahan ditempatkan di sisi server (menggunakan makro `format!("{}: {}", addr, text)`) adalah demi efisiensi dan keamanan. Daripada meminta setiap klien mengirimkan IP mereka sendiri (yang bisa saja dimanipulasi atau dipalsukan), server bertindak sebagai otoritas tunggal yang valid untuk menempelkan label identitas `IP:Port` tepat di depan teks pesan asli.
3. **Penyebaran Data:** Pesan yang telah diformat menjadi `"127.0.0.1:[Port]: Pesan"` inilah yang kemudian dikirimkan ke dalam saluran `bcast_tx.send()`, sehingga saat diterima oleh klien-klien lain, informasi pengirim langsung tercetak secara otomatis di layar terminal mereka.

![Hasil Penambahan IP dan Port](screenshots/2_3_server.png)