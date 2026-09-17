# Katkı Rehberi

CactAgent'a katkıda bulunmak istediğiniz için teşekkürler!

## Geliştirme Ortamı

1. Rust 1.70+ kurun
2. Repoyu klonlayın
3. `cargo build` ile derleyin
4. `cargo test` ile testleri çalıştırın

## Kod Standartları

- `cargo fmt` ile formatlayın
- `cargo clippy` ile lint kontrolü yapın
- Yeni özellikler için test yazın
- Dokümantasyon ekleyin

## Pull Request Süreci

1. Bir issue açın veya mevcut bir issue'yu seçin
2. Feature branch oluşturun: `git checkout -b feature/yeni-ozellik`
3. Değişikliklerinizi yapın
4. Testleri çalıştırın: `cargo test`
5. Commit edin: `git commit -m "feat: yeni özellik eklendi"`
6. Push edin: `git push origin feature/yeni-ozellik`
7. Pull request açın

## Commit Mesajları

Conventional Commits formatını kullanın:
- `feat:` yeni özellik
- `fix:` hata düzeltmesi
- `docs:` dokümantasyon
- `refactor:` kod iyileştirme
- `test:` test ekleme
- `chore:` bakım işleri

## Yeni Araç Ekleme

Bkz. [docs/TOOLS.md](docs/TOOLS.md)