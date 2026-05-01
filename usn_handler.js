const { exec } = require('child_process');
const util = require('util');
const execPromise = util.promisify(exec);

async function manageUSN() {
    const drive = "C:";
    
    const psCommand = `fsutil usn deletejournal /D ${drive}`;
    const encodedCommand = Buffer.from(psCommand, 'utf16le').toString('base64');
    
    console.log(`[*] İşlem başlatıldı: ${drive} için USN Purge protokolü uygulanıyor...`);

    try {
        const { stdout, stderr } = await execPromise(`powershell -ExecutionPolicy Bypass -EncodedCommand ${encodedCommand}`);
        
        if (stderr) {
            console.error(`[!] Hata oluştu: ${stderr}`);
            return;
        }

        console.log(`[✓] NTFS Jurnal başarıyla sıfırlandı.\nDetaylar:\n${stdout}`);
    } catch (error) {
        console.error(`[X] Kritik Hata: Yetki yetersiz veya fsutil erişimi engellendi. (Admin haklarını kontrol et)`);
    }
}

manageUSN();