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

## Rust ile İlk Programınızı Yazma ve Çalıştırma: Adım Adım Kılavuz

Rust programlama diline yeni başlayanlar için ilk adım, "Hello, World!" programını yazmak ve çalıştırmaktır.
Bu yazıda, Cargo kullanarak nasıl basit bir Rust programı oluşturacağınızı ve çalıştıracağınızı öğreneceksiniz.

1. Proje Oluşturma

Öncelikle, yeni bir Rust projesi oluşturmalısınız. 
Terminalde aşağıdaki komutu kullanarak yeni bir uygulama paketi oluşturabilirsiniz:

```bash
cargo new --bin hello_world
```

Bu komut, hello adında bir dizin oluşturur ve gerekli dosyaları ayarlar.

2. main.rs Dosyasını İnceleme

Oluşturulan proje dizininde, src klasöründe main.rs dosyası bulunur. 
Bu dosya, Rust programınızın giriş noktasıdır. Varsayılan olarak şu şekilde görünür:

```rust
fn main() {
    println!("Hello, world!");
}
```

Burada, main fonksiyonu programın başlangıç noktasıdır ve println! makrosu ile ekrana "Hello, world!" yazdırır.

3. Programı Çalıştırma

Artık programınızı çalıştırmaya hazırsınız. 
Terminalde aşağıdaki komutu girerek programı derleyebilir ve çalıştırabilirsiniz:

```bash
cargo run
```

Bu komut, Cargo'nun Rust derleyicisini (rustc) çağırarak programı derlemesini sağlar.
Ardından, oluşturulan yürütülebilir dosyayı çalıştırır. Çıktı aşağıdaki gibi olacaktır:

```
Compiling hello v0.1.0 (file:///path/to/your/hello)
Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
 Running `target\debug\hello_world.exe`  
Hello, world!
```

Resim :

