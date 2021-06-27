//! UEFI parsing primitives.

#![no_std]

use core::ops::Deref;

mod utils;

/// Represents an UEFI error.
#[derive(Debug)]
pub enum Error {
    /// The signature of the table does not match the expected one.
    InvalidSignature,

    /// The CRC32 checksum of the table does not match the expected one.
    InvalidCheckSum,

    /// Unknown UEFI Error.
    Unknown(u32),
}

/// Represents an UEFI handle.
#[derive(Debug)]
#[repr(transparent)]
pub struct Handle(usize);

impl Deref for Handle {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a generic pointer.
#[derive(Debug)]
#[repr(transparent)]
pub struct Ptr(usize);

impl Deref for Ptr {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The `EFI_GUID` type of the UEFI specification.
#[repr(C)]
struct EfiGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

/// The `EFI_TABLE_HEADER` type of the UEFI specification.
#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

/// The signature of an `EFI_SYSTEM_TABLE`.
const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;

/// The `EFI_SYSTEM_TABLE` type of the UEFI specification.
#[repr(C)]
struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: Ptr,
    firmware_revision: u32,
    console_in_handle: Handle,
    cons_in: Ptr,
    console_out_handle: Handle,
    cons_out: Ptr,
    standard_error_handle: Handle,
    std_err: Ptr,
    runtime_services: Ptr,
    boot_services: *const EfiBootServices,
    number_of_table_entries: usize,
    configuration_table: Ptr,
}

/// Represents the EFI System Table. It provides access to the boot and runtime
/// Services.
pub struct SystemTable {
    ptr: *const EfiSystemTable,
}

impl SystemTable {
    /// Creates a new `SystemTable` from a given `ptr`. Usually this pointer
    /// is the `system_table` argument of the UEFI entry point.
    ///
    /// ```ignore
    /// extern "C" fn efi_main(
    ///     image_handler: uefi::Handle,
    ///     system_table: uefi::Ptr,
    /// ) -> ! {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// If the signature or the CRC32 of the table do not match the expected
    /// values the function will return an error.
    ///
    /// # Safety
    ///
    /// The system table is created using an arbitry pointer. Thus, this
    /// function is considered to be unsafe.
    pub unsafe fn new(ptr: Ptr) -> Result<SystemTable, Error> {
        let efi_system_table_ptr = *ptr as *const EfiSystemTable;

        // Check table's signature.
        if (*efi_system_table_ptr).hdr.signature != EFI_SYSTEM_TABLE_SIGNATURE
        {
            return Err(Error::InvalidSignature);
        }

        // Check table's CRC32.
        let mut system_table = core::ptr::read_unaligned(efi_system_table_ptr);
        system_table.hdr.crc32 = 0;
        let crc32 = utils::crc32_for_value(system_table);
        if crc32 != (*efi_system_table_ptr).hdr.crc32 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(SystemTable {
            ptr: efi_system_table_ptr,
        })
    }

    /// Returns the Boot Services.
    pub fn boot_services(&self) -> Result<BootServices, Error> {
        let efi_boot_services_ptr = unsafe { (*self.ptr).boot_services };

        // A `SystemTable` is only created after checking its signature
        // and CRC32. Thus, we assume the pointer to the Boot Services Table
        // will be valid.
        unsafe { BootServices::new(efi_boot_services_ptr) }
    }
}

/// The signature of an EFI Boot Services table.
const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;

/// The `EFI_BOOT_SERVICES` type of the UEFI specification.
#[repr(C)]
struct EfiBootServices {
    hdr: EfiTableHeader,

    // Task priority services.
    raise_tpl: Ptr,
    restore_tpl: Ptr,

    // Memory services.
    allocate_pages: Ptr,
    free_pages: Ptr,
    get_memory_map: Ptr,
    allocate_pool: Ptr,
    free_pool: Ptr,

    // Event & timer services.
    create_event: Ptr,
    set_timer: Ptr,
    wait_for_event: Ptr,
    signal_event: Ptr,
    close_event: Ptr,
    check_event: Ptr,

    // Protocol handler services.
    install_protocol_interface: Ptr,
    reinstall_protocol_interface: Ptr,
    uninstall_protocol_interface: Ptr,
    handle_protocol: Ptr,
    reserved: Ptr,
    register_protocol_notify: Ptr,
    locate_handle: Ptr,
    locate_device_path: Ptr,
    install_configuration_table: Ptr,

    // Image services.
    load_image: Ptr,
    start_image: Ptr,
    exit: Ptr,
    unload_image: Ptr,
    exit_boot_services: Ptr,

    // Miscelaneous services.
    get_next_monotonic_count: extern "C" fn(*mut u64) -> u32,
    stall: Ptr,
    set_watchdog_timer: Ptr,

    // DriverSupport services.
    connect_controller: Ptr,
    disconnect_controller: Ptr,

    // Open and close protocol services.
    open_protocol: Ptr,
    close_protocol: Ptr,
    open_protocol_information: Ptr,

    // Library services.
    protocols_per_handle: Ptr,
    locate_handle_buffer: Ptr,
    locate_protocol: Ptr,
    install_multiple_protocol_interfaces: Ptr,
    uninstall_multiple_protocol_interfaces: Ptr,

    // 32-bit CRC services.
    calculate_crc32: Ptr,

    // Miscelaneous services.
    copy_mem: Ptr,
    set_mem: Ptr,
    create_event_ex: Ptr,
}

/// Represents the EFI Boot Services table. It provides access to the boot
/// services.
pub struct BootServices {
    ptr: *const EfiBootServices,
}

impl BootServices {
    /// Creates a new `BootServices` from a given `ptr`.
    ///
    /// # Errors
    ///
    /// If the signature or the CRC32 of the table do not match the expected
    /// values the function will return an error.
    ///
    /// # Safety
    ///
    /// The boot services table is created using an arbitry pointer. Thus, this
    /// function is considered to be unsafe.
    unsafe fn new(ptr: *const EfiBootServices) -> Result<BootServices, Error> {
        // Check table's signature.
        if (*ptr).hdr.signature != EFI_BOOT_SERVICES_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Check table's CRC32.
        let mut boot_services_table = core::ptr::read_unaligned(ptr);
        boot_services_table.hdr.crc32 = 0;
        let crc32 = utils::crc32_for_value(boot_services_table);
        if crc32 != (*ptr).hdr.crc32 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(BootServices { ptr })
    }

    /// TODO(rm): Example function.
    pub fn get_next_monotonic_count(&self) -> Result<u64, Error> {
        let mut count = 0u64;
        let f = unsafe { (*self.ptr).get_next_monotonic_count };
        match f(&mut count) {
            0 => Ok(count),
            ret => Err(Error::Unknown(ret)),
        }
    }
}

/// The `EFI_CONFIGURATION_TABLE` type of the UEFI specification.
#[repr(C)]
struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: Ptr,
}
