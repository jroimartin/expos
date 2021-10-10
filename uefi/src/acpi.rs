//! This module provides access to ACPI.

use core::convert::TryInto;

use crate::utils;
use crate::{Error, Ptr};

/// Signature of the RSDP structure.
const ACPI_RSDP_SIGNATURE: &[u8] = b"RSD PTR ";

/// Size of the SDT header.
const ACPI_SDT_SIZE: usize = core::mem::size_of::<AcpiSdtHeader>();

/// Root System Description Pointer (RSDP) structure of the ACPI 2.0 and later
/// specifications.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
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

/// Represents the Root System Description Pointer (RSDP) of ACPI 2.0+.
#[derive(Debug)]
pub struct Rsdp20 {
    rsdp20: AcpiRsdp20,
}

impl Rsdp20 {
    /// Creates a new `Rsdp20` from a given pointer.
    ///
    /// # Errors
    ///
    /// This function returns error if the pointer does not point to a valid
    /// RSDP 2.0+ structure.
    ///
    /// # Safety
    ///
    /// The `Rsdp20` structure is created using a pointer. Thus, this function
    /// is considered unsafe.
    pub unsafe fn new(rsdp20_ptr: Ptr) -> Result<Self, Error> {
        let rsdp20_ptr = rsdp20_ptr.0 as *const AcpiRsdp20;
        let rsdp20 = core::ptr::read_unaligned(rsdp20_ptr);

        // Check table's signature.
        if rsdp20.signature != ACPI_RSDP_SIGNATURE {
            return Err(Error::InvalidSignature);
        }

        // Check table's revision.
        if rsdp20.revision < 2 {
            return Err(Error::InvalidRevision);
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

    /// Returns the Extended System Description Table (XSDT).
    pub fn xsdt(&self) -> Result<Xsdt, Error> {
        // An `Rsdp20` is only created after checking its signature, checksum
        // and revision. Thus, we assume that the pointer to the XSDT
        // will be valid.
        unsafe { Xsdt::new(self.rsdp20.xsdt_addr.try_into()?) }
    }
}

/// System Description Table types.
enum SdtType {
    Xsdt,
    Madt,
}

impl SdtType {
    /// Returns the signature of the SDT.
    fn signature(&self) -> &[u8] {
        match self {
            SdtType::Xsdt => b"XSDT",
            SdtType::Madt => b"APIC",
        }
    }
}

/// System Description Table header of the ACPI specification. It is common to
/// all System Description Tables.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
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
    /// This function returns error if the signature of the table does not
    /// match the provided `SdtType` or the checksum is invalid.
    unsafe fn new(sdt_ptr: Ptr, sdt_type: SdtType) -> Result<Self, Error> {
        // Parse SDT header.
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

/// Maximum number of entries in the XSDT.
const ACPI_XSDT_ENTRIES_LEN: usize = 32;

/// Represents the Extended System Description Table (XSDT).
#[derive(Debug)]
pub struct Xsdt {
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
        // Parse header.
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

        // Parse entries.
        let mut entries = [0u64; ACPI_XSDT_ENTRIES_LEN];
        for (i, it) in entries.iter_mut().take(num_entries).enumerate() {
            let ptr = (xsdt_ptr.0 as *const u8).add(ACPI_SDT_SIZE + i * 8)
                as *const u64;
            *it = core::ptr::read_unaligned(ptr);
        }

        Ok(Xsdt {
            entries,
            num_entries,
        })
    }

    /// Returns the Multiple APIC Description Table (MADT).
    pub fn madt(&self) -> Result<Madt, Error> {
        // An `Xsdt` is only created after checking its signature and checksum
        // Thus, we assume that the pointer to the MADT will be valid.

        for &entry in self.entries.iter().take(self.num_entries) {
            // Look for a table with the correct signature.
            let ptr = entry as *const [u8; 4];
            let signature = unsafe { core::ptr::read_unaligned(ptr) };
            if signature == SdtType::Madt.signature() {
                return unsafe { Madt::new(entry.try_into()?) };
            }
        }

        // If we reach this point, the table could not be found.
        Err(Error::NotFound)
    }
}

/// Size of the SDT header.
const ACPI_MADT_FIELDS_SIZE: usize = core::mem::size_of::<AcpiMadtFields>();

/// Maximum number of entries in the MADT.
const ACPI_MADT_ENTRIES_LEN: usize = 256;

/// Extra fields of the Multiple APIC Description Table (MADT) in the ACPI
/// specification.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct AcpiMadtFields {
    lapic_addr: u32,
    flags: u32,
}

/// Processor Local APIC Structure in the ACPI specification.
#[repr(C, packed)]
struct AcpiMadtLapic {
    ty: u8,
    length: u8,
    proc_uid: u8,
    apic_id: u8,
    flags: u32,
}

/// Represents a Processor Local APIC Structure.
#[derive(Debug, Default, Clone, Copy)]
pub struct MadtLapic {
    proc_uid: u8,
    apic_id: u8,
    flags: u32,
}

impl MadtLapic {
    /// Processor's UID.
    pub fn proc_uid(&self) -> u8 {
        self.proc_uid
    }

