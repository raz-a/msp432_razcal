extern crate serde;
extern crate toml;

use std::collections::HashMap;
use std::env;
use std::fs;

/// Defines the fields of the RazCAL configuration toml.
#[derive(serde::Deserialize)]
struct RazCalConfig {
    /// The microcontroller RazCAL is being built for.
    mcu: String,
}

/// Supported MSP432 Package Types
const MSP432_PACKAGE_VQFN: &str = "vqfn";
const MSP432_PACKAGE_NFBGA: &str = "nfbga";
const MSP432_PACKAGE_LQFP: &str = "lqfp";

/// Defines the MSP432 compile-time configurations for a given MCU
struct Msp432Config {
    package: &'static str,
}

fn main() {
    // Supported MSP432 Variants:
    let msp432_supported_types = get_supported_mcus();

    // Find RAZCAL_CONFIG toml file
    let config_location = env!(
        "RAZCAL_CONFIG",
        "RAZCAL_CONFIG environment variable not found"
    );
    let config_string = fs::read_to_string(config_location).unwrap();
    let config: RazCalConfig = toml::from_str(&config_string).unwrap();

    match msp432_supported_types.get(&config.mcu.to_lowercase()) {
        Some(found_mcu) => {
            println!("cargo:rustc-cfg=razcal_msp432_package=\"{}\"", found_mcu.package);
        }

        None => {
            panic!("MSP432 Variant not supported.");
        }
    }

    // MSP432 Support 8-bit and 16-bit GPIO ports.
    println!("cargo:rustc-cfg=razcal_gpio_port_size=\"{}\"", 8);
    println!("cargo:rustc-cfg=razcal_gpio_port_size=\"{}\"", 16);
}

fn get_supported_mcus() -> HashMap<String, Msp432Config> {
    let mut support_map = HashMap::new();

    // MSP432P401M family
    support_map.insert(
        String::from("msp432p401mipz"),
        Msp432Config {
            package: MSP432_PACKAGE_LQFP,
        },
    );

    support_map.insert(
        String::from("msp432p401mipzr"),
        Msp432Config {
            package: MSP432_PACKAGE_LQFP,
        },
    );

    support_map.insert(
        String::from("msp432p401mirgcr"),
        Msp432Config {
            package: MSP432_PACKAGE_VQFN,
        },
    );

    support_map.insert(
        String::from("msp432p401mirgct"),
        Msp432Config {
            package: MSP432_PACKAGE_VQFN,
        },
    );

    support_map.insert(
        String::from("msp432p401mizxhr"),
        Msp432Config {
            package: MSP432_PACKAGE_NFBGA,
        },
    );

    support_map.insert(
        String::from("msp432p401mizxht"),
        Msp432Config {
            package: MSP432_PACKAGE_NFBGA,
        },
    );

    // MSP432P401R family
    support_map.insert(
        String::from("msp432p401ripz"),
        Msp432Config {
            package: MSP432_PACKAGE_LQFP,
        },
    );

    support_map.insert(
        String::from("msp432p401ripzr"),
        Msp432Config {
            package: MSP432_PACKAGE_LQFP,
        },
    );

    support_map.insert(
        String::from("msp432p401rirgcr"),
        Msp432Config {
            package: MSP432_PACKAGE_VQFN,
        },
    );

    support_map.insert(
        String::from("msp432p401rirgct"),
        Msp432Config {
            package: MSP432_PACKAGE_VQFN,
        },
    );

    support_map.insert(
        String::from("msp432p401rizxhr"),
        Msp432Config {
            package: MSP432_PACKAGE_NFBGA,
        },
    );

    support_map.insert(
        String::from("msp432p401rizxht"),
        Msp432Config {
            package: MSP432_PACKAGE_NFBGA,
        },
    );

    //TODO: Add more MSP432 variants.

    support_map
}
