//! UEFI parsing primitives.

#![no_std]

use core::ops::Deref;
use core::convert::{TryFrom, TryInto};

mod utils;

/// Represents an UEFI error.
#[derive(Debug)]
pub enum Error {
    /// The signature of the table does not match the expected one.
    InvalidSignature,

    /// The CRC32 checksum of the table does not match the expected one.
    InvalidCheckSum,

    /// Invalid status code conversion.
    InvalidStatusConversion,

    /// The returned status code is an error.
    StatusError(Status),
}

/// Represents an UEFI status code.
#[derive(Debug)]
pub enum Status {
    /// Success status code.
    Success,

    /// Warning status code.
    Warning(StatusWarning),

    /// Error status code.
    Error(StatusError),
}

/// Represents an UEFI warning status code.
#[derive(Debug)]
pub enum StatusWarning {
    /// The string contained one or more characters that the device could not
    /// render and were skipped.
    UnknownGlyph,

    /// The handle was closed, but the file was not deleted.
    DeleteFailure,

    /// The handle was closed, but the data to the file was not flushed
    /// properly.
    WriteFailure,

    /// The resulting buffer was too small, and the data was truncated to the
    /// buffer size.
    BufferTooSmall,

    /// The data has not been updated within the timeframe set by local policy
    /// for this type of data.
    StaleData,

    /// The resulting buffer contains UEFI-compliant file system.
    FileSystem,

    /// The operation will be processed across a system reset.
    ResetRequired,

    /// Unknown `EFI_STATUS` warning code.
    Unknown(usize),
}

impl TryFrom<usize> for StatusWarning {
    type Error = Error;

    fn try_from(status: usize) -> Result<Self, Self::Error> {
        // Code 0 is reserved for success and warnings must have the highest
        // bit clear.
        if status == 0 || status >> (usize::BITS - 1) == 1 {
            return Err(Error::InvalidStatusConversion);
        }

        match status {
            1 => Ok(StatusWarning::UnknownGlyph),
            2 => Ok(StatusWarning::DeleteFailure),
            3 => Ok(StatusWarning::WriteFailure),
            4 => Ok(StatusWarning::BufferTooSmall),
            5 => Ok(StatusWarning::StaleData),
            6 => Ok(StatusWarning::FileSystem),
            7 => Ok(StatusWarning::ResetRequired),
            _ => Ok(StatusWarning::Unknown(status)),
        }
    }
}

/// Represents an UEFI error status code.
#[derive(Debug)]
pub enum StatusError {
    /// The image failed to load.
    LoadError,

    /// A parameter was incorrect.
    InvalidParameter,

    /// The operation is not supported.
    Unsupported,

    /// The buffer was not the proper size for the request.
    BadBufferSize,

    /// The buffer is not large enough to hold the requested data. The required
    /// buffer size is returned in the appropriate parameter when this error
    /// occurs.
    BufferTooSmall,

    /// There is no data pending upon return.
    NotReady,

    /// The physical device reported anerror while attempting the operation.
    DeviceError,

    /// The device cannot be written to.
    WriteProtected,

    /// A resource has run out.
    OutOfResources,

    /// An inconstancy was detected on the file system causing the operating to
    /// fail.
    VolumeCorrupted,

    /// There is no more space on the file system.
    VolumeFull,

    /// The device does not contain any medium to perform the operation.
    NoMedia,

    /// The medium in the device has changed since the last access.
    MediaChanged,

    /// The item was not found.
    NotFound,

    /// Access was denied.
    ACcessDenied,

    /// The server was not found or did not respond to the request.
    NoResponse,

    /// A mapping to a device does not exist.
    NoMapping,

    /// The timeout time expired.
    Timeout,

    /// The protocol has not been started.
    NotStarted,

    /// The protocol has already been started.
    AlreadyStarted,

    /// The operation was aborted.
    Aborted,

    /// An ICMP error occurred during the network operation.
    IcmpError,

    /// A TFTP error occurred during the network operation.
    TftpError,

    /// A protocol error occurred during the network operation.
    ProtocolError,

    /// The function encountered an internal version that was incompatible with
    /// a version requested by the caller.
    IncompatibleVersion,

    /// The function was not performed due to a security violation.
    SecurityViolation,

    /// A CRC error was detected.
    CrcError,

    /// Beginning or end of media was reached.
    EndOfMedia,

    /// The end of the file was reached.
    EndOfFile,

    /// The language specified was invalid.
    InvalidLanguage,

    /// The security status of the data is unknown or compromised and the data
    /// must be updated or replaced to restore a valid security status.
    CompromisedData,

    /// There is an IP address conflict.
    IpAddressConflict,

    /// A HTTP error occurred during the network operation.
    HttpError,

    /// Unknown `EFI_STATUS` error code.
    Unknown(usize),
}