    /// Processor's local APIC ID.
    pub fn acpi_id(&self) -> u8 {
        self.apic_id
    }

    /// Local APIC flags.
    ///
    /// Bit offset | Bit length | Flag
    /// ---------- | ---------- | ---------------
    /// 0          | 1          | Enabled
    /// 1          | 1          | Online Capable
    /// 2          | 30         | Reserved (zero)
    pub fn flags(&self) -> u32 {
        self.flags
    }
}

/// Represents the Multiple APIC Description Table (MADT).
#[derive(Debug)]
pub struct Madt {
    fields: AcpiMadtFields,

    lapic_entries: [MadtLapic; ACPI_MADT_ENTRIES_LEN],
    num_lapic_entries: usize,
}

impl Madt {
    /// Creates a new `Madt` from a given pointer.
    ///
    /// # Errors
    ///
    /// This function returns error if the pointer does not point to a valid
    /// MADT.
    ///
    /// # Safety
    ///
    /// The `Madt` structure is created using a pointer. Thus, this function is
    /// considered unsafe.
    pub unsafe fn new(madt_ptr: Ptr) -> Result<Madt, Error> {
        // Parse header.
        let hdr = AcpiSdtHeader::new(madt_ptr, SdtType::Madt)?;

        // Parse fields.
        let fields = core::ptr::read_unaligned(
            (madt_ptr.0 as *const u8).add(ACPI_SDT_SIZE)
                as *const AcpiMadtFields,
        );

        // Parse entries.
        let mut num_lapic_entries = 0;
        let mut lapic_entries = [MadtLapic::default(); ACPI_MADT_ENTRIES_LEN];

        let mut ptr = (madt_ptr.0 as *const u8)
            .add(ACPI_SDT_SIZE + ACPI_MADT_FIELDS_SIZE);
        let end = (madt_ptr.0 as *const u8).add(hdr.length as usize);

        while ptr < end {
            let ty = core::ptr::read_unaligned(ptr);
            let length = core::ptr::read_unaligned(ptr.add(1));

            // LAPIC.
            if ty == 0 {
                if num_lapic_entries >= ACPI_MADT_ENTRIES_LEN {
                    return Err(Error::BufferTooSmall);
                }

                let lapic =
                    core::ptr::read_unaligned(ptr as *const AcpiMadtLapic);
                lapic_entries[num_lapic_entries] = MadtLapic {
                    proc_uid: lapic.proc_uid,
                    apic_id: lapic.apic_id,
                    flags: lapic.flags,
                };
                num_lapic_entries += 1;
            }

            ptr = ptr.add(length as usize);
        }

        Ok(Madt {
            fields,
            lapic_entries,
            num_lapic_entries,
        })
    }

    /// Local Interrupt Controller Address. In other words, the 32-bit physical
    /// address at which each processor can access its local interrupt
    /// controller.
    pub fn lapic_addr(&self) -> u32 {
        self.fields.lapic_addr
    }

    /// Multiple ACPI flags.
    ///
    /// Bit offset | Bit length | Flag
    /// ---------- | ---------- | ---------------
    /// 0          | 1          | PCAT_COMPAT
    /// 1          | 31         | Reserved (zero)
    pub fn flags(&self) -> u32 {
        self.fields.flags
    }

    /// Returns the detected local APIC structures.
    pub fn lapic(&self) -> &[MadtLapic] {
        &self.lapic_entries[..self.num_lapic_entries]
    }
}
