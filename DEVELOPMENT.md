# OVERVIEW

remy's own compilation process is compatible with standard cargo. We wrap some common workflows with tinyrick tasks for convenience.

# BUILDTIME REQUIREMENTS

* [Rust](https://www.rust-lang.org/en-US/) 1.30+

## Recommended

* [clippy](https://github.com/rust-lang-nursery/rust-clippy)
* [tinyrick](https://github.com/mcandre/tinyrick) (e.g., `cargo install tinyrick`)
* [Docker](https://www.docker.com/)

# INSTALL BINARIES FROM SOURCE

```console
$ tinyrick install
```

# UNINSTALL BINARIES

```console
$ tinyrick uninstall
```

# BUILD: Doc, Lint, Test, and Compile

```console
$ tinyrick [build]
```

# PORT

```console
$ tinyrick port
```

# PUBLISH CRATE

```console
$ tinyrick publish
```

# PUBLISH DOCKER IMAGES

```console
$ tinyrick publish_docker
```

# CLEAN

```console
$ tinyrick clean
```
