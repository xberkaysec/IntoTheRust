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

Resim :

![Resim](https://i.ibb.co/x254wVm/resim-2024-09-19-160414076.png)

Yukarıdaki komutları çalıştırarak her birinin hangi sürümde olduğunu sorguladık.
Komutların işlevleri şu şekildedir:

1. Cargo: Rust'ın derleme yöneticisi, paket yöneticisi ve genel amaçlı aracıdır.
Yeni bir proje başlatmak, programınızı derlemek ve çalıştırmak,
ayrıca kodunuzun bağımlı olduğu dış kütüphaneleri yönetmek için Cargo'yu kullanabilirsiniz.
  
2. rustc: Rust derleyicisidir. Genellikle Cargo'nun derleyiciyi bizim için çağırmasına izin veririz, ancak bazen doğrudan çalıştırmak faydalı olabilir.

3. rustdoc: Rust belgelendirme aracıdır. Programınızın kaynak kodundaki uygun biçimdeki yorumlarda belgeler yazarsanız,
rustdoc bunlardan güzel formatlanmış HTML oluşturabilir. rustc gibi, genellikle Cargo'nun rustdoc'u çalıştırmasına izin veririz.

Cargo, standart meta verilerle yeni bir Rust paketi oluşturma konusunda da bize yardımcı olabilir:

```bash
$ cargo new --bin hello
 Created binary (application) `hello` package
```

Resim : 

![Resim](https://i.ibb.co/yRC6NPm/Cargo-New-Project.png)


## Proje Dizini Yapısını İnceleme

Yeni oluşturduğunuz hello dizinine geçelim:

```bash
$ cd hello
$ ls -la or dir
```

Bu komut, dizininizdeki dosya ve klasörlerin listesini gösterir. Çıktınız şöyle görünmelidir:

![Resim](https://i.ibb.co/6JFbQjb/Directory.png)

## Proje Dosyalarının Açıklaması

Cargo, proje dizininde birkaç önemli dosya ve klasör oluşturur:

1. Cargo.toml: Bu dosya, projenizin meta verilerini içerir. Başlangıçta şu şekilde görünür:

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Resim :

![Resim](https://i.ibb.co/n8dsVKB/Rust-Toml-File.png)

Burada, projenizin adı, sürümü vb yer alır. İleride başka kütüphanelere bağımlılıklar eklemeniz gerektiğinde, bu dosyada güncellemeler yapabilirsiniz.

2. .git ve .gitignore: Cargo, projenizi Git versiyon kontrol sistemi ile kullanıma hazır hale getirir.
.gitignore dosyası, hangi dosyaların versiyon kontrolüne alınmayacağını belirler.

3. src Klasörü: Uygulamanızın gerçek Rust kodunu barındıran klasördür. İçinde main.rs adında bir dosya bulunur:

```bash
$ cd src
$ ls -l or dir
```

Çıktınız şöyle görünmelidir:

Resim :

![Resim](https://i.ibb.co/cYB1Cwc/Rust-Src-File.png)
