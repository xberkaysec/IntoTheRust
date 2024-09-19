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

