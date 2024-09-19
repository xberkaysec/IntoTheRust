# Rust Nedir?

- Güvenli ve Hızlı Sistem Programlama Dili

Rust, Mozilla ve bir topluluk tarafından geliştirilen modern bir sistem programlama dilidir. 
C ve C++ gibi dillerle benzerlikler taşırken, Rust'ın sunduğu önemli avantajlar sayesinde yazılım geliştirme sürecinde güvenlik ve performans ön plandadır.

## Rust'ın Temel Özellikleri

1. Bellek Güvenliği

Rust, bellek yönetiminde güvenliği sağlamak için tasarlanmıştır. C ve C++ gibi dillerde yaygın olarak karşılaşılan bellek hataları, 
Rust'ın sahip olduğu "ownership" (sahiplik) sistemi sayesinde minimize edilmiştir. 
Bu sistem, geliştiricilerin bellek üzerinde daha fazla kontrol sahibi olmasını sağlar ve potansiyel güvenlik açıklarını azaltır.

2. Paralel Programlama

Günümüzdeki bilgisayarlar çoğunlukla çoklu işlemci mimarisiyle donatılmıştır. 
Rust, eşzamanlı (concurrent) programlamayı kolaylaştırarak, geliştiricilerin modern makinelerin tüm potansiyelinden yararlanmasına olanak tanır.
Bu, performansı artırırken, aynı zamanda yeni hata türlerinin ortaya çıkma riskini de azaltır.

3. Performans

Rust, C ve C++ ile karşılaştırılabilir bir performans sunar. Geliştiriciler, düşük seviyeli bellek işlemleri üzerinde tam kontrol sahibi olurken,
dilin sağladığı soyutlamalar sayesinde daha verimli ve güvenli kod yazabilirler. Rust, "kullanmıyorsanız ödemiyorsunuz" ilkesine dayanarak tasarlanmıştır.

## Neden Rust Tercih Edilmeli?

1. Güvenlik Açıkları

Geliştiriciler, yazılım güvenliği konusunda sürekli bir mücadele içindedir. 
Rust, güvenli kod yazma sürecini kolaylaştırarak, yazılımda meydana gelebilecek güvenlik açıklarını minimize eder. 
Bu, özellikle kritik sistemler için büyük bir avantajdır.

2. Çoklu İşlemci Desteği
   
Rust, çoklu iş parçacığı (multithreading) desteği ile modern donanımın verimliliğini artırır. 
Eşzamanlı programlama ile ilgili zorlukları azaltarak, geliştiricilerin daha az hata ile daha karmaşık uygulamalar yazmasına olanak tanır.

3. Sahiplik Sistemi ve Güvenli Eşzamanlılık

Rust, modern programlama dilleri arasında dikkat çeken bir yere sahiptir.
Bu başarı, Rust'ın yenilikçi sahiplik (ownership), taşımalar (moves) ve ödünç alma (borrowing) sistemine dayanmaktadır.
Bu sistem, derleme zamanında kontrol edilerek Rust’ın esnek statik tür sistemiyle uyumlu bir şekilde tasarlanmıştır.

## Sahiplik Sistemi Nedir?

Rust'ın sahiplik sistemi, her bir değer için belirli bir yaşam süresi tanımlar. 
Bu, dilin temelinde çöp toplama (garbage collection) gereksinimini ortadan kaldırır. 
Ayrıca, soketler ve dosya tanıtıcıları gibi diğer kaynakları yönetmek için sağlam ama esnek arayüzler sağlar. 

1. Taşımalar (Moves)

Değerlerin bir sahipten diğerine aktarılmasını sağlar. 
Bu, bellek yönetimini daha güvenli hale getirirken, aynı zamanda performansı artırır.

2. Ödünç Alma (Borrowing)

Kodun bir değeri geçici olarak kullanmasına izin verirken, o değerin sahipliğini etkilemez. 
Bu özellik, özellikle çok iş parçacıklı (multithreaded) programlamada büyük avantajlar sunar.

## Güvenli Eşzamanlılık

Rust'ın sahiplik kuralları, güvenilir bir eşzamanlılık modelinin temelini oluşturur. 
Çoğu dil, bir mutex ile koruduğu veri arasındaki ilişkiyi yorumlara bırakırken, Rust bu kontrolü derleme zamanında yapar.
Yani, kodunuz veriye erişirken mutex'in kilitlendiğini kontrol eder. 

