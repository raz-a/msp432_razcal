
use std::fs;

fn main() {
    let config_string = fs::read_to_string("razcal.toml");
    match config_string {
        Ok(buffer) => {
            println!("cargo:rustc-cfg=msp432_package=\"{}\"", buffer);   
        },

        Err(_) => {
            println!("NOOOOOO");
        }
    }
}