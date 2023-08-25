#![feature(core_intrinsics)]
use std::env::var;
use std::intrinsics::size_of;
#[allow(unused_imports)]
use std::io::{stderr, Write};
use std::fs;
use std::vec::Vec;

#[allow(unused_macros)]
macro_rules! err {
    ($( $x:expr ),+) => {
        writeln!(&mut stderr(), $($x,)+);
    };
}

fn list_files_in_subdirs_intenal(dirname: String, path_list: &mut Vec<String>) {
    for path_err in fs::read_dir(dirname).unwrap() {
        if let Ok(path) = path_err {
            let mut subdirsubfile = String::from(path.path().as_os_str().to_str().unwrap());
            if path.file_type().unwrap().is_dir() {
                list_files_in_subdirs_intenal(subdirsubfile, path_list);
            } else {
                subdirsubfile = subdirsubfile.replace("\\", "/");
                path_list.push(subdirsubfile);
            }
        }
    }
}

fn list_files_in_subdirs(dirname: &String) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    list_files_in_subdirs_intenal(dirname.clone(), &mut paths);
    paths
}

fn get_file_content(filename: &String) -> Option<Vec<u8>> {
    println!("cargo:rerun-if-changed={}", filename);
    if let Ok(content) = fs::read(filename) {
        Some(content)
    } else {
        None
    }
}

fn create_filename(prefix: &String, path: &String)->String {
    let new_path = path.clone();
    new_path.replace(prefix, "")
}

fn generate_fs_blob(prefix: &String, files: &Vec<String>) -> Vec<u8> {
    struct FileEntry(String, Vec<u8>);

    let mut blob: Vec<u8> = Vec::new();

    let data_offset = (files.len() + 1) * size_of::<u32>();
    blob.resize(data_offset, 0u8);
    let mut header: Vec<u32> = Vec::new();
    header.resize(files.len() + 1, 0u32);
    header[0] = files.len() as u32;

    let file_entries = files.iter().map(|x| FileEntry(create_filename(prefix, x),
        get_file_content(x).unwrap())).collect::<Vec<FileEntry>>();

    let mut offset = data_offset as u32;
    let mut header_offset = 1usize;
    for entry in file_entries {
        let mut data: Vec<u8> = Vec::new();
        data.append(&mut (entry.0.as_bytes().len() as u16).to_le_bytes().to_vec());
        data.append(&mut entry.0.as_bytes().to_vec());
        data.append(&mut (entry.1.len() as u32).to_le_bytes().to_vec());
        data.append(&mut entry.1.clone());
        header[header_offset] = offset;
        header_offset += 1;
        offset += data.len() as u32;
        blob.append(&mut data);
    }
    let headblob = header.iter().map(|x| {x.to_le_bytes().to_vec()}).fold(Vec::new(), |x, y| {
        let mut r = x.clone();
        r.append(&mut y.clone()); r
    });
    blob[0..data_offset].copy_from_slice(headblob.as_slice());
    blob
}

const BINARY_FILE_NAME: &str = "fsblob.bin";

fn generate_filesystem() {
    println!("cargo:rerun-if-env-changed=ROOT_FS_DIR");
    let src_dir = var("ROOT_FS_DIR").unwrap();
    let out_dir = var("OUT_DIR").unwrap();
    let files = list_files_in_subdirs(&src_dir);
    let blob = generate_fs_blob(&src_dir, &files);
    let mut binary_path = out_dir.clone() + "/" + BINARY_FILE_NAME;
    binary_path = binary_path.replace("\\", "/");
    let _ = fs::write(&binary_path, blob);
    // err!("ss: {}", binary_path);
    // panic!("Finished!");
}

fn main() {
    generate_filesystem();
}