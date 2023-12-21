use std::env;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

fn main() {
    let mut to_split_file = true;
    let mut split_by_line = true;
    let mut split_size = String::from("1k");
    let mut path = None;

    env::args().for_each(|arg| {
        if arg.eq_ignore_ascii_case("merge") {
            to_split_file = false;
        } else if arg.starts_with("--type") {
            if "size".eq_ignore_ascii_case(arg.split("=").last().unwrap()) {
                split_by_line = false;
            }
        } else if arg.starts_with("--size") {
            split_size = String::from(arg.split("=").last().unwrap().trim().to_ascii_lowercase());
        } else if arg.starts_with("--target") {
            path = Some(PathBuf::from(arg.split("=").last().unwrap().trim()));
        }
    });

    let path = if let Some(path) = path {
        path
    } else {
        panic!("no specified file.");
    };

    if !path.is_file() {
        panic!("{:?} is not a file.", path);
    }

    if !path.exists() {
        panic!("{:?} is not exists.", path);
    }

    if to_split_file {
        let split_size = if split_size.ends_with("k") {
            split_size.replace("k", "").parse::<usize>().unwrap() * 1024
        } else if split_size.ends_with("m") {
            split_size.replace("m", "").parse::<usize>().unwrap() * 1024 * 1024
        } else if split_size.ends_with("g") {
            split_size.replace("g", "").parse::<usize>().unwrap() * 1024 * 1024 * 1024
        } else {
            split_size.parse::<usize>().unwrap()
        };

        if split_by_line {
        } else {
            split_by_size(path, split_size);
        }
    }
}

fn split_by_size(path: PathBuf, split_size: usize) {
    let parent_path = if let Some(parent) = path.parent() {
        parent
    } else {
        panic!("unknow error.");
    };

    let file_name = if let Some(name) = path.file_name() {
        name.to_str().unwrap()
    } else {
        panic!("unknow error.");
    };

    let mut reader = BufReader::new(File::open(path.as_path()).unwrap());

    let mut file_serial_num = 0;
    let mut file_path_to_write = PathBuf::from(parent_path);
    file_path_to_write.push(fetch_file_name(file_name, file_serial_num));
    let mut writer = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path_to_write)
            .unwrap(),
    );

    let mut byte_counter = 0;
    let mut buf = [0; 1024];
    while let Ok(n) = reader.read(&mut buf) {
        if n > 0 {
            if byte_counter + n >= split_size {
                let _ = writer.write_all(&buf[..(split_size - byte_counter)]);

                file_serial_num += 1;
                let mut file_path_to_write = PathBuf::from(parent_path);
                file_path_to_write.push(fetch_file_name(file_name, file_serial_num));
                writer = BufWriter::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(file_path_to_write)
                        .unwrap(),
                );
                let _ = writer.write_all(&buf[(split_size - byte_counter)..n]);
            } else {
                let _ = writer.write_all(&buf[..n]);
                byte_counter += n;
            }
        } else {
            break;
        }
    }
}

fn fetch_file_name(file_name: &str, serial: u32) -> String {
    let mut name = String::from(file_name);
    name.push_str("_");
    name.push_str(&serial.to_string());

    name
}
