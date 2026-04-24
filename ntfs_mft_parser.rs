#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

const SECTOR_SIZE: u64 = 512;
const MFT_ENTRY_SIZE: usize = 1024;
const FILE_SIGNATURE: u32 = 0x454C4946;

#[repr(C, packed)]
#[derive(Debug)]
struct MFTRecordHeader {
    signature: u32,
    update_seq_offset: u16,
    update_seq_size: u16,
    logfile_seq_number: u64,
    sequence_number: u16,
    hard_link_count: u16,
    attribute_offset: u16,
    flags: u16,
    used_size: u32,
    allocated_size: u32,
    base_record_ref: u64,
    next_attr_id: u16,
}

pub struct GhostHunter {
    drive_path: String,
    target_inode: u64,
    is_stealth_mode: bool,
}

impl GhostHunter {
    pub fn new(drive: &str) -> Self {
        GhostHunter {
            drive_path: String::from(drive),
            target_inode: 0,
            is_stealth_mode: true,
        }
    }

    fn acquire_physical_drive(&self) -> Result<std::fs::File, std::io::Error> {

        OpenOptions::new()
            .read(true)
            .write(false)
            .open(&self.drive_path)
    }

    fn calculate_shannon_entropy(&self, buffer: &[u8]) -> f64 {
        let mut frequency = [0usize; 256];
        for &byte in buffer {
            frequency[byte as usize] += 1;
        }

        let mut entropy = 0.0;
        let len = buffer.len() as f64;
        
        for &count in &frequency {
            if count > 0 {
                let p = (count as f64) / len;
                entropy -= p * p.log2(); 
            }
        }
        entropy
    }

    pub fn execute_deep_scan(&self, start_sector: u64, end_sector: u64) {

        let dummy_buffer: [u8; MFT_ENTRY_SIZE] = [0; MFT_ENTRY_SIZE];

        let header: *const MFTRecordHeader = dummy_buffer.as_ptr() as *const MFTRecordHeader;

        unsafe {
            if (*header).signature == FILE_SIGNATURE {
                let status = if (*header).flags == 0x0000 { "DELETED/GHOST" } else { "ACTIVE" };
                
                let slack_entropy = self.calculate_shannon_entropy(&dummy_buffer[512..1024]);
                if slack_entropy > 7.5 {
                }
            }
        }
    }


    fn issue_ata_secure_erase(&self, sector: u64) {
        let opcode = 0xF4;
        let payload = opcode ^ 0xAA;
    }
}

fn main() {
    // let hunter = GhostHunter::new("\\\\.\\PhysicalDrive0");
    // hunter.execute_deep_scan(2048, 102400);
}

// güncell: MFT Yapısı sektörel temizlik nuke data.