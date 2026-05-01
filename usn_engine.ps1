function Get-USNStatus {
    param([string]$drive = "C:")
    Write-Host "[*] $drive sürücüsü için USN Jurnali sorgulanıyor..." -ForegroundColor Cyan
    fsutil usn queryjournal $drive
}

function Purge-USNJournal {
    param([string]$drive = "C:")
    try {
        Write-Host "[!] USN Jurnali siliniyor ve geçmiş sıfırlanıyor..." -ForegroundColor Red
        fsutil usn deletejournal /D $drive
        
        Write-Host "[+] Temiz USN Jurnali yeniden oluşturuluyor..." -ForegroundColor Green
        fsutil usn createjournal m=1000 a=100 $drive
    } catch {
        Write-Host "[HATA] USN manipülasyonu başarısız: $_" -ForegroundColor Yellow
    }
}