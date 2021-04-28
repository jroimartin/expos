//! UEFI primitives.

/// Corresponds to the type `EFI_HANDLE` of the UEFI specification.
#[derive(Debug)]
#[repr(transparent)]
pub struct EfiHandle(usize);

/// Represents a pointer in an UEFI structure.
#[derive(Debug)]
#[repr(transparent)]
pub struct EfiPtr(usize);

/// Corresponds to the type `EFI_SYSTEM_TABLE` of the UEFI specification.
/// Provides access to UEFI Boot Services, UEFI Runtime Services, consoles,
/// firmware vendor information and the system configuration tables.
#[derive(Debug)]
#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: EfiPtr,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    pub cons_in: EfiPtr,
    pub console_out_handle: EfiHandle,
    pub cons_out: EfiPtr,
    pub standard_error_handle: EfiHandle,
    pub std_err: EfiPtr,
    pub runtime_services: EfiPtr,
    pub boot_services: EfiPtr,
    pub number_of_table_entries: usize,
    pub configuration_table: EfiPtr,
}

/// Corresponds to the type `EFI_TABLE_HEADER` of the UEFI specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}