![Resim](https://i.ibb.co/LStRWkx/Cargo-Run-File.png)

4. Yürütülebilir Dosyanın Bulunduğu Yer

Cargo, yürütülebilir dosyayı proje dizininizdeki target/debug klasörüne yerleştirir. 
Bu dizindeki dosyaları görmek için şu komutu kullanabilirsiniz:

```bash
ls -l ../target/debug or dir target\debug 
````

Çıktınız aşağıdaki gibi görünmelidir:

Resim :

![Resim](https://i.ibb.co/kyDTqz0/resim-2024-09-19-171539146.png)

Burada hello dosyası, yürütülebilir dosyanızdır. 
Bunu doğrudan çalıştırmak için aşağıdaki komutu kullanabilirsiniz:

```bash
../target/debug/hello
```

5. Geçici Dosyaları Temizleme

Projenizle işiniz bittiğinde, Cargo otomatik olarak oluşturulan dosyaları temizlemenizi sağlar.
Aşağıdaki komutu kullanarak tüm geçici dosyaları kaldırabilirsiniz:

```bash
cargo clean
```

Temizleme işlemi tamamlandığında, artık yürütülebilir dosya mevcut olmayacaktır:

Bu komut çalıştırıldığında "No such file or directory" hatası alırsınız; bu da dosyanın silindiğini gösterir.

Resim :

![Resim](https://i.ibb.co/ZhPx14D/resim-2024-09-19-171809211.png)

# Rust ile En Büyük Ortak Bölgenin Hesaplanması: Euclid Algoritması

Rust programlama dilinin sözdizimi, C, C++, Java veya JavaScript gibi dillerden aşina olanlar için oldukça tanıdık bir yapıya sahiptir.
Bu yazıda, iki tam sayının en büyük ortak bölenini (EBOB) hesaplamak için Euclid algoritmasını kullanan basit bir Rust fonksiyonu inceleyeceğiz.

## GCD Fonksiyonu Nedir?

İki tamsayı için EBOB'u hesaplayan gcd adlı bir fonksiyon yer almaktadır. (/gcd_func.rs)


1. Fonksiyonun Yapısı

- Fonksiyon Tanımı: fn anahtar kelimesi ile başlar ve gcd adında bir fonksiyon tanımlanır.
- Bu fonksiyon, n ve m adında iki u64 (64-bit işaretsiz tam sayı) parametresi alır. Fonksiyon, u64 türünde bir değer döndürür.
- Değişkenler: Rust'ta değişkenler varsayılan olarak değiştirilemez.
Ancak, mut anahtar kelimesi ile değişkenlerin değeri değiştirilebilir hale getirilir. Bu sayede n ve m üzerinde işlem yapabiliriz.

2. Hata Kontrolü

Fonksiyonun başlangıcında yer alan assert! makrosu, her iki argümanın sıfır olmadığını kontrol eder. 
Eğer bu koşul sağlanmazsa, program hata mesajıyla sonlanır. 
Rust, bu tür kontrolleri her zaman gerçekleştirir; bu da güvenilir bir kod yazımını teşvik eder.

3. Döngü ve Koşul Kontrolü

Fonksiyonun ana kısmı, bir while döngüsü içerir. 
Rust, koşul ifadeleri için parantez kullanımını zorunlu kılmaz, 
ancak kontrol edilen ifadelerin süslü parantezler içinde yazılmasını gerektirir. 

4. Yerel Değişkenler

Fonksiyon içinde let ifadesi kullanılarak yerel bir değişken olan t tanımlanır. 
Rust, t değişkeninin türünü kullanımına göre otomatik olarak çıkarır; bu durumda u64 türündedir.

5. Değer Döndürme

Rust'ta bir fonksiyonun sonuna geldiğinde eğer son ifade bir noktalı virgülle bitmiyorsa, o ifade fonksiyonun dönüş değeri olur. 
Bu özellik, Rust programlamada yaygın olarak kullanılır.

# Rust ile Birim Testleri Yazma ve Çalıştırma

Rust, dilin içine yerleşik basit bir test desteği sunar. 
Bu yazıda, gcd (En Büyük Ortak Bölgenin) fonksiyonumuzu test etmek için nasıl birim testleri yazabileceğimizi öğreneceğiz.

1. Birim Testi Tanımlama

Rust'ta birim testleri, #[test] özniteliği ile işaretlenmiş fonksiyonlar aracılığıyla tanımlanır. 
GCD fonksiyonunu test eden örnek bir test fonksiyonu bulunmaktadır. (/gcd_test_func.rs)


2. Test Fonksiyonu Açıklaması

- Fonksiyon Adı: test_gcd, test edilen gcd fonksiyonunu çağırır ve doğru değerler döndürdüğünü kontrol eder.
- assert_eq! Makrosu: Bu makro, iki değerin eşit olup olmadığını kontrol eder. Eğer değerler eşit değilse, test başarısız olur.

3. Testlerin Çalıştırılması

Testleri çalıştırmak için terminalde aşağıdaki komutu kullanabilirsiniz:

```bash
$ cargo test
```

4. Çıktı Örneği

Komutu çalıştırdığınızda aşağıdaki gibi bir çıktı alacaksınız:

```bash
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s                                                    
Running unittests src/main.rs (target\debug\deps\gcd_test-62e132308c5a818b.exe)        

running 1 test
test test_gcd ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s 
```

Resim :

![Resim](https://i.ibb.co/H4hxKSb/resim-2024-09-20-123445500.png)

Bu çıktı, testlerinizin başarılı bir şekilde geçtiğini gösterir.
cargo test komutu, kaynak ağacınızda dağınık halde bulunan tüm test fonksiyonlarını otomatik olarak toplar ve çalıştırır.

5. Öznitelikler ve Kullanım Alanları

#[test] işareti, bir öznitelik örneğidir. 
Rust'taki öznitelikler, fonksiyonlar ve diğer tanımlamalar için ek bilgi sağlamak amacıyla kullanılan açık uçlu bir sistemdir. 
Öznitelikler, derleyici uyarılarını kontrol etmek, kod stilini denetlemek,
belirli koşullara göre kodu dahil etmek ve Rust'ın diğer dillerle nasıl etkileşime gireceğini belirtmek gibi birçok amaç için kullanılabilir.