Not: Mutex , paylaşılan kaynağa yalnızca bir iş parçacığına özel erişim veren bir eşitleme temel öğesidir. 
Bir iş parçacığı bir mutex alırsa, bu mutex almak isteyen ikinci iş parçacığı ilk iş parçacığı mutex serbest bırakana kadar askıya alınır.

Rust, başka bir iş parçacığına bir veri yapısını verdikten sonra onu kullanmamanız gerektiğini belirtmekle kalmaz; 
aynı zamanda bunu derleme aşamasında kontrol eder.
Bu sayede, veri yarışlarını (data races) önleyerek güvenli bir programlama deneyimi sunar.

## Rust'ın Programlama Paradigmaları

Rust, tam anlamıyla nesne yönelimli (object-oriented) bir dil değildir; ancak bazı nesne yönelimli özelliklere sahiptir. 
Ayrıca, işlevsel (functional) bir dil olarak da tanımlanamaz; fakat işlevsel dillerde olduğu gibi hesaplamanın sonuçlarını daha belirgin hale getirir. 
C ve C++ dillerine benzerlik gösterse de, bu dillerin tipik kalıpları Rust için geçerli değildir. 
Rust’ın ne tür bir dil olduğunu anlamak için, dilin özelliklerine aşina olmanız en iyisidir.

## Gerçek Dünya Uygulamaları: Servo

Mozilla, Rust dilinin tasarımını gerçek dünya ortamında test etmek amacıyla Servo adında yeni bir web tarayıcı motoru geliştirmiştir.
Servo’nun ihtiyaçları ile Rust’ın hedefleri iyi bir uyum içindedir: Bir tarayıcı hem yüksek performans göstermeli hem de güvenilir verileri işlemelidir. 

Servo, Rust’ın güvenli eşzamanlılığını kullanarak, C veya C++ dillerinde paralelleştirilmesi zor olan görevleri gerçekleştirmek için tüm makina gücünü kullanır.
Bu iki proje birlikte gelişmiştir; Servo en son dil özelliklerini kullanırken, Rust da Servo geliştiricilerinden gelen geri bildirimler doğrultusunda evrim geçirmiştir.

# Rust ve Tür Güvenliği: Neden Önemlidir?

Rust, tür güvenliği (type safety) sunan bir programlama dilidir. 
Ancak "tür güvenliği" derken tam olarak neyi kastettiğimizi anlamak önemlidir. 
Güvenli olmak kulağa hoş gelse de, aslında hangi tehlikelerden korunuyoruz? 

## Tanımsız Davranış Nedir?

C programlama dili için 1999 yılında belirlenen standart olan C99'a göre tanımsız davranış, 
taşınabilir olmayan veya hatalı bir program yapısının ya da hatalı verinin kullanılması durumunda meydana gelen ve
bu uluslararası standart tarafından hiçbir gereklilik getirilmeyen bir davranıştır.

## C Programı Örneği

Aşağıdaki C programını ele alalım:

```c
int main(int argc, char **argv) {
    unsigned long c[1];
    c[3] = 0x7ffff7b36cebUL;
    return 0;
}
```

Bu program, c dizisinin sonundan bir elemanı erişmeye çalıştığı için tanımsız bir davranış sergiler. 
C99'a göre, bu tür bir erişim sonucu programın davranışı belirsizdir; yani, program her türlü sonuç verebilir.

Örneğin, bu programı bilgisayarımızda çalıştırdığımızda şu çıktıyı alıyoruz:

```
undef: Error: .netrc file is readable by others.
undef: Remove password or make file unreadable by others.
```

Sonuçta program çöküyor. Bilgisayarda .netrc dosyası bile yoktur. Eğer siz de denerseniz, muhtemelen tamamen farklı bir sonuçla karşılaşacaksınız.

## C'nin Belirsizlikleri

C derleyicisi, bu ana fonksiyon için ürettiği makine kodunu, c dizisini geri dönüş adresinden üç kelime önce yığının (stack) üzerine yerleştirecek şekilde tasarlamıştır. 
Böylece `c[3]'e 0x7ffff7b36cebUL` değerini atamak, main fonksiyonunun geri dönüş adresini değiştirir ve 
bu da C standart kütüphanesindeki bir kod parçasına yönlendirir. Bu durum, programın beklenmedik bir şekilde çalışmasına neden olur.

C99 standardı, derleyiciye bu tür belirsizlikleri göz ardı etme yetkisi vermektedir. 
Tanımsız bir işlem yalnızca belirsiz bir sonuç üretmekle kalmaz; aynı zamanda programın tamamen rastgele bir şekilde davranmasına izin verir.

# Rust ile Tür Güvenliği

