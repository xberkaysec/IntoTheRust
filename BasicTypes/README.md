# Giriş

## Rust Programlama Dili: Güvenlik, Verimlilik ve Kısa Kod

Rust, modern yazılım geliştirme için tasarlanmış bir programlama dilidir. 
Geliştiricilere güvenli, verimli ve sade bir kod yazma imkanı sunar. 
Rust'ın sunduğu türler (types) sayesinde programcılar birçok yaygın hatayı önleyebilir.

## Güvenlik

Rust, programın türlerini kontrol ederek birçok yaygın hatayı önler. 
Null pointer'lar ve kontrolsüz birleşimler (unchecked unions) gibi hatalı kullanım senaryolarını ortadan kaldıran tür güvenli alternatifler sunar. 
Bu sayede, diğer dillerde sıkça karşılaşılan çökme hatalarının önüne geçilir.

## Verimlilik

Rust, programcıların değerlerin bellekte nasıl temsil edileceği üzerinde ince ayar yapmalarına olanak tanır. 
Bu, işlemcinin verimli bir şekilde yönetebileceği türleri seçmelerini sağlar. 
Geliştiriciler, kullanmadıkları genel veya esnek özellikler için ekstra maliyet ödemek zorunda kalmazlar.

## Kısa Kod

Rust, programcıdan fazla tür bilgisi talep etmeden bu avantajları sağlar. 
Rust ile yazılan programlar, benzer C++ programlarına göre genellikle daha az karmaşık ve daha az tür bilgisi içerir.

## Önceden Derleme

Rust, bir yorumlayıcı veya JIT (Just-In-Time) derleyici yerine, önceden derleme (ahead-of-time compilation) kullanacak şekilde tasarlanmıştır. 
Programınızın tamamı makine koduna çevrilir ve yürütmeye başlamadan önce bu işlem tamamlanır. 
Rust'ın türleri, önceden derleyiciye programınızın çalıştığı değerler için iyi makine düzeyinde temsiller seçmesine yardımcı olur. 
Bu temsillerin performansı tahmin edilebilir ve makinenin yeteneklerine tam erişim sağlar.

## Statik Tip Sistemi

Rust, statik olarak tiplenmiş bir dildir; yani program çalıştırılmadan önce derleyici, 
her olası yürütme yolunun değerleri yalnızca türleriyle tutarlı bir şekilde kullanacağını kontrol eder. 
Bu özellik, Rust'ın birçok programlama hatasını erken aşamada yakalamasını sağlar ve dilin güvenlik garantileri için kritik öneme sahiptir.

Rust, statik tipli bir dil olmasına rağmen, dinamik tipli diller (JavaScript veya Python gibi) ile 
karşılaştırıldığında daha fazla önceden planlama gerektirir. 
Fonksiyonların parametrelerinin ve dönüş değerlerinin türlerini, 
yapı (struct) türlerinin üyelerini ve bazı diğer yapıları önceden belirtmek zorundasınız.

## Rust'ın Avantajları

Rust'ın tür belirleme sürecini daha az zahmetli hale getiren önemli özellikleri vardır:

## 1. Tip Çıkarımı

Rust, belirttiğiniz türlere dayanarak çoğu durumda diğer türleri otomatik olarak çıkarabilir. 
Pratikte, bir değişken veya ifade için genellikle yalnızca bir tür geçerli olur; bu durumda Rust, türü belirtmenizi gerektirmeden işlemi tamamlar.

Örnek: build_vector.rs

Bu tanım, karmaşık ve tekrarlayıcıdır. Fonksiyonun dönüş türünden yola çıkarak v'nin bir Vec<i16> olduğu açıktır; başka bir tür geçerli olamaz. 
Bu durumda, her bir vektör elemanının da i16 olması gerektiği sonucuna varılır.

## 2. Daha Sade Kod Yazımı 

Rust, yukarıdaki tanımın yerine aşağıdaki gibi daha sade bir tanım yapmanıza olanak tanır.

Örnek : build_vector2.rs

