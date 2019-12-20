
use std::env;
use std::fs;

fn main() {
    println!("cargo:warning=build");
    let config_location = env!("RAZCAL_CONFIG");
    let config_string = fs::read_to_string(config_location);
    match config_string {
        Ok(buffer) => {
            println!("cargo:rustc-cfg=msp432_package=\"{}\"", buffer);   
        },

        Err(_) => {
            println!("cargo:warning=Cannot find config");
        }
    }
}