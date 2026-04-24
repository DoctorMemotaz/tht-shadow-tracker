# 🇹🇷 THT Shadow Tracker | Advanced OpSec Framework

<div align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-0078d7?style=for-the-badge&logo=windows" alt="Windows">
  <img src="https://img.shields.io/badge/Core-Electron-47848f?style=for-the-badge&logo=electron" alt="Electron">
  <img src="https://img.shields.io/badge/Engine-Node.js-339933?style=for-the-badge&logo=node.js" alt="Node.js">
  <img src="https://img.shields.io/badge/Security-Obfuscated-critical?style=for-the-badge" alt="Security">
</div>

<br>

> **Operasyonel Güvenlik (OpSec) ve Adli Bilişim Karşıtlığı (Anti-Forensics) Aracı**
> İşletim sisteminin derinliklerinde (Prefetch, Recent, RunMRU, Temp) bırakılan uygulama ve dosya erişim izlerini zaman damgası tabanlı analiz eden ve adli bilişim standartlarında yok eden kapalı kaynak mimarili bir gözetim aracıdır.

---

## ⚙️ Teknik Mimari ve Çalışma Prensibi

THT Shadow Tracker, Windows işletim sisteminin telemetri ve önbellek mekanizmalarını tersine çevirerek çalışır. 

* **Base64 Payload Injection:** PowerShell üzerinden sistemin en alt katmanlarına gönderilen tarama ve silme komutları, açık metin olarak gönderilmez. Komutlar **UTF-16LE Base64** formatında şifrelenir ve `-EncodedCommand` parametresiyle işletim sistemine enjekte edilir. Bu sayede Windows Execution Policy (Çalıştırma İlkesi) kısıtlamaları ve String tespit mekanizmaları tamamen **Bypass** edilir.
* **Zaman Damgası Kancalama (Timestamp Hooking):** Araç "Gözlem Modu"na alındığı an sistemin epoch zamanını kaydeder. Operasyon bittikten sonra, sadece bu zaman aralığında oluşturulmuş veya değiştirilmiş kalıntıları filtreleyerek hedefe yönelik isabetli bir temizlik sağlar.
* **IPC Köprü İzolasyonu:** Uygulamanın ön yüzü (UI) ve arka plan sistem işlemleri (Node.js/Child_Process), Context Bridge üzerinden tamamen izole edilmiştir (`nodeIntegration: false`).

---

## 🛡️ Anti-Reverse Engineering (Tersine Mühendislik Koruması)

Bu projenin çekirdek dosyaları (`main.js`, `preload.js`, `renderer.js`) standart bir okumayı veya deşifre etmeyi imkansız kılacak **Abstract Syntax Tree** manipülasyonu ile korunmaktadır:

* **Control Flow Flattening (Akış Düzleştirme):** Kodun mantıksal yapısı spagetti koda çevrilmiş, if-else ve döngü yapıları devasa switch-case bloklarına hapsedilmiştir.
* **RC4 String Encryption:** Uygulama içindeki komutlar, yollar ve log çıktıları statik olarak okunamaz. Tüm string ifadeler RC4 algoritmasıyla şifrelenmiş olup, sadece çalışma anında (runtime) bellekte çözülür.
* **Dead Code Injection:** Analiz araçlarını yormak ve dosya boyutunu manipüle etmek amacıyla çalışan kodun arasına rastgele, ölü ve anlamsız kod blokları gömülmüştür.
* **Self-Defending (Öz Savunma):** Kodu deobfuscate etmek veya formatlamak (beautify) isteyen herhangi bir girişim, uygulamanın kendi kendini bir sonsuz döngüye sokarak çökertmesiyle sonuçlanır.

---

## 🔍 Hedeflenen Sistem Eserleri (Artifacts)

Araç aşağıdaki Windows adli bilişim izlerini tarar ve yok eder:

1. **Prefetch (`C:\Windows\Prefetch\*.pf`):** Çalıştırılan tüm `.exe` dosyalarının ve yükledikleri DLL'lerin önbellek kayıtları.
2. **Recent Files (`%APPDATA%\Microsoft\Windows\Recent`):** Explorer üzerinden erişilen, açılan veya değiştirilen dosyaların LNK (kısayol) kayıtları.
3. **RunMRU (Registry):** Kayıt defteri üzerinde, çalıştır (Win+R) veya arama çubuğu üzerinden tetiklenen komutların geçmişi.
4. **Temp Directory:** İşletim sisteminin operasyon anında oluşturduğu geçici yığın dosyaları.

---

## 🚀 Kurulum ve Derleme (Build)

Uygulamanın Prefetch ve Registry katmanlarına inebilmesi için **Yönetici Hakları (Administrator Privileges)** gereklidir.
