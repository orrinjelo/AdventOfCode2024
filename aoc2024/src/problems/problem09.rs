use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::fmt;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
pub struct Disk {
    alloc: Vec<u32>,
    hdd: Vec<MemNugget>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct File {
    filename: u32,
    filesize: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum MemNugget {
    FILE(File),
    EMPTY,
}

impl Disk {
    pub fn new(input: Vec<String>) -> Disk {
        let mut d = Disk {
            alloc: Vec::new(),
            hdd: Vec::new(),
        };

        d.alloc = input[0].chars()
                .into_iter()
                .map(|x|
                     x.to_digit(10).unwrap()
                ).collect();

        // debug!("{:?} {:?}", input[0].chars(), d);

        for i in 0..(d.alloc.len()+1)/2 {
            // Allocate file
            let f = File {
                filename: i as u32,
                filesize: d.alloc[i*2],
            };
            for _ in 0..f.clone().filesize {
                d.hdd.push(MemNugget::FILE(f.clone()));
            }
            if d.alloc.len() == i*2+1 {
                break;
            }
            // Write freespace
            for _ in 0..d.alloc[i*2+1] {
                d.hdd.push(MemNugget::EMPTY);
            }
        }

        d
    }
}


impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "|");
        for x in self.hdd.clone() {
            match x {
                MemNugget::FILE(file) => {
                    let _ = write!(f, "{}|", file.filename);
                },
                MemNugget::EMPTY => {
                    let _ = write!(f, ".|");
                }
            }
        }
        let _ = write!(f, "\n");
        fmt::Result::Ok(())
    }
}

pub fn fragment_disk(disk: &mut Disk) {
    for dp in 0..disk.hdd.len() {
        if disk.hdd[dp] == MemNugget::EMPTY {
            for fp in 1..disk.hdd.len() {
                let fpr = disk.hdd.len() - fp;
                if dp == fpr {
                    return;
                }
                if disk.hdd[fpr] != MemNugget::EMPTY {
                    disk.hdd[dp] = disk.hdd[fpr].clone();
                    disk.hdd[fpr] = MemNugget::EMPTY;
                    break;
                }
            }
        }
    }
}

pub fn defragment_disk(disk: &mut Disk) {
    // debug!("{}", disk);
    for dp in 0..disk.hdd.len() {
        // debug!("{}", disk.clone());
        let dpr = disk.hdd.len() - dp - 1;
        match &disk.hdd[dpr] {
            MemNugget::FILE(f) => {
                for fp in 0..dpr {
                    if disk.hdd[fp] == MemNugget::EMPTY {
                        // Check empty size
                        let mut empty_size: usize = 0;
                        for dpi in fp..disk.hdd.len() {
                            if disk.hdd[dpi] == MemNugget::EMPTY {
                                empty_size += 1;
                            } else {
                                break;
                            }
                        }
                        // debug!("{}", disk);
                        // debug!("fp: {}, ES: {}", fp, empty_size);
                        if f.filesize <= empty_size as u32 {
                            for dpi in 0..f.filesize as usize {
                                disk.hdd[fp+dpi] = disk.hdd[dpr-dpi].clone();
                                disk.hdd[dpr-dpi] = MemNugget::EMPTY;
                            }
                            break;
                        }
                    }
                }
            },
            MemNugget::EMPTY => {}
        }
    }
}

pub fn calc_checksum(disk: Disk) -> u64 {
    let mut s = 0u64;
    for i in 0..disk.hdd.len() {
        match &disk.hdd[i] {
            MemNugget::FILE(f) => {
                s += (i as u64) * f.filename as u64;
            }
            MemNugget::EMPTY => {}
        }
    }
    s
}

pub fn problem_091(input: Vec<String>) -> RetType {
    let mut disk = Disk::new(input);
    fragment_disk(&mut disk);
    return RetType::U64(calc_checksum(disk));
}

pub fn problem_092(input: Vec<String>) -> RetType {
    let mut disk = Disk::new(input);
    defragment_disk(&mut disk);
    return RetType::U64(calc_checksum(disk));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
   fn test_disk() {
        init();

        let input = vec![
            "12345".to_string(),
        ];

        let mut disk = Disk::new(input);

        assert_eq!(disk.alloc, vec![1, 2, 3, 4, 5]);
        assert_eq!(disk.hdd, vec![
            MemNugget::FILE(File{filename:0, filesize:1}),
            MemNugget::EMPTY,
            MemNugget::EMPTY,
            MemNugget::FILE(File{filename:1, filesize:3}),
            MemNugget::FILE(File{filename:1, filesize:3}),
            MemNugget::FILE(File{filename:1, filesize:3}),
            MemNugget::EMPTY,
            MemNugget::EMPTY,
            MemNugget::EMPTY,
            MemNugget::EMPTY,
            MemNugget::FILE(File{filename:2, filesize:5}),
            MemNugget::FILE(File{filename:2, filesize:5}),
            MemNugget::FILE(File{filename:2, filesize:5}),
            MemNugget::FILE(File{filename:2, filesize:5}),
            MemNugget::FILE(File{filename:2, filesize:5}),
        ]);

        fragment_disk(&mut disk);
        assert_eq!(calc_checksum(disk), 60);
    }

    #[test]
    fn test_fragmentation_checksum() {
        init();

        let input = vec![
            "2333133121414131402".to_string(),
        ];

        let mut disk = Disk::new(input);
        fragment_disk(&mut disk);
        assert_eq!(calc_checksum(disk), 1928);
    }

    #[test]
    fn test_defragmentation_checksum() {
        init();

        let input = vec![
            "2333133121414131402".to_string(),
        ];

        let mut disk = Disk::new(input);
        defragment_disk(&mut disk);

        debug!("{}", disk);
        assert_eq!(calc_checksum(disk), 2858);

    }
}