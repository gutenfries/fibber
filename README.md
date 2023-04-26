# Fibber

---
> _Fibber_ is a command line program that calculates and prints the [Fibbinacci Sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence) in a customizable fasion.
---

## Summary of development

The name "fibber" was chosen for two reasons:

1) using `_` or `-` can often lead to unneccary confusion & issues in executable names. (e.g., "fibbinacci_gen")
2) i believe having a product name is important, even if it's cheesy :)

The design of the codebase is dynamic, implimenting both Object-Oriented-Programming principals, and
Functionally-Oriented-Programming principals.

## Running

### From Docker

Fibber can be run from a contained docker image, with no external dependencies.

To build and run the docker image, run the following in your shell of choice:
_(assuming you have [Docker](https://docs.docker.com/get-docker/) installed on your system)_

```bash
docker build -t fibber . # build the image
docker run -it --rm --name fibber-instance fibber # run an anstance of the image
```

any desired args/flags can simply be appended to the `run` command., i.e.:

```bash
docker run -it --rm --name fibber-instance fibber {args_go_here}
```

> NOTE: the first time you build, the build may take ~2 minutes, this is normal and for optimization purposes

### From Rust Compiler

If you have [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system,
you can simply compile and run the application with ne external dependencies.

run in root of repo:

```bash
cargo run -- <args/flags here>
```

the `--` is neccesary if running from the compiler cli, else args/flags will be passed to the compiler.

### From raw executable

It is unadviasable in development, however you may run fibber directly from the executable provided by the compiler.
the executable is avilable at `{repo_root}/target/{debug/release}/fibber{.exe}`

To compile, run:

```bash
cargo build
```

In this case, args can simply be appended to the path of the executable, as is standard.

## Usage

<!-- TODO -->
