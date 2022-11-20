# PUN

## About

Originally known as Pugnix, PUN is a command-line tool for GNU/Linux. _Tested only on ArchLinux_.

> Created with the purpose of learning [Rust](https://www.rust-lang.org/).

## Building

### Dependencies

- [Sudo](https://www.sudo.ws/sudo/) or administrator access. ([Build script](#from-script-automatic));
- [Git](https://git-scm.com/);
- [Rust](https://www.rust-lang.org/).

### From script (Automatic)

**IMPORTANT:** Before running _any_ install/build script from the internet, you _should_ read and understand what it will do to your computer. [View script](build.sh)

I recommend you to read and modify the build script to your needs.

What will happen (by default):

- **The script will clone the repo**;
- Create/delete a temporary folder at _/tmp/_;
- Compile source with cargo (Rust utility);
- Send the binary file to _/usr/local/bin/_ folder.

### From source (Manual)

Clone this repo using Git.

```sh
git clone https://gitlab.com/devkcud/pun.git
cd ./pun/
```

Now, supposing you are on the cloned directory and have rust installed, you can run using (debug mode):

```sh
cargo run -q -- help
```

To compile:

```sh
cargo build --release
./target/release/pun help
```

## Updating script

> WIP

## License

This project uses [this license](./LICENSE).
