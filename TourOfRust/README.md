# Rust ile Programlama: Başlangıç Rehberi

Bu bölümde, Rust dilinin sözdizimi, türleri ve anlamsal özelliklerinin nasıl bir araya geldiğini keşfedeceğiz.
Rust, güvenli, eşzamanlı ve verimli kod yazmayı destekleyen birçok özelliğe sahiptir.
Rust'u indirip kurma sürecini inceleyecek, basit matematiksel kodlar yazacak,
üçüncü taraf kütüphanelerle bir web sunucusu oluşturacak ve Mandelbrot kümesini çizmek için çoklu iş parçacıkları kullanarak süreci hızlandıracağız.

## Rust İndirme ve Kurulum

Rust'u kurmanın en iyi yolu rustup kullanmaktır. 
[rustup.rs](https://rustup.rs) adresine gidin ve oradaki talimatları izleyin.
Alternatif olarak, [rust-lang.org](https://www.rust-lang.org) adresine giderek Linux, macOS ve Windows için önceden derlenmiş paketleri indirebilirsiniz.
Bazı işletim sistemi dağıtımları da Rust'u içermektedir. 
Ancak rustup'ı tercih ediyoruz çünkü bu, Rust kurulumlarını yönetmek için bir araçtır; Ruby için RVM veya Node için NVM gibi. 
Örneğin, Rust'un yeni bir sürümü yayınlandığında, rustup update komutunu yazarak güncellemeyi tek bir tıklama ile gerçekleştirebilirsiniz.

Kurulum işlemini tamamladıktan sonra, komut satırınızda üç yeni komutun mevcut olduğunu görmelisiniz:

```bash
$ cargo --version
cargo 1.81.0 (2dbb1af80 2024-08-20)

$ rustc --version
rustc 1.81.0 (eeb90cda1 2024-09-04)

$ rustdoc --version
rustdoc 1.81.0 (eeb90cda1 2024-09-04)
```

![](https://i.ibb.co/x254wVm/resim-2024-09-19-160414076.png)

