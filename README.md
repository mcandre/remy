# remy: *sous-chef de métro*

![pizza rat](https://raw.githubusercontent.com/mcandre/remy/master/remy.gif)

# EXAMPLE

```console
$ cd example

$ remy hello

$ tree target/bin
target/bin
├── arm-unknown-linux-gnueabi
│   └── hello
├── i686-unknown-linux-gnu
│   └── hello
└── x86_64-unknown-linux-gnu
    └── hello
```

See `remy -h` for more details.

# ABOUT

remy quickly generates Rust ports for multiple target environments, by running your Rust project through an array of Docker containers for different target toolchains.

# RUNTIME REQUIREMENTS

* [Cargo](https://www.rust-lang.org/en-US/)
* [Docker](https://www.docker.com/)

## Recommended

* [tree](https://linux.die.net/man/1/tree)

# INSTALL

```console
$ cargo install remy
```

# USAGE

1. Define any custom linkers in `.cargo/config`. See the included [example](example) project for more information.
2. Run `remy` with your project's binary names as arguments.
3. remy products ports in the structure `target/bin/<environment>/<binary>`.

# CONTRIBUTING

For more details on developing remy itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
