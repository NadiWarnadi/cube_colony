
## 1. Product Requirement Document (PRD)

### **Nama Projek:** Cube Colony (Tentatif)

### **Genre:** Minimalis Grid-Based Supply Chain & Defense Game (Inspirasi: Achikaps, WorldBox)

### **A. Core Gameplay (Mekanik Utama)**

* **Grid System:** Map menggunakan grid heksagon atau kotak 2D yang digambar secara prosedural.
* **Resource Management:** Ada 3 sumber daya utama: *Wood* (Kayu), *Food/Honey* (Makanan), dan *Stone* (Batu).
* **Cube Workers (Warga Kotak):** Unit otonom yang dikendalikan oleh AI (Rust). Mereka otomatis mencari tugas (menebang pohon, bertani, memindahkan barang ke gudang) berdasarkan prioritas.
* **Base Building:** Pemain bisa tap di area kosong untuk membangun:
* *Storage (Gudang):* Tempat mengumpulkan resource.
* *Barrack (Barak):* Untuk melatih warga biasa menjadi prajurit.
* *Turret (Menara Pertahanan):* Menembak musuh otomatis.


* **Threat (Ancaman):** Setiap beberapa menit (*wave*), musuh berbentuk kotak merah akan muncul dari pinggir map untuk menyerang gudang utama.

### **B. Pembagian Teknologi (Tech Stack)**

* **Frontend (Flutter + Flame Engine):**
* Menggambar semua bentuk geometri di layar menggunakan `CustomPainter`/Flame Canvas.
* Menangani *input touch* (membangun, memilih unit).
* UI Overlay (Tombol build, teks jumlah resource).


* **Backend Game Loop (Rust):**
* Menyimpan seluruh status (*state*) game: koordinat semua unit, isi gudang, darah bangunan.
* Menghitung *Pathfinding* (A* algoritma) agar warga kotak tidak tabrakan dan tahu jalan ke gudang.
* Menghitung logika *battle* ketika musuh datang.
