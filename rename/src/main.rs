use clap::Parser;
use std::fs::{read_dir, rename};

#[derive(Parser, Debug)]
struct Args {
    dir: String,
    index: u32,
    name: String,
}

fn main() {
    let args = Args::parse();
    let entries = read_dir(&args.dir).expect("Failed to read directory");

    let mut index = args.index;
    for entry in entries {
        let path = entry.expect("Failed to read path").path();
        if let Some(path_str) = path.to_str() {
            if !path_str.contains(&args.name) {
                if let Some(extension) = path.extension() {
                    let ext_str = extension.to_string_lossy();
                    let mut new_name = format!("{}-{}.{}", args.name, index, ext_str);
                    let mut new_path = path.with_file_name(&new_name);

                    while new_path.exists() {
                        index += 1;
                        new_name = format!("{}-{}.{}", args.name, index, ext_str);
                        new_path = path.with_file_name(&new_name);
                    }
                    rename(&path, &new_path).expect("Failed to rename file");
                }
            }
        }
    }
    println!("Renamed {} images", index)
}
