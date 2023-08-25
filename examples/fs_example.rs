use simple_ro_fs;

fn main() {
    let mut file_name: String = String::from("");
    for (file, file_buf) in simple_ro_fs::iter() {
        println!("File name: {}; Length: {}", file, file_buf.len());
        file_name = String::from(file);//Save the last file to demonstrate another API
    }

    //Print hex dump of the last file. Just for demo
    if let Some(file) = simple_ro_fs::read_file(&file_name) {
        for chunk in file.chunks(16) {
            let res = chunk.iter().map(|x| format!("0x{:02X} ", x)).fold(String::new(), |acc, x| acc + &x);
            println!("{}", res);
        }
    }
}
