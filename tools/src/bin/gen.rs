use form::create_directory_structure;
use std::env::set_current_dir;
use std::fs::{create_dir_all, read_to_string, remove_dir_all, rename, File};
use std::io::Write;
use svd2rust::{generate, Config, Generation};

pub fn main() -> () {
    //parse xml
    let xml = read_to_string("svd/EFM32HG309F64.svd.patched").unwrap();
    let Generation {
        lib_rs,
        device_specific,
        ..
    } = generate(&xml, &Config::default()).unwrap();

    let device_specific = device_specific.unwrap();

    create_dir_all(".tmp/src").unwrap();
    set_current_dir(".tmp").unwrap();

    //save other files
    writeln!(
        File::create("device.x").unwrap(),
        "{}",
        device_specific.device_x
    )
    .unwrap();
    writeln!(
        File::create("build.rs").unwrap(),
        "{}",
        device_specific.build_rs
    )
    .unwrap();

    //form lib.rs and save
    create_directory_structure("src/", &lib_rs).unwrap();

    set_current_dir("..").unwrap();
    remove_dir_all("./src").unwrap_or_else(|err| println!("./src: {} [ignored]", err));
    rename(".tmp/build.rs", "./build.rs").unwrap();
    rename(".tmp/device.x", "./device.x").unwrap();
    rename(".tmp/src", "./src").unwrap();
    remove_dir_all(".tmp").unwrap();
}
