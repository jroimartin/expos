# ExpOS

Tiny OS for experimentation.

## Build

Building the kernel requires to set up some specific Cargo configurations. The
script `/tools/cargo-uefi.sh` takes care of it. This script it is just a Cargo
wrapper, so it accepts the same sub-commands and arguments.

Run the following command to build the kernel:

```
./tools/cargo-uefi.sh build
```

### Dwarf info

In order to build the kernel with dwarf information, the following extra
command-line flag must be passed to rustc:

```
-C link-arg=/debug:dwarf
```

This can be done by setting the `RUSTFLAGS` env var. For example:

```
RUSTFLAGS='-C link-arg=/debug:dwarf' ./tools/cargo-uefi.sh build
```

## Run in QEMU

Run the following command to run the kernel in QEMU:

```
./tools/cargo-uefi.sh run
```

## Test

Run the following command to run the tests for a specific package:

```
cargo test -p <package>
```

Take into account that this command will fail for the `expos` package.