Rust, bu sorunları çözmek için tasarlanmıştır. Tür güvenliği sayesinde, bellek yönetimi ve veri erişimi konularında daha fazla kontrol sağlar.
Rust, geliştiricilerin hatalı bellek erişimlerini önceden tespit etmelerine yardımcı olur ve bu sayede tanımsız davranışların ortaya çıkmasını engeller. 

Rust’ın sahiplik (ownership) ve ödünç alma (borrowing) sistemleri, bellek güvenliğini sağlarken performansı da artırır. 
Bu sayede geliştiriciler, daha güvenilir ve hatasız yazılımlar geliştirme imkanı bulurlar.


## Tanımlanmış ve Belirsiz Davranış

Araştırmalar, birçok programın belirsiz davranış sergilediğini göstermektedir. 
Utah Üniversitesi'nde çalışan Peng Li, C ve C++ derleyicilerini modifiye ederek belirli belirsiz davranışları raporlayan bir sistem geliştirmiştir. 
Sonuçlar, iyi standartlara sahip projeler dahil olmak üzere hemen hemen tüm programların belirsiz davranışlar içerdiğini ortaya koymuştur. 
Bu tür belirsizlikler, güvenlik açıklarına yol açabilmektedir; örneğin, Morris worm'u bu tür bir açığı kullanarak yayıldı ve bu tür istismarlar günümüzde de yaygın olarak kullanılmaktadır.

## Tanımlar: İyi Tanımlanmış ve Tür Güvenliği

Bir program, hiçbir olası yürütme durumunun belirsiz davranış sergilemeyecek şekilde yazılmışsa, bu program "iyi tanımlanmış" olarak adlandırılır.
Eğer bir dilin güvenlik kontrolleri her programın iyi tanımlanmış olmasını sağlıyorsa, bu dil "tip güvenli" olarak kabul edilir.

C veya C++ dillerinde dikkatlice yazılmış bir program iyi tanımlanmış olabilir, ancak bu diller tip güvenli değildir.
Örneğin, daha önce gösterilen bir programda tip hatası yoktur; ancak yine de belirsiz davranış sergileyebilir.
Buna karşılık, Python tip güvenli bir dildir. Python, dizinin sınırlarını aşan indeksleri daha dostane bir şekilde tespit etmek için işlemci zamanı harcamaktadır:

```
>>> p = [0]
>>> p[3] = 0x7ffff7b36ceb
Traceback (most recent call last):
 File "<stdin>", line 1, in <module>
IndexError: list assignment index out of range
```

Python, c[3] ataması için bir IndexError istisnası vermiştir; bu durum belirsiz davranış değildir,
çünkü Python belgeleri bu atamanın bir IndexError istisnası vermesi gerektiğini belirtmektedir.
Elbette, ctypes gibi makineye sınırsız erişim sağlayan bir modül Python'da belirsiz davranışa neden olabilir, ancak temel dil kendisi tip güvenlidir.
Java, JavaScript, Ruby ve Haskell de benzer özelliklere sahiptir.

## Tip Güvenliği ve Derleme Zamanı Kontrolleri

Tip güvenliği, bir dilin tipleri derleme zamanında mı yoksa çalışma zamanında mı kontrol ettiğinden bağımsızdır:
C derleme zamanında kontrol eder ve tip güvenli değildir; Python ise çalışma zamanında kontrol eder ve tip güvenlidir.

C ve C++ gibi baskın sistem programlama dillerinin tip güvenli olmaması ironiktir. 
Bu diller, sistemin temellerini uygulamak için tasarlanmış olup, güvenlik sınırlarını uygulamak ve güvensiz verilerle etkileşimde bulunmak için kullanılmaktadır.
Bu nedenle, tip güvenliğinin bu diller için özellikle değerli bir nitelik olması beklenir.

## Çoklu İşlemci Programlama ve Rust

Rust’ın tip güvenliği, çoklu iş parçacığı programlaması için beklenmedik sonuçlar doğurmaktadır.
C ve C++ dillerinde eşzamanlılık genellikle zorlayıcıdır; geliştiriciler genellikle tek iş parçacıklı kodlarının,
yeterli performansı sağlayamadığı durumlarda eşzamanlılığa başvururlar.
Ancak Rust, eşzamanlı kodun veri yarışlarından arındırılmış olmasını garanti eder ve 
mutexler veya diğer senkronizasyon araçlarının yanlış kullanımını derleme zamanında yakalar.
Rust ile eşzamanlılık kullanırken kodunuzu yalnızca en deneyimli programcıların çalışabileceği hale getirdiğinizden endişelenmenize gerek yoktur.