Bu iki tanım tamamen eşdeğerdir; Rust, her iki durumda da aynı makine kodunu oluşturur. 
Tip çıkarımı, dinamik tipli dillerin okunabilirliğini geri kazandırırken, aynı zamanda derleme zamanında tür hatalarını yakalar.

Resim 1 :

![Resim](https://i.ibb.co/QnXG7Yt/Rust-Vector-1.png)

Resim 2 :

![Resim](https://i.ibb.co/VTKcLkv/Rust-Vector-2.png)

## Rust Programlama Dili: Türler ve Açıklamaları

Rust, güçlü bir tür sistemi ile birlikte gelir ve bu sistem, hem basit hem de karmaşık veri yapıları oluşturmanıza olanak tanır. 

## 1. Temel Sayı Türleri

Rust, çeşitli integer ve float türleri sunar:

- Integer:
  - i8: 8 bit işaretli tam sayı.
  - i16: 16 bit işaretli tam sayı.
  - i32: 32 bit işaretli tam sayı.
  - i64: 64 bit işaretli tam sayı.
  - u8: 8 bit işaretsiz tam sayı.
  - u16: 16 bit işaretsiz tam sayı.
  - u32: 32 bit işaretsiz tam sayı.
  - u64: 64 bit işaretsiz tam sayı.
  - isize: İşletim sisteminin kelime boyutuna göre değişen işaretli tam sayı.
  - usize: İşletim sisteminin kelime boyutuna göre değişen işaretsiz tam sayı.

- Float Sayılar:
  - f32: 32 bit kayan nokta sayısı.
  - f64: 64 bit kayan nokta sayısı.

## 2. Diğer Temel Türler

- Boolean (Bool):
  - bool: true veya false değerlerini alabilen mantıksal tür.

- Karakter (Char):
  - char: Tek bir Unicode karakterini temsil eden tür.

## 3. Tuple ve Unit Türleri

- Tuple:
  - (char, u8, i32): Farklı türlerdeki verileri bir arada tutan bir yapı.
  Örneğin, bir karakter, bir işaretsiz tam sayı ve bir işaretli tam sayı içeren bir tuple.

- Unit Türü:
  - (): Değer içermeyen bir tür. Fonksiyonların hiçbir değer döndürmediğini belirtmek için kullanılır.

## 4. Yapılar (Struct)

Yapılar, birden fazla değeri bir arada tutmak için kullanılır:

- Basit Yapı:
  ```rust
    struct Test {
      x: f32,
      y: f32,
  }
  ```
  
- Tuple Yapısı:
    struct Test(i32, char);

  
- Boş Yapı:
    struct Test;
  

## 5. Enum (Sıralı Türler)

Enumlar, belirli bir grup değeri temsil etmek için kullanılır:

- Basit Enum:
  ```rust
  enum Attend {
      OnTime,
      Late(u32),
  }
  ```

## 6. Kutu (Box)

Box, heap üzerinde veri saklamak için kullanılır:

- Box<Attend>: Attend enum'ını heap üzerinde saklamak için kullanılan bir kutu.

## 7. Referanslar

Rust, referans türlerini de destekler:

- Referanslar:
  - &i32: İşaretli tam sayının referansı.
  - &mut i32: Değiştirilebilir işaretli tam sayının referansı.

## 8. String ve Dizi Türleri

- String: Dinamik uzunlukta metin verisi.
- &str: Sabit uzunlukta metin dilimi.
  
- Diziler:
  - [f64; 4]: 4 elemanlı float dizisi.
  - [u8; 256]: 256 elemanlı işaretsiz tam sayı dizisi.

## 9. Vektörler

- Vektör:
  - Vec<f64>: Dinamik uzunlukta float dizisi.

## 10. Diğer Referans Türleri

- Dizi Referansları:
  - &[u8], &mut [u8]: Sabit ve değiştirilebilir dizi referansları.

## 11. Fonksiyon Türleri

Fonksiyonların türleri de tanımlanabilir:

- fn(&str, usize) -> isize: Bir string ve bir unsigned integer alıp işaretli tam sayı döndüren fonksiyon türü.
