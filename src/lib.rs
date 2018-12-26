//! Porting defaults

extern crate regex;

use regex::Regex;
use std::env;
use std::fs;
use std::path;
use std::process;

/// A Platform details a Docker and cargo build target configuration for building Rust applications
pub struct Platform {
    pub image_tag : String,
    pub target : String,
    pub features : String
}

lazy_static::lazy_static! {
    pub static ref PLATFORMS : Vec<Platform> = vec![
        Platform{
            image_tag: "x86_64-unknown-linux-gnu".to_string(),
            target: "x86_64-unknown-linux-gnu".to_string(),
            features: "".to_string()
        },
        Platform{
            image_tag: "x86_64-unknown-linux-gnu".to_string(),
            target: "i686-unknown-linux-gnu".to_string(),
            features: "".to_string()
        },
        Platform{
            image_tag: "arm-unknown-linux-gnueabi".to_string(),
            target: "arm-unknown-linux-gnueabi".to_string(),
            features: "".to_string()
        },
        Platform{
            image_tag: "x86_64-unknown-cloudabi".to_string(),
            target: "x86_64-unknown-cloudabi".to_string(),
            features: "cloudlibc".to_string()
        },
    ];
}

/// PortConfig parameterizes a port job
pub struct PortConfig {
    pub image : String,
    pub is_release : bool,
    pub is_verbose : bool,
    pub target_exclusions : Vec<Regex>
}

/// Build a default PortConfig
pub fn new_portconfig() -> PortConfig {
    PortConfig{
        image: "mcandre/remy".to_string(),
        is_release: true,
        is_verbose: false,
        target_exclusions: vec![]
    }
}

/// Supply the standard executable suffix for a given target environment
pub fn executable_suffix(target : &str) -> &str {
    if target.contains("windows") {
        ".exe"
    } else {
        ""
    }
}

/// Generate application ports based on a PortConfig and a collection of binary basenames
pub fn port(portconfig : &PortConfig, binaries : Vec<String>) {
    for binary in binaries {
        for platform in PLATFORMS.iter() {
            let target : &str = &platform.target;

            if portconfig.target_exclusions.iter().any(|exclusion| exclusion.is_match(target)) {
                continue;
            }

            let tagged_image : &str = &format!("{}:{}", portconfig.image, platform.image_tag);

            let cur_dir_canonical_pathbuf : path::PathBuf = env::current_dir()
                .unwrap()
                .as_path()
                .canonicalize()
                .unwrap();

            let cur_dir_canonical_str : &str = cur_dir_canonical_pathbuf
                .as_path()
                .to_str()
                .unwrap();

            let mount_join : &str = &format!(
                "{}:/src",
                cur_dir_canonical_str
            );

            let release_flag = if portconfig.is_release {
                " --release"
            } else {
                ""
            };

            let feature_flag = if platform.features != "" {
                format!(" --features {}", platform.features)
            } else {
                "".to_string()
            };

            let build_command : &str = &format!(
                "cd /src && cargo build --target {} --bin {}{}{}",
                target,
                binary,
                release_flag,
                feature_flag
            );

            let command_bare : &mut process::Command = &mut process::Command::new("docker");

            let command : &mut process::Command = command_bare
                .args(&[
                    "run",
                    "--rm",
                    "-v", mount_join,
                    tagged_image,
                    "sh", "-c", build_command
                ]);

            if portconfig.is_verbose {
                println!("{:?}", command);
            }

            assert!(
                command
                    .status()
                    .unwrap()
                    .success()
            );

            let build_mode = if portconfig.is_release {
                "release"
            } else {
                "debug"
            };

            let binary_filename : &str = &format!("{}{}", binary, executable_suffix(target));

            let port_path = path::Path::new("target")
                .join(target)
                .join(build_mode)
                .join(binary_filename);

            let bin_target = path::Path::new("target")
                .join("bin")
                .join(target);

            let merged_port_path = bin_target
                .join(binary_filename);

            fs::create_dir_all(bin_target)
                .unwrap();
            fs::copy(port_path, merged_port_path)
                .unwrap();
        }
    }
}
