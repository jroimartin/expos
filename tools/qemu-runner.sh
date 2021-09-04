#!/bin/sh

# Exit on error or unset variable.
set -e -u

# Parse command line arguments.
if [ $# -ne 1 ]; then
	echo "usage: $0 <efi_bin>" >&2
	exit 1
fi
efi_bin=$1

# TFTP parameters for QEMU.
tftp_dir=$(dirname "${efi_bin}")
tftp_bootfile=$(basename "${efi_bin}")

# Run QEMU downloading the kernel via tftp.
qemu-system-x86_64 \
	-nodefaults \
	-nographic \
	-smp 'cores=4' \
	-serial mon:stdio \
	-m 1024 \
	-bios '/usr/share/ovmf/OVMF.fd' \
	-device 'e1000,netdev=n0' \
	-netdev "user,id=n0,tftp=${tftp_dir},bootfile=${tftp_bootfile}"
