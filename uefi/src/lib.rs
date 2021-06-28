//! UEFI parsing primitives.

#![no_std]

use core::convert::{TryFrom, TryInto};
use core::ops::Deref;

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
    StatusError(StatusError),
}

/// The `EFI_STATUS` type of the UEFI specification.
#[derive(Debug)]
#[repr(transparent)]
struct EfiStatus(usize);

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

impl TryFrom<EfiStatus> for StatusWarning {
    type Error = Error;

    fn try_from(status: EfiStatus) -> Result<Self, Self::Error> {
        // Code 0 is reserved for success and warnings must have the highest
        // bit clear.
        if status.0 == 0 || status.0 >> (usize::BITS - 1) == 1 {
            return Err(Error::InvalidStatusConversion);
        }

        match status.0 {
            1 => Ok(StatusWarning::UnknownGlyph),
            2 => Ok(StatusWarning::DeleteFailure),
            3 => Ok(StatusWarning::WriteFailure),
            4 => Ok(StatusWarning::BufferTooSmall),
            5 => Ok(StatusWarning::StaleData),
            6 => Ok(StatusWarning::FileSystem),
            7 => Ok(StatusWarning::ResetRequired),
            code => Ok(StatusWarning::Unknown(code)),
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
    AccessDenied,

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

impl TryFrom<EfiStatus> for StatusError {
    type Error = Error;

    fn try_from(status: EfiStatus) -> Result<Self, Self::Error> {
        // Code 0 is reserved for success and errors must have the highest bit
        // set.
        if status.0 == 0 || status.0 >> (usize::BITS - 1) == 0 {
            return Err(Error::InvalidStatusConversion);
        }

        let error = match status.0 & (usize::MAX >> 1) {
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
            15 => StatusError::AccessDenied,
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
            _ => StatusError::Unknown(status.0),
        };

        Ok(error)
    }
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

impl From<EfiStatus> for Status {
    fn from(status: EfiStatus) -> Self {
        if status.0 == 0 {
            Status::Success
        } else if status.0 >> (usize::BITS - 1) == 0 {
            Status::Warning(status.try_into().unwrap())
        } else {
            Status::Error(status.try_into().unwrap())
        }
    }
}

/// An UEFI operation will return one of the following status codes: Success,
/// Warning and Error. This type represents them along with the returned result
/// for operations finishing in Success or Warning.
#[derive(Debug)]
pub enum StatusResult<T> {
    Ok(T),
    Warn(T, StatusWarning),
    Err(StatusError),
}

impl<T> From<(T, Status)> for StatusResult<T> {
    fn from(result_status: (T, Status)) -> Self {
        match result_status.1 {
            Status::Success => StatusResult::Ok(result_status.0),
            Status::Warning(status) => {
                StatusResult::Warn(result_status.0, status)
            }
            Status::Error(status) => StatusResult::Err(status),
        }
    }
}

/// Represents an UEFI handle. It is equivalent to the `EFI_HANDLE` type of the
/// UEFI specification.
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
struct EfiPtr(usize);

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
    firmware_vendor: EfiPtr,
    firmware_revision: u32,
    console_in_handle: Handle,
    cons_in: EfiPtr,
    console_out_handle: Handle,
    cons_out: EfiPtr,
    standard_error_handle: Handle,
    std_err: EfiPtr,
    runtime_services: EfiPtr,
    boot_services: *const EfiBootServices,
    number_of_table_entries: usize,
    configuration_table: EfiPtr,
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

/// Represents a physical memory address. It is equivalent to the
/// `EFI_PHYSICAL_ADDRESS` type of the UEFI specification.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct PhysAddr(u64);

impl Deref for PhysAddr {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a virtual memory address. It is equivalent to the
/// `EFI_VIRTUAL_ADDRESS` type of the UEFI specification.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl Deref for VirtAddr {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The `EFI_MEMORY_TYPE` type of the UEFI specification.
#[repr(transparent)]
struct EfiMemoryType(u32);

/// The type of memory.
#[derive(Debug, Copy, Clone)]
pub enum MemoryType {
    /// Not usable memory.
    ReservedMemoryType,

    /// The UEFI OS Loader and/or OS may use this memory as they see fit.
    LoaderCode,

    /// The UEFI OS Loader and/or OS may use this memory as they see fit.
    LoaderData,

    /// Memory available for general use.
    BootServicesCode,

    /// Memory available for general use.
    BootServicesData,

    /// The memory in this range is to be preserved by the UEFI OS loader and
    /// OS in the working and ACPI S1-S3 states.
    RuntimeServicesCode,

    /// The memory in this range is to be preserved by the UEFI OS loader and
    /// OS in the working and ACPI S1-S3 states.
    RuntimeServicesData,

    /// Memory available for general use.
    ConventionalMemory,

    /// Memory that contains errors and is not to be used.
    UnusableMemory,

    /// This memory is to be preserved by the UEFI OS loader and OS until ACPI
    /// is enabled. Once ACPI is enabled, the memory in this range is available
    /// for general use.
    ACPIReclaimMemory,

    /// This memory is to be preserved by the UEFI OS loader and OS in the
    /// working and ACPI S1-S3 states.
    ACPIMemoryNVS,

    /// This memory is not used by the OS. All system memory-mapped IO
    /// information should come from ACPI tables.
    MemoryMappedIO,

    /// This memory is not used by the OS. All system memory-mapped IO port
    /// space information should come from ACPI tables.
    MemoryMappedIOPortSpace,

    /// This memory is to be preserved by the UEFI OS loader and OS in the
    /// working and ACPI S1-S4 states. This memory may also have other
    /// attributes that are defined by the processor implementation.
    PalCode,

    /// A memory region that operates as `ConventionalMemory`. However, it
    /// happens to also support byte-addressable non-volatility.
    PersistentMemory,

    /// A memory region that represents unaccepted memory, that must be
    /// accepted by the boot target before it can be used. Unless otherwise
    /// noted, all other EFI memory types are accepted. For platforms that
    /// support unaccepted memory, all unaccepted valid memory will be reported
    /// as unaccepted in the memory map. Unreported physical address ranges
    /// must be treated as not-present memory.
    UnacceptedMemoryType,

    /// Unknown memory type.
    Unknown(u32),
}

impl From<EfiMemoryType> for MemoryType {
    fn from(mem_type: EfiMemoryType) -> Self {
        match mem_type.0 {
            0 => MemoryType::ReservedMemoryType,
            1 => MemoryType::LoaderCode,
            2 => MemoryType::LoaderData,
            3 => MemoryType::BootServicesCode,
            4 => MemoryType::BootServicesData,
            5 => MemoryType::RuntimeServicesCode,
            6 => MemoryType::RuntimeServicesData,
            7 => MemoryType::ConventionalMemory,
            8 => MemoryType::UnusableMemory,
            9 => MemoryType::ACPIReclaimMemory,
            10 => MemoryType::ACPIMemoryNVS,
            11 => MemoryType::MemoryMappedIO,
            12 => MemoryType::MemoryMappedIOPortSpace,
            13 => MemoryType::PalCode,
            14 => MemoryType::PersistentMemory,
            15 => MemoryType::UnacceptedMemoryType,
            ty => MemoryType::Unknown(ty),
        }
    }
}

/// The `EFI_MEMORY_DESCRIPTOR` type of the UEFI specification.
#[repr(C)]
struct EfiMemoryDescriptor {
    memory_type: EfiMemoryType,
    physical_start: PhysAddr,
    virtual_start: VirtAddr,
    number_of_pages: u64,
    attribute: u64,
}

/// Describes a contiguous block of memory.
#[derive(Debug, Copy, Clone)]
pub struct MemoryDescriptor {
    memory_type: MemoryType,
    physical_start: PhysAddr,
    virtual_start: VirtAddr,
    number_of_pages: u64,
    attribute: u64,
}

impl From<EfiMemoryDescriptor> for MemoryDescriptor {
    fn from(mem_desc: EfiMemoryDescriptor) -> Self {
        MemoryDescriptor {
            memory_type: mem_desc.memory_type.into(),
            physical_start: mem_desc.physical_start,
            virtual_start: mem_desc.virtual_start,
            number_of_pages: mem_desc.number_of_pages,
            attribute: mem_desc.attribute,
        }
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

/// The `EFI_CONFIGURATION_TABLE` type of the UEFI specification.
#[repr(C)]
struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: EfiPtr,
}

/// The signature of an EFI Boot Services Table.
const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;

/// Fixed length of the memory map.
const MEMORY_MAP_LEN: usize = 128;

/// The `EFI_BOOT_SERVICES` type of the UEFI specification.
#[repr(C)]
pub struct EfiBootServices {
    hdr: EfiTableHeader,

    // Task priority services.
    raise_tpl: EfiPtr,
    restore_tpl: EfiPtr,

    // Memory services.
    allocate_pages: EfiPtr,
    free_pages: EfiPtr,
    get_memory_map: extern "C" fn(
        *mut usize,
        *mut u8,
        *mut usize,
        *mut usize,
        *mut u32,
    ) -> EfiStatus,
    allocate_pool: EfiPtr,
    free_pool: EfiPtr,

    // Event & timer services.
    create_event: EfiPtr,
    set_timer: EfiPtr,
    wait_for_event: EfiPtr,
    signal_event: EfiPtr,
    close_event: EfiPtr,
    check_event: EfiPtr,

    // Protocol handler services.
    install_protocol_interface: EfiPtr,
    reinstall_protocol_interface: EfiPtr,
    uninstall_protocol_interface: EfiPtr,
    handle_protocol: EfiPtr,
    reserved: EfiPtr,
    register_protocol_notify: EfiPtr,
    locate_handle: EfiPtr,
    locate_device_path: EfiPtr,
    install_configuration_table: EfiPtr,

    // Image services.
    load_image: EfiPtr,
    start_image: EfiPtr,
    exit: EfiPtr,
    unload_image: EfiPtr,
    exit_boot_services: EfiPtr,

    // Miscelaneous services.
    get_next_monotonic_count: extern "C" fn(*mut u64) -> EfiStatus,
    stall: EfiPtr,
    set_watchdog_timer: EfiPtr,

    // DriverSupport services.
    connect_controller: EfiPtr,
    disconnect_controller: EfiPtr,

    // Open and close protocol services.
    open_protocol: EfiPtr,
    close_protocol: EfiPtr,
    open_protocol_information: EfiPtr,

    // Library services.
    protocols_per_handle: EfiPtr,
    locate_handle_buffer: EfiPtr,
    locate_protocol: EfiPtr,
    install_multiple_protocol_interfaces: EfiPtr,
    uninstall_multiple_protocol_interfaces: EfiPtr,

    // 32-bit CRC services.
    calculate_crc32: EfiPtr,

    // Miscelaneous services.
    copy_mem: EfiPtr,
    set_mem: EfiPtr,
    create_event_ex: EfiPtr,
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
    pub fn get_next_monotonic_count(&self) -> StatusResult<u64> {
        let mut count = 0u64;
        let service = unsafe { (*self.ptr).get_next_monotonic_count };
        let status = service(&mut count);

        StatusResult::from((count, status.into()))
    }

    /// Returns the current memory map and its map key. The map is an array of
    /// memory descriptors, each of which describes a contiguous block of
    /// memory.
    pub fn get_memory_map(
        &self,
    ) -> StatusResult<([Option<MemoryDescriptor>; MEMORY_MAP_LEN], usize)>
    {
        const BUFFER_SIZE: usize = 4096;

        // Allocate the arguments of the boot service.
        let mut memory_map_size = BUFFER_SIZE;
        let mut memory_map = [0u8; BUFFER_SIZE];
        let mut map_key = 0usize;
        let mut descriptor_size = 0usize;
        let mut descriptor_version = 0u32;

        // Call the boot service.
        let service = unsafe { (*self.ptr).get_memory_map };
        let status = service(
            &mut memory_map_size,
            memory_map.as_mut_ptr(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );

        // Fill the array to be returned.
        let mut ret = [None; MEMORY_MAP_LEN];
        let mut idx = 0;
        while (idx + 1) * descriptor_size <= memory_map_size {
            if idx >= MEMORY_MAP_LEN {
                return StatusResult::Err(StatusError::BufferTooSmall);
            }

            let descriptor = unsafe {
                let descriptor_ptr =
                    memory_map.as_ptr().add(idx * descriptor_size)
                        as *const EfiMemoryDescriptor;
                core::ptr::read(descriptor_ptr)
            };

            ret[idx] = Some(descriptor.into());

            idx += 1;
        }

        StatusResult::from(((ret, map_key), status.into()))
    }
}
