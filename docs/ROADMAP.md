# Yol Haritası

## v0.1.0 — Temel Ajan (Mevcut)

- [x] Needle v2 entegrasyonu
- [x] Qwen2.5-0.5B entegrasyonu
- [x] `web_search` aracı
- [x] `read_url` aracı
- [x] Otomatik model indirme
- [x] Retry mekanizması

## v0.2.0 — Genişletilmiş Araçlar

- [ ] `write_file(path, content)` — Dosyaya yaz
- [ ] `read_file(path)` — Dosyadan oku
- [ ] `list_dir(path)` — Dizin listele
- [ ] `run_command(cmd)` — Terminal komutu çalıştır
- [ ] Araç kayıt sistemi (plugin mimarisi)

## v0.3.0 — Çok Adımlı Görevler

- [ ] ReAct döngüsü (Reason + Act)
- [ ] Görev planlama (task decomposition)
- [ ] Araç zincirleme (chaining)
- [ ] Hata toleransı ve yeniden deneme

## v0.4.0 — Performans

- [ ] CUDA GPU desteği (RTX serisi)
- [ ] Metal GPU desteği (Apple Silicon)
- [ ] Vulkan GPU desteği (mobil)
- [ ] Model quantization seçenekleri (Q4, Q5, Q8)

## v0.5.0 — Mobil

- [ ] Android portu (JNI)
- [ ] iOS portu (Swift bindings)
- [ ] WASM derlemesi (tarayıcı)
- [ ] Hafif model seçenekleri

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