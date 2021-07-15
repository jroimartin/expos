//! Helpers needed for parsing UEFI structures.

/// Builds a lookup table for the standard CRC32 algorithm using a seed
/// polynomial value of 0x04c11db7.
fn build_crc32_table() -> [u32; 256] {
    const BIT_REFLECTED_SEED: u32 = 0x04c11db7u32.reverse_bits();

    let mut table = [0u32; 256];

    for (i, item) in table.iter_mut().enumerate() {
        *item = i as u32;
        for _ in 0..8 {
            *item = if *item & 1 != 0 {
                (*item >> 1) ^ BIT_REFLECTED_SEED
            } else {
                *item >> 1
            };
        }
    }

    table
}

/// Returns the CRC32 checksum of the provided buffer.
pub unsafe fn crc32(ptr: *const u8, len: usize) -> u32 {
    let table = build_crc32_table();

    let mut crc = 0xffffffffu32;
    for off in 0..len {
        let b = core::ptr::read_unaligned(ptr.add(off));
        let idx = ((crc as u8) ^ b) as usize;
        crc = table[idx] ^ (crc >> 8);
    }
    crc ^ 0xffffffff
}

/// Returns the CRC32 checksum of the provided value.
pub unsafe fn crc32_for_value<T>(value: T) -> u32 {
    let ptr = &value as *const T as *const u8;
    let len = core::mem::size_of::<T>();
    crc32(ptr, len)
}

/// Returns the result of adding all the bytes of the provided buffer.
pub unsafe fn add_bytes(ptr: *const u8, len: usize) -> u8 {
    let mut checksum = 0u8;
    for off in 0..len {
        let b = core::ptr::read_unaligned(ptr.add(off));
        checksum = checksum.wrapping_add(b);
    }
    checksum
}
