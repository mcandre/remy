//! Build configuration

extern crate remy;
extern crate tinyrick;
extern crate tinyrick_extras;

use std::env;
use std::fs;
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

    let dockerfile_str : &str = &format!("{}.Dockerfile", platform.target);

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
    tinyrick::exec_mut!("remy")
      .args(&["hello"])
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

/// Remove cargo target directory
fn clean_target() {
  let target_path = path::Path::new("example")
    .join("target");

  fs::remove_dir_all(target_path).unwrap();
}

/// Clean workspaces
fn clean() {
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
    clean_target,
    clean
  );
}
