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
pub fn crc32<R: AsRef<[u8]>>(buf: R) -> u32 {
    let table = build_crc32_table();

    let crc = buf.as_ref().iter().fold(0xffffffff, |acc, b| {
        // We can safely use `idx` as the index of the lookup table because it
        // is a `u8` casted to `usize` and the size of the lookup table is 256.
        let idx = ((acc as u8) ^ b) as usize;
        table[idx] ^ (acc >> 8)
    });

    crc ^ 0xffffffff
}

/// Returns the CRC32 checksum of the provided value.
pub unsafe fn crc32_for_value<T>(value: T) -> u32 {
    let ptr = &value as *const T as *const u8;
    let len = core::mem::size_of::<T>();
    let buf = core::slice::from_raw_parts(ptr, len);
    crc32(buf)
}
