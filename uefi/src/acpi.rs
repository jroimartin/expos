//! This module provides access to ACPI.

use core::convert::TryInto;

use crate::utils;
use crate::{Error, Ptr};

/// The signature of the RSDP structure.
const ACPI_RSDP_SIGNATURE: &[u8] = b"RSD PTR ";

/// The size of a SDT header.
const ACPI_SDT_SIZE: usize = core::mem::size_of::<AcpiSdtHeader>();

/// The Root System Description Pointer (RSDP) structure for the ACPI 2.0 or
/// later specification.
#[derive(Debug)]
#[repr(C)]
struct AcpiRsdp20 {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
    length: u32,
    xsdt_addr: u64,
    ext_checksum: u8,
    reserved: [u8; 3],
}

/// Represents the Root System Description Pointer (RSDP) for the ACPI 2.0 or
/// later specification.
#[derive(Debug)]
pub struct Rsdp20 {
    /// The `RSDP` structure provided by ACPI.
    rsdp20: AcpiRsdp20,
}

impl Rsdp20 {
    /// Creates a new `Rsdp` from a given pointer.
    ///
    /// # Errors
    ///
    /// This function returns error if the pointer does not point to a valid
    /// RSDP 2.0+ structure.
    ///
    /// # Safety
    ///
    /// The `Rsdp` structure is created using a pointer. Thus, this function is
    /// considered unsafe.
    pub unsafe fn new(rsdp20_ptr: Ptr) -> Result<Self, Error> {
        let rsdp20_ptr = rsdp20_ptr.0 as *const AcpiRsdp20;
        let rsdp20 = core::ptr::read_unaligned(rsdp20_ptr);

        // Check table's signature.
        if rsdp20.signature != ACPI_RSDP_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Check table's revision.
        if rsdp20.revision < 2 {
            return Err(Error::InvalidAcpiData);
        }

        // Check table's checksum.
        let checksum = utils::add_bytes(
            &rsdp20 as *const AcpiRsdp20 as *const u8,
            rsdp20.length as usize,
        );
        if checksum != 0 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(Rsdp20 { rsdp20 })
    }

    /// Returns the address of the Extended System Description Table (XSDT).
    ///
    /// # Errors
    ///
    /// This function returns error if the size of the address is not
    /// compatible with the pointer size of the host.
    pub fn xsdt_ptr(&self) -> Result<Ptr, Error> {
        self.rsdp20.xsdt_addr.try_into()
    }
}

/// System Description Table types.
pub enum SdtType {
    Xsdt,
    Madt,
}

impl SdtType {
    /// Returns the signature of the SDT.
    pub fn signature(&self) -> &[u8] {
        match self {
            SdtType::Xsdt => b"XSDT",
            SdtType::Madt => b"APIC",
        }
    }
}

/// The System Description Table header, which is common to all System
/// Description Tables.
#[derive(Debug)]
#[repr(C)]
struct AcpiSdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl AcpiSdtHeader {
    /// Creates a new `AcpiSdtHeader` from a given pointer.
    ///
    /// # Errors
    ///
    /// The function returns error if the signature of the table does not match
    /// the provided `SdtType` or the checksum is invalid.
    unsafe fn new(sdt_ptr: Ptr, sdt_type: SdtType) -> Result<Self, Error> {
        // Read SDT header.
        let sdt_ptr = sdt_ptr.0 as *const AcpiSdtHeader;
        let hdr = core::ptr::read_unaligned(sdt_ptr);

        // Check SDT header's signature.
        if hdr.signature != sdt_type.signature() {
            return Err(Error::InvalidSignature);
        }

        // Check SDT header's checksum.
        let checksum =
            utils::add_bytes(sdt_ptr as *const u8, hdr.length as usize);
        if checksum != 0 {
            return Err(Error::InvalidCheckSum);
        }

        Ok(hdr)
    }
}

/// The maximum number of entries in the XSDT.
const ACPI_XSDT_ENTRIES_LEN: usize = 32;

/// Represents the Extended System Description Table (XSDT).
#[derive(Debug)]
pub struct Xsdt {
    hdr: AcpiSdtHeader,
    entries: [u64; ACPI_XSDT_ENTRIES_LEN],
    num_entries: usize,
}

impl Xsdt {
    /// Creates a new `Xsdt` from a given pointer.
    ///
    /// # Errors
    ///
    /// This function returns error if the pointer does not point to a valid
    /// XSDT.
    ///
    /// # Safety
    ///
    /// The `Xsdt` structure is created using a pointer. Thus, this function is
    /// considered unsafe.
    pub unsafe fn new(xsdt_ptr: Ptr) -> Result<Self, Error> {
        // Create header.
        let hdr = AcpiSdtHeader::new(xsdt_ptr, SdtType::Xsdt)?;

        // Calculate number of entries.
        let entries_length = hdr.length as usize - ACPI_SDT_SIZE;
        if entries_length % 8 != 0 {
            return Err(Error::InvalidAcpiData);
        }
        let num_entries = entries_length / 8;

        // Check that there is enough room for the entries in the fixed size
        // array.
        if num_entries > ACPI_XSDT_ENTRIES_LEN {
            return Err(Error::BufferTooSmall);
        }

        // Read entries.
        let mut entries = [0u64; ACPI_XSDT_ENTRIES_LEN];
        for (i, it) in entries.iter_mut().take(num_entries).enumerate() {
            let ptr = (xsdt_ptr.0 as *const AcpiSdtHeader as *const u8)
                .add(ACPI_SDT_SIZE + i * 8)
                as *const u64;
            *it = core::ptr::read_unaligned(ptr);
        }

        Ok(Xsdt {
            hdr,
            entries,
            num_entries,
        })
    }

    /// Returns the pointer to the Multiple APIC Description Table (MADT).
    pub fn madt_ptr(&self) -> Result<Ptr, Error> {
        for &entry in self.entries.iter().take(self.num_entries) {
            let ptr = entry as *const [u8; 4];
            let signature = unsafe { core::ptr::read_unaligned(ptr) };
            if signature == SdtType::Madt.signature() {
                return entry.try_into();
            }
        }

        Err(Error::InvalidAcpiData)
    }
}
