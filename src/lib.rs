#![no_std]

use core::mem::size_of;
use core::str::from_utf8_unchecked;

const FSBLOB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"),"/fsblob.bin"));
const FILE_COUNT: *const u32 = FSBLOB.as_ptr() as *const u32;

fn get_u32_from_blob(offset: u32) -> u32 {
    let mut data_buffer: [u8; 4] = [0; 4];
    let off: usize = offset as usize;
    data_buffer[0] = FSBLOB[off + 0];
    data_buffer[1] = FSBLOB[off + 1];
    data_buffer[2] = FSBLOB[off + 2];
    data_buffer[3] = FSBLOB[off + 3];
    u32::from_le_bytes(data_buffer)
}

fn get_u32_from_blob_by_idx(idx: u32) -> u32 {
    get_u32_from_blob(idx * 4)
}

fn get_u16_from_blob(offset: u32) -> u16 {
    let mut data_buffer: [u8; 2] = [0; 2];
    let off: usize = offset as usize;
    data_buffer[0] = FSBLOB[off + 0];
    data_buffer[1] = FSBLOB[off + 1];
    u16::from_le_bytes(data_buffer)
}


fn get_file_by_index(file_idx: u32) -> (&'static str, &'static [u8]) {
    let file_offset = get_u32_from_blob_by_idx(file_idx + 1);
    let name_length = get_u16_from_blob(file_offset);
    let name_offset = file_offset + (size_of::<u16>() as u32);
    let name: &'static str = unsafe { from_utf8_unchecked(&(FSBLOB[name_offset as usize..(name_offset + name_length as u32) as usize])) };
    let file_length_offset = name_offset + name_length as u32;
    let file_length = get_u32_from_blob(file_length_offset);
    let file_offset = file_length_offset +  (size_of::<u32>() as u32);
    let file: &'static [u8] = &FSBLOB[file_offset as usize..(file_offset + file_length) as usize];
    (name, file)
}

pub struct FsIterator {
    current_file: u32
}

impl FsIterator {
    fn iter() -> Self {
        FsIterator {current_file: 0u32}
    }
}

pub fn iter() -> FsIterator {
    FsIterator::iter()
}

impl Iterator for FsIterator {
    type Item = (&'static str, &'static [u8]);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let current = self.current_file;
        self.current_file += 1;
        if current >= unsafe { *FILE_COUNT } {
            None
        } else {
            Some(get_file_by_index(current))
        }

    }
}

pub fn read_file(fname: &str) -> Option<&'static [u8]> {
    for (name, buf) in iter() {
        if name == fname {
            return Some(buf);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::println;
    use super::*;

    #[test]
    fn iterator_test() {
        let file_iter = iter();
        for (name, buf) in file_iter {
            println!("File name: {}; File length: {}", name, buf.len());
        }
    }
}
