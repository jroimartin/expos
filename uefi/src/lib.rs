//! UEFI parsing primitives.

#![no_std]

/// The type `EFI_HANDLE` of the UEFI specification.
#[derive(Debug)]
#[repr(transparent)]
pub struct EfiHandle(pub usize);

/// Represents a pointer in an UEFI structure.
#[derive(Debug)]
#[repr(transparent)]
pub struct EfiPtr(pub usize);

/// The type `EFI_GUID` of the UEFI specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The type `EFI_TABLE_HEADER` of the UEFI specification.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

/// The type `EFI_SYSTEM_TABLE` of the UEFI specification. Provides access to
/// UEFI Boot Services, UEFI Runtime Services, consoles, firmware vendor
/// information and the system configuration tables.
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

/// The type `EFI_BOOT_SERVICES` of the UEFI specification. It contains
/// pointers to all of the boot services.
#[derive(Debug)]
#[repr(C)]
pub struct EfiBootServices {}

#[derive(Debug)]
#[repr(C)]
pub struct EfiConfigurationTable {
    pub vendor_guid: EfiGuid,
    pub vendor_table: EfiPtr,
}
