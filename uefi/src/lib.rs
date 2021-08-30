//! UEFI parsing primitives.

#![no_std]

use core::convert::{TryFrom, TryInto};
use core::ops::Deref;

use mm::{PhysAddr, VirtAddr};

pub mod acpi;
pub mod mem;
mod utils;

/// Represents an UEFI error.
#[derive(Debug)]
pub enum Error {
    /// The signature of the table does not match the expected one.
    InvalidSignature,

    /// The CRC32 checksum of the table does not match the expected one.
    InvalidCheckSum,

    /// The revision of the parsed table is not valid.
    InvalidRevision,

    /// Invalid status code conversion.
    InvalidStatusConversion,

    /// The size of the memory address does not match the target architecture.
    InvalidAddressSize,

    /// Could not parse ACPI structures.
    InvalidAcpiData,

    /// The fixed size buffer is too small.
    BufferTooSmall,

    /// The entity could not be found.
    NotFound,

    /// The status code returned by an EFI interface is an error.
    StatusError(StatusError),

    /// The status code returned by an EFI interface is a warning.
    StatusWarning(StatusWarning),

    /// Error related to a memory map operation.
    RangeError(range::Error),
}

impl From<range::Error> for Error {
    fn from(err: range::Error) -> Self {
        Error::RangeError(err)
    }
}

/// The `EFI_STATUS` type of the UEFI specification.
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

