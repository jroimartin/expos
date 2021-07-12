#!/bin/sh

# Cargo UEFI configuration.
export CARGO_BUILD_TARGET='x86_64-unknown-uefi'
export CARGO_TARGET_X86_64_UNKNOWN_UEFI_RUNNER='tools/qemu-runner.sh'
export CARGO_UNSTABLE_BUILD_STD='core'
export CARGO_UNSTABLE_BUILD_STD_FEATURES='compiler-builtins-mem'

# Run Cargo.
exec cargo "$@"
