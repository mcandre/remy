//! Build configuration

extern crate regex;
extern crate remy;
extern crate tinyrick;
extern crate tinyrick_extras;

use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::path;

/// Generate documentation
fn doc() {
    tinyrick_extras::doc();
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
}

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
}

/// Install binaries
fn install() {
    tinyrick_extras::install_binaries();
}

/// Uninstall binaries
fn uninstall() {
    tinyrick_extras::uninstall_binaries();
}

/// Build Docker images
fn images() {
    for platform in remy::PLATFORMS.iter() {
        let title : &str = &format!("mcandre/remy:{}", platform.image_tag);

        let dockerfile_str : &str = &format!("{}.Dockerfile", platform.image_tag);

        let current_directory_pathbuf : path::PathBuf = env::current_dir()
            .unwrap();

        let current_directory_str : &str = current_directory_pathbuf
            .as_path()
            .to_str()
            .unwrap();

        tinyrick::exec!(
            "docker",
            &[
                "build",
                "-t", title,
                "-f", dockerfile_str,
                current_directory_str
            ]
        );
    }
}

/// Doc, lint, and run tests
fn test() {
    tinyrick::deps(lint);
    tinyrick::deps(install);
    tinyrick::deps(images);

    assert!(
        tinyrick::exec_mut!("remy", &["hello"])
            .current_dir("example")
            .env("VERBOSE", "1")
            .status()
            .unwrap()
            .success()
    );
}

/// Doc, lint, test, and compile
fn build() {
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Publish Docker images
fn publish_docker() {
    for platform in remy::PLATFORMS.iter() {
        let title : &str = &format!("mcandre/remy:{}", platform.image_tag);

        tinyrick::exec!(
            "docker",
            &[
                "push",
                title
            ]
        );
    }
}

/// Generate application ports for remy itself
fn port() {
    tinyrick::deps(test);

    let binaries = vec!["remy"];

    let target_exclusions_str = vec!["cloudabi"];

    let args_vec : &mut Vec<&str> = &mut target_exclusions_str
        .iter()
        .flat_map(|target_exclusion_str| vec!["-e", target_exclusion_str])
        .collect();
    args_vec.extend(&binaries);

    tinyrick::exec!("remy", &args_vec[..]);

    let banner : &str = &format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let archive_path : &str = &format!("{}.zip", banner);

    let zip_file : fs::File = fs::File::create(archive_path).unwrap();
    let zip_writer : &mut zip::ZipWriter<fs::File> = &mut zip::ZipWriter::new(zip_file);
    let file_options : zip::write::FileOptions = zip::write::FileOptions::default();

    let target_exclusions : Vec<Regex> = target_exclusions_str
        .iter()
        .map(|target_exclusion_str| Regex::new(target_exclusion_str).unwrap())
        .collect();

    for binary in binaries.iter() {
        for platform in remy::PLATFORMS.iter() {
            let target = &platform.target;

            if target_exclusions.iter().any(|exclusion| exclusion.is_match(target)) {
                continue;
            }

            let suffix = remy::executable_suffix(target);
            let binary_filename = &format!("{}{}", binary, suffix);

            let source_pathbuf : path::PathBuf = path::Path::new("target")
                .join("bin")
                .join(target)
                .join(binary_filename);

            let source_path_str : &str = source_pathbuf
                .to_str()
                .unwrap();

            let destination_path : path::PathBuf = path::Path::new(banner)
                .join(target)
                .join(binary_filename);

            let destination_path_str : &str = destination_path
                .to_str()
                .unwrap();

            let entry_file : &mut fs::File = &mut fs::File::open(source_path_str).unwrap();
            zip_writer.start_file(destination_path_str, file_options).unwrap();
            io::copy(entry_file, zip_writer).unwrap();
        }
    }
}

fn clean_port() {
    match glob::glob("*.zip") {
        Ok(path_results) => {
            for p in path_results.flat_map(|p_res| p_res) {
                fs::remove_file(p).is_ok();
            }
        }
        _ => {}
    }
}

/// Remove cargo target directory
fn clean_target() {
    let target_path = path::Path::new("example")
        .join("target");

    fs::remove_dir_all(target_path).is_ok();
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_port);
    tinyrick::deps(clean_target);
    tinyrick_extras::clean_cargo();
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        doc,
        clippy,
        lint,
        install,
        uninstall,
        images,
        test,
        publish,
        publish_docker,
        port,
        clean_port,
        clean_target,
        clean
    );
}
