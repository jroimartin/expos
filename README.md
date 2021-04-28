# ExpOS

Tiny OS for experimentation.

## Dwarf info

In order to build the kernel with dwarf information, the following environment
variable must be set:

```
RUSTFLAGS='-C link-arg=/debug:dwarf'
```

So, we can use `cargo` this way:

```
RUSTFLAGS='-C link-arg=/debug:dwarf' cargo build
```
