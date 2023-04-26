# Fibber (COMPLETE)

---
> _Fibber_ is a command line program that calculates and prints the [fibonacci Sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence) in a customizable fasion.
---

## Summary of development

The name "fibber" was chosen for two reasons:

1) using `_` or `-` can often lead to unneccary confusion & issues in executable names. (e.g., "fibonacci_gen")
2) i believe having a product name is important, even if it's cheesy :)

The design of the codebase is dynamic, implimenting both Object-Oriented-Programming principals, and
Functionally-Oriented-Programming principals, each where is most efficient in a modern programming setting.

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

## Limitations

`--count` must be <= 186, as anything greater will cause the fibonacci numbers generated to exceed
the allocation size of of a unsigned 128-bit integer (unsigned long long)

---
---

## examples

output of running `fibber --help`:

```console
Fibber v0.1.0:


Usage: fibber [OPTIONS] --count <COUNT>

Options:
  -v, --verbose        Explicitly set the log level to verbose
  -q, --quiet          Explicitly set the log level to quiet (overrides verbose)
  -c, --count <COUNT>  the length of the sequence to output, i.e.: 0, 1, 1, 2, 3, 5 would be the result of -c 6
  -1, --one-line       Print on one line
      --numbering      Preface each number in the sequence with it's position within the sequence, i.e: 1:0, 2:1, 3:1, 4:2, 5:3, 6:5
      --last-only      Print only the last number of the sequence for the given count
  -h, --help           Print help
  -V, --version        Print version
```

## Screenshots

### no args

![no args](docs/Screenshot%202023-04-26%20105702.png)
---

### count 6, numbering, last-only

![count 6, numbering, last-only](docs/Screenshot%202023-04-26%20110046.png)
---

### count 13

![count 13](docs/Screenshot%202023-04-26%20110226.png)
---
