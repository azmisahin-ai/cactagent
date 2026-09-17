# Yol Haritası

## v0.1.0 — Temel Ajan (Tamamlandı)

- [x] Needle v2 entegrasyonu
- [x] `web_search` aracı
- [x] `read_url` aracı (readability ile)
- [x] Otomatik model indirme
- [x] Retry mekanizması

## v0.2.0 — Sadeleştirme (Mevcut)

- [x] Qwen2.5-0.5B bağımlılığını kaldırma
- [x] Binary boyutunu ~10 MB'a düşürme
- [x] RAM kullanımını ~50 MB'a düşürme
- [x] Odak: sadece Needle + araçlar
- [ ] README ve dokümantasyon güncellemesi

## v0.3.0 — Sağlamlaştırma

- [ ] CI + test altyapısı (GitHub Actions)
- [ ] `cargo test` için temel testler
- [ ] `cargo clippy` entegrasyonu
- [ ] Hata yönetimi iyileştirmeleri
- [ ] `examples/` klasörü

## v0.4.0 — Güvenli Dosya Araçları

- [ ] `read_file(path)` — sandbox'lı
- [ ] `write_file(path, content)` — sandbox'lı
- [ ] `list_dir(path)` — sandbox'lı
- [ ] Path traversal koruması
- [ ] `./workspace/` dizini sınırlaması

## v0.5.0 — Çok Adımlı Görevler

- [ ] ReAct döngüsü (Reason + Act)
- [ ] Görev planlama (task decomposition)
- [ ] Araç zincirleme (chaining)
- [ ] Hata toleransı ve yeniden deneme

## v0.6.0 — E2 Micro / Mobil Optimizasyonu

- [ ] Bellek kullanımı ölçümü
- [ ] Binary boyutu minimizasyonu
- [ ] Cross-compilation (ARM)
- [ ] Raspberry Pi testi
- [ ] Android portu (JNI)
- [ ] iOS portu (Swift bindings)

## v0.7.0 — Opsiyonel: Özet Modeli

- [ ] Daha küçük özet modeli araştırması (100-200 MB)
- [ ] `feature = "summarize"` flag'i arkasında
- [ ] Qwen2.5-0.5B'yi daha iyi prompt ile tekrar dene
- [ ] Kullanıcı onayı ile opsiyonel çalıştırma

## v1.0.0 — Olgun Sürüm

- [ ] Kendi kendine geliştirme modu
- [ ] Eklenti sistemi (harici araçlar)
- [ ] Web UI (opsiyonel)
- [ ] Kapsamlı test kapsamı
- [ ] Kararlı API

## Uzun Vadeli Vizyon

- **Kendi kendine öğrenen ajan:** Kullanıcı geri bildirimi ile araç seçimini iyileştirme
- **Çoklu ajan:** Birden fazla CactAgent örneğinin işbirliği
- **Yerel RAG:** Kendi dokümanlarınızı vektör veritabanında saklayıp sorgulama
- **Ses desteği:** STT + TTS ile tam sesli asistan