impl From<StatusWarning> for Error {
    fn from(warn: StatusWarning) -> Self {
        Error::StatusWarning(warn)
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

impl From<StatusError> for Error {
    fn from(err: StatusError) -> Self {
        Error::StatusError(err)
    }
}

/// Represents the status code returned by an EFI interface.
enum Status {
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

/// Represents an UEFI handle. It is equivalent to the `EFI_HANDLE` type of the
/// UEFI specification.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Handle(usize);

impl Deref for Handle {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a generic pointer.
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct Ptr(pub usize);

impl From<usize> for Ptr {
    fn from(addr: usize) -> Self {
        Ptr(addr)
    }
}

impl TryFrom<u64> for Ptr {
    type Error = Error;

    fn try_from(addr: u64) -> Result<Self, Self::Error> {
        let addr: usize =
            addr.try_into().or(Err(Error::InvalidAddressSize))?;
        Ok(addr.into())
    }
}

/// The `EFI_TABLE_HEADER` type of the UEFI specification.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
    boot_services: Ptr,
    number_of_table_entries: usize,
    configuration_table: Ptr,
}

/// Represents the EFI System Table. It provides access to the boot and runtime
/// services.
#[derive(Debug)]
pub struct SystemTable {
    /// The `EFI_SYSTEM_TABLE` structure provided by the firmware.
    system_table: EfiSystemTable,
}

impl SystemTable {
    /// Creates a new `SystemTable` from a given pointer `system_table_ptr`.
    /// Usually this pointer is the `system_table` argument of the UEFI entry
    /// point.
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
    /// The System Table is created using a pointer. Thus, this function is
    /// considered unsafe.
    pub unsafe fn new(system_table_ptr: Ptr) -> Result<Self, Error> {
        let system_table_ptr = system_table_ptr.0 as *const EfiSystemTable;
        let system_table = core::ptr::read_unaligned(system_table_ptr);

        // Check table's signature.
        if system_table.hdr.signature != EFI_SYSTEM_TABLE_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Check table's CRC32.
        let mut system_table_crc32 = system_table.clone();
        system_table_crc32.hdr.crc32 = 0;
        let crc32 = utils::crc32_for_value(system_table_crc32);
        if crc32 != system_table.hdr.crc32 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(SystemTable { system_table })
    }

    /// Returns the boot services.
    pub fn boot_services(&self) -> Result<BootServices, Error> {
        // A `SystemTable` is only created after checking its signature
        // and CRC32. Thus, we assume that the pointer to the Boot Services
        // Table will be valid.
        unsafe { BootServices::new(self.system_table.boot_services) }
    }

    /// Returns the configuration tables.
    pub fn configuration_tables(&self) -> Result<ConfigurationTables, Error> {
        // A `SystemTable` is only created after checking its signature
        // and CRC32. Thus, we assume that the pointer to the Configuration
        // Tables and the number of table entries are valid.
        unsafe {
            ConfigurationTables::new(
                self.system_table.configuration_table,
                self.system_table.number_of_table_entries,
            )
        }
    }
}

/// Represents a physical memory address. It is equivalent to the
/// `EFI_PHYSICAL_ADDRESS` type of the UEFI specification.
#[repr(transparent)]
struct EfiPhysAddr(u64);

impl From<EfiPhysAddr> for PhysAddr {
    fn from(addr: EfiPhysAddr) -> Self {
        PhysAddr::from(addr.0)
    }
}

/// Represents a virtual memory address. It is equivalent to the
/// `EFI_VIRTUAL_ADDRESS` type of the UEFI specification.
#[repr(transparent)]
struct EfiVirtAddr(u64);

impl From<EfiVirtAddr> for VirtAddr {
    fn from(addr: EfiVirtAddr) -> Self {
        VirtAddr::from(addr.0)
    }
}

/// The `EFI_MEMORY_TYPE` type of the UEFI specification.
#[repr(transparent)]
struct EfiMemoryType(u32);

/// The type of memory.
enum MemoryType {
    /// Not usable memory.
    ReservedMemory,

    /// The code portions of a loaded UEFI application.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, the UEFI OS
    /// Loader and/or OS may use this memory as they see fit. However, the UEFI
    /// OS loader that called `EFI_BOOT_SERVICES.ExitBootServices()` is
    /// utilizing one or more of these ranges.
    LoaderCode,

    /// The data portions of a loaded UEFI application and the default data
    /// allocation type used by a UEFI application to allocate pool memory.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, the UEFI OS
    /// Loader and/or OS may use this memory as they see fit. However, the UEFI
    /// OS loader that called `EFI_BOOT_SERVICES.ExitBootServices()` is
    /// utilizing one or more of these ranges.
    LoaderData,

    /// The code portions of a loaded UEFI Boot Service Driver.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory s
    /// available for general use.
    BootServicesCode,

    /// The data portions of a loaded UEFI Boot Serve Driver, and the default
    /// data allocation type used by a UEFI Boot Service Driver to allocate
    /// pool memory.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is
    /// available for general use.
    BootServicesData,

    /// The code portions of a loaded UEFI Runtime Driver.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, the memory in
    /// this range is to be preserved by the UEFI OS loader and OS in the
    /// working and ACPI S1-S3 states.
    RuntimeServicesCode,

    /// The data portions of a loaded UEFI Runtime Driver and the default data
    /// allocation type used by a UEFI Runtime Driver to allocate pool memory.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, the memory in
    /// this range is to be preserved by the UEFI OS loader and OS in the
    /// working and ACPI S1-S3 states.
    RuntimeServicesData,

    /// Free (unallocated) memory.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is
    /// available for general use.
    ConventionalMemory,

    /// Memory in which errors have been detected.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is
    /// not to be used.
    UnusableMemory,

    /// Memory that holds the ACPI tables.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is to
    /// be preserved by the UEFI OS loader and OS until ACPI is enabled. Once
    /// ACPI is enabled, the memory in this range is available for general use.
    ACPIReclaimMemory,

    /// Address space reserved for use by the firmware.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is to
    /// be preserved by the UEFI OS loader and OS in the working and ACPI S1-S3
    /// states.
    ACPIMemoryNVS,

    /// Used by system firmware to request that a memory-mapped IO region be
    /// mapped by the OS to a virtual address so it can be accessed by EFI
    /// runtime services.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is
    /// not used by the OS. All system memory-mapped IO information should come
    /// from ACPI tables.
    MemoryMappedIO,

    /// System memory-mapped IO region that is used to translate memory cycles
    /// to IO cycles by the processor.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is
    /// not used by the OS. All system memory-mapped IO port space information
    /// should come from ACPI tables.
    MemoryMappedIOPortSpace,

    /// Address space reserved by the firmware for code that is part of the
    /// processor.
    ///
    /// After calling `EFI_BOOT_SERVICES.ExitBootServices()`, this memory is to
    /// be preserved by the UEFI OS loader and OS in the working and ACPI S1-S4
    /// states. This memory may also have other attributes that are defined by
    /// the processor implementation.
    PalCode,

    /// A memory region that operates as `MemoryType::ConventionalMemory`.
    /// However, it happens to also support byte-addressable non-volatility.
    PersistentMemory,

    /// A memory region that represents unaccepted memory, that must be
    /// accepted by the boot target before it can be used. Unless otherwise
    /// noted, all other EFI memory types are accepted. For platforms that
    /// support unaccepted memory, all unaccepted valid memory will be reported
    /// as unaccepted in the memory map. Unreported physical address ranges
    /// must be treated as not-present memory.
    UnacceptedMemory,

    /// Unknown memory type.
    Unknown(u32),
}

impl From<EfiMemoryType> for MemoryType {
    fn from(mem_type: EfiMemoryType) -> Self {
        match mem_type.0 {
            0 => MemoryType::ReservedMemory,
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
            15 => MemoryType::UnacceptedMemory,
            ty => MemoryType::Unknown(ty),
        }
    }
}

/// The `EFI_MEMORY_DESCRIPTOR` type of the UEFI specification.
#[repr(C)]
struct EfiMemoryDescriptor {
    memory_type: EfiMemoryType,
    physical_start: EfiPhysAddr,
    virtual_start: EfiVirtAddr,
    number_of_pages: u64,
    attribute: u64,
}

/// The signature of an EFI Boot Services Table.
const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x56524553544f4f42;

/// The `EFI_BOOT_SERVICES` type of the UEFI specification.
#[derive(Debug, Clone)]
#[repr(C)]
struct EfiBootServices {
    hdr: EfiTableHeader,

    // Task priority services.
    raise_tpl: Ptr,
    restore_tpl: Ptr,

    // Memory services.
    allocate_pages: Ptr,
    free_pages: Ptr,
    get_memory_map: extern "C" fn(
        *mut usize,
        *mut u8,
        *mut usize,
        *mut usize,
        *mut u32,
    ) -> EfiStatus,
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
    exit_boot_services:
        extern "C" fn(image_handle: Handle, map_key: usize) -> EfiStatus,

    // Miscelaneous services.
    get_next_monotonic_count: Ptr,
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
#[derive(Debug)]
pub struct BootServices {
    /// The `EFI_BOOT_SERVICES` structure provided by the firmware.
    boot_services: EfiBootServices,
}

impl BootServices {
    /// Creates a new `BootServices` from a given pointer `boot_services_ptr`.
    ///
    /// # Errors
    ///
    /// If the signature or the CRC32 of the table do not match the expected
    /// values the function will return an error.
    ///
    /// # Safety
    ///
    /// The Boot Services Table is created using a pointer. Thus, this function
    /// is considered unsafe.
    pub unsafe fn new(boot_services_ptr: Ptr) -> Result<Self, Error> {
        let boot_services_ptr = boot_services_ptr.0 as *const EfiBootServices;
        let boot_services = core::ptr::read_unaligned(boot_services_ptr);

        // Check table's signature.
        if boot_services.hdr.signature != EFI_BOOT_SERVICES_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Check table's CRC32.
        let mut boot_services_crc32 = boot_services.clone();
        boot_services_crc32.hdr.crc32 = 0;
        let crc32 = utils::crc32_for_value(boot_services_crc32);
        if crc32 != boot_services.hdr.crc32 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(BootServices { boot_services })
    }

    /// This function must be called by the currently executing UEFI OS loader
    /// image to terminate all boot services. On success, the UEFI OS loader
    /// becomes responsible for the continued operation of the system.
    ///
    /// A UEFI OS loader must ensure that it has the system's current memory
    /// map at the time it calls this function. This is done by passing in the
    /// current memory map's `map_key` value as returned by `get_memory_map`.
    pub fn exit_boot_services(
        &self,
        image_handle: Handle,
        map_key: usize,
    ) -> Result<(), Error> {
        // Call `EFI_BOOT_SERVICES.ExitBootServices()`.
        let status =
            (self.boot_services.exit_boot_services)(image_handle, map_key);

        // Return with error in the case of warning and error status codes.
        match status.into() {
            Status::Success => {}
            Status::Warning(warn) => return Err(Error::StatusWarning(warn)),
            Status::Error(err) => return Err(Error::StatusError(err)),
        }

        Ok(())
    }
}

/// The `EFI_GUID` type of the UEFI specification.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
struct EfiGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

/// The `EFI_CONFIGURATION_TABLE` type of the UEFI specification.
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: Ptr,
}

/// The EFI GUID for a pointer to the ACPI 2.0 or later specification RSDP.
const EFI_ACPI_20_TABLE_GUID: EfiGuid = EfiGuid {
    data1: 0x8868e871,
    data2: 0xe4f1,
    data3: 0x11d3,
    data4: [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
};

/// The maximum number of entries in `ConfigurationTables`.
const EFI_CONFIGURATION_TABLES_LEN: usize = 32;

/// Represents the EFI Configuration Tables. Among other things, it provides
/// access to the ACPI structures.
#[derive(Debug)]
pub struct ConfigurationTables {
    config_tables: [EfiConfigurationTable; EFI_CONFIGURATION_TABLES_LEN],
    num_entries: usize,
}

impl ConfigurationTables {
    /// Creates a new `ConfigurationTables` from a given pointer
    /// `config_tables_ptr` and the number of table entries `num_entries`.
    ///
    /// # Safety
    ///
    /// The `ConfigurationTables` structure is created using a pointer.  Thus,
    /// this function is considered unsafe.
    pub unsafe fn new(
        config_tables_ptr: Ptr,
        num_entries: usize,
    ) -> Result<Self, Error> {
        if num_entries > EFI_CONFIGURATION_TABLES_LEN {
            return Err(Error::BufferTooSmall);
        }

        let ptr = config_tables_ptr.0 as *const EfiConfigurationTable;

        let mut config_tables =
            [EfiConfigurationTable::default(); EFI_CONFIGURATION_TABLES_LEN];
        for (i, it) in config_tables.iter_mut().take(num_entries).enumerate() {
            *it = core::ptr::read_unaligned(ptr.add(i));
        }

        Ok(ConfigurationTables {
            config_tables,
            num_entries,
        })
    }

    /// Returns a pointer to the Root System Description Pointer (RSDP)
    /// structure for the ACPI 2.0 or later specification.
    ///
    /// # Errors
    ///
    /// This function will return `Error::NotFound` if a configuration table
    /// with a valid ACPI table GUID cannot be found.
    pub fn acpi_rsdp20_ptr(&self) -> Result<Ptr, Error> {
        for cfg_table in self.config_tables {
            if cfg_table.vendor_guid == EFI_ACPI_20_TABLE_GUID {
                return Ok(cfg_table.vendor_table);
            }
        }

        Err(Error::NotFound)
    }
}