impl TryFrom<usize> for StatusError {
    type Error = Error;

    fn try_from(status: usize) -> Result<Self, Self::Error> {
        // Code 0 is reserved for success and errors must have the highest bit
        // set.
        if status == 0 || status >> (usize::BITS - 1) == 0 {
            return Err(Error::InvalidStatusConversion);
        }

        let error = match status & (usize::MAX >> 1) {
            1 => StatusError::LoadError,
            2 => StatusError::InvalidParameter,
            3 => StatusError::Unsupported,
            4 => StatusError::BadBufferSize,
            5 => StatusError::BufferTooSmall,
            6 => StatusError::NotReady,
            7 => StatusError::DeviceError,
            8 => StatusError::WriteProtected,
            9 => StatusError::OutOfResources,
            10 => StatusError::VolumeCorrupted,
            11 => StatusError::VolumeFull,
            12 => StatusError::NoMedia,
            13 => StatusError::MediaChanged,
            14 => StatusError::NotFound,
            15 => StatusError::ACcessDenied,
            16 => StatusError::NoResponse,
            17 => StatusError::NoMapping,
            18 => StatusError::Timeout,
            19 => StatusError::NotStarted,
            20 => StatusError::AlreadyStarted,
            21 => StatusError::Aborted,
            22 => StatusError::IcmpError,
            23 => StatusError::TftpError,
            24 => StatusError::ProtocolError,
            25 => StatusError::IncompatibleVersion,
            26 => StatusError::SecurityViolation,
            27 => StatusError::CrcError,
            28 => StatusError::EndOfMedia,
            31 => StatusError::EndOfFile,
            32 => StatusError::InvalidLanguage,
            33 => StatusError::CompromisedData,
            34 => StatusError::IpAddressConflict,
            35 => StatusError::HttpError,
            _ => StatusError::Unknown(status),
        };

        Ok(error)
    }
}

/// The `EFI_STATUS` type of the UEFI specification.
#[derive(Debug)]
#[repr(transparent)]
struct EfiStatus(usize);

impl From<EfiStatus> for Status {
    fn from(status: EfiStatus) -> Self {
        if status.0 == 0 {
            Status::Success
        } else if status.0 >> (usize::BITS - 1) == 0 {
            Status::Warning(status.0.try_into().unwrap())
        } else {
            Status::Error(status.0.try_into().unwrap())
        }
    }
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
#[repr(transparent)]
struct Ptr(usize);

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
pub struct EfiSystemTable {
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
/// services.
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
    ///     system_table: *const uefi::EfiSystemTable,
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
    /// The System Table is created using a raw pointer. Thus, this function is
    /// considered to be unsafe.
    pub unsafe fn new(ptr: *const EfiSystemTable) -> Result<Self, Error> {
        // Check table's signature.
        if (*ptr).hdr.signature != EFI_SYSTEM_TABLE_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Check table's CRC32.
        let mut system_table = core::ptr::read_unaligned(ptr);
        system_table.hdr.crc32 = 0;
        let crc32 = utils::crc32_for_value(system_table);
        if crc32 != (*ptr).hdr.crc32 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(SystemTable { ptr })
    }

    /// Returns the boot services.
    pub fn boot_services(&self) -> Result<BootServices, Error> {
        let efi_boot_services_ptr = unsafe { (*self.ptr).boot_services };

        // A `SystemTable` is only created after checking its signature
        // and CRC32. Thus, we assume the pointer to the Boot Services Table
        // will be valid.
        unsafe { BootServices::new(efi_boot_services_ptr) }
    }
}

/// The signature of an EFI Boot Services Table.
const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;

/// The `EFI_BOOT_SERVICES` type of the UEFI specification.
#[repr(C)]
pub struct EfiBootServices {
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
    get_next_monotonic_count: extern "C" fn(*mut u64) -> EfiStatus,
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

/// Represents the EFI Boot Services Table. It provides access to the boot
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
    /// The Boot Services Table is created using a raw pointer. Thus, this
    /// function is considered to be unsafe.
    pub unsafe fn new(ptr: *const EfiBootServices) -> Result<Self, Error> {
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

    /// Returns a monotonically increasing count for the platform.
    ///
    /// # Errors
    ///
    /// The function can return an error if the device is not functioning
    /// properly.
    pub fn get_next_monotonic_count(&self) -> Result<(u64, Status), Error> {
        let mut count = 0u64;
        let f = unsafe { (*self.ptr).get_next_monotonic_count };
        let status = f(&mut count);
        match status.into() {
            status @ Status::Success => Ok((count, status)),
            status @ Status::Warning(_) => Ok((count, status)),
            status @ Status::Error(_) => Err(Error::StatusError(status)),
        }
    }
}

/// The `EFI_CONFIGURATION_TABLE` type of the UEFI specification.
#[repr(C)]
struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: Ptr,
}
