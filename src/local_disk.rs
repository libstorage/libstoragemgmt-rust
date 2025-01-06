// Copyright (C) 2017-2025 Red Hat, Inc.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Author: Tony Asleson <tasleson@redhat.com>

#![allow(non_camel_case_types)]

use super::error::*;

use std::convert::TryFrom;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::os::raw::{c_char, c_int};

// Types for opaque C types
enum c_lsm_string_list {}
enum c_lsm_error {}
enum c_lsm_led_handle {}
enum c_lsm_led_slot_itr {}
enum c_lsm_led_slot {}

/// Possible values of link type.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LinkType {
    /// No support
    NoSupport = -2,
    /// Unknown
    Unknown = -1,
    /// Fibre channel
    Fc = 0,
    /// Serial Storage Architecture, old IBM tech.
    Ssa = 2,
    /// Serial Bus Protocol, used by IEEE 1394
    Sbp = 3,
    /// SCSI RDMA Protocol
    Srp = 4,
    /// Internet Small Computer System Interface
    Iscsi = 5,
    /// Serial attached SCSI
    Sas = 6,
    /// Automation/Drive Interface Transport Protocol, often used by Tape.
    Adt = 7,
    /// PATA/IDE or SATA.
    Ata = 8,
    // USB - Universal Serial Bus
    Usb = 9,
    /// SCSI over PCI-e
    Sop = 10,
    /// PCI-e, e.g. NVMe
    Pcie = 11,
}

#[link(name = "storagemgmt")]
extern "C" {

    fn lsm_local_disk_list(
        disk_paths: *mut *mut c_lsm_string_list,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;
    fn lsm_string_list_free(lsm_string: *const c_lsm_string_list) -> c_int;
    fn lsm_string_list_size(lsm_string_list: *const c_lsm_string_list) -> u32;
    fn lsm_string_list_elem_get(
        lsm_string_list: *const c_lsm_string_list,
        index: u32,
    ) -> *const c_char;

    fn lsm_local_disk_vpd83_search(
        vpd83: *const c_char,
        lsm_string_list: *mut *mut c_lsm_string_list,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_serial_num_get(
        disk_path: *const c_char,
        serial_num: *mut *mut c_char,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_vpd83_get(
        disk_path: *const c_char,
        vpd83: *mut *mut c_char,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_health_status_get(
        disk_path: *const c_char,
        health_status: &mut i32,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_rpm_get(
        disk_path: *const c_char,
        rpm: &mut i32,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_link_type_get(
        disk_path: *const c_char,
        link_type: *mut LinkType,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_ident_led_on(
        disk_path: *const c_char,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_ident_led_off(
        disk_path: *const c_char,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_fault_led_on(
        disk_path: *const c_char,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_fault_led_off(
        disk_path: *const c_char,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_led_status_get(
        disk_path: *const c_char,
        led_status: &mut u32,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_local_disk_link_speed_get(
        disk_path: *const c_char,
        link_speed: &mut u32,
        lsm_error: *mut *mut c_lsm_error,
    ) -> c_int;

    fn lsm_error_number_get(lsm_error: *const c_lsm_error) -> c_int;
    fn lsm_error_message_get(lsm_error: *const c_lsm_error) -> *const c_char;
    fn lsm_error_free(lsm_error: *const c_lsm_error) -> c_int;

    fn lsm_led_handle_get(handle: *mut *mut c_lsm_led_handle, flags: u64) -> c_int;
    fn lsm_led_handle_free(handle: *const c_lsm_led_handle);
    fn lsm_led_slot_iterator_free(
        handle: *const c_lsm_led_handle,
        slot_itr: *const c_lsm_led_slot_itr,
    );
    fn lsm_led_slot_iterator_get(
        handle: *const c_lsm_led_handle,
        slot_itr: *mut *mut c_lsm_led_slot_itr,
        lsm_error: *mut *mut c_lsm_error,
        flags: u64,
    ) -> c_int;

    fn lsm_led_slot_iterator_reset(
        handle: *const c_lsm_led_handle,
        slot_itr: *const c_lsm_led_slot_itr,
    );

    fn lsm_led_slot_next(
        handle: *const c_lsm_led_handle,
        itr: *mut c_lsm_led_slot_itr,
    ) -> *const c_lsm_led_slot;

    fn lsm_led_slot_status_get(slot: *const c_lsm_led_slot) -> u32;

    fn lsm_led_slot_status_set(
        handle: *const c_lsm_led_handle,
        slot: *const c_lsm_led_slot,
        status: u32,
        lsm_error: *mut *mut c_lsm_error,
        flag: u64,
    ) -> c_int;

    fn lsm_led_slot_id(slot: *const c_lsm_led_slot) -> *const c_char;
    fn lsm_led_slot_device(slot: *const c_lsm_led_slot) -> *const c_char;
}

extern "C" {
    fn free(ptr: *mut c_void);
}

fn c_char_ptr_to_string(c_str: *const c_char) -> String {
    let m_str;
    unsafe {
        m_str = CStr::from_ptr(c_str);
    }
    let str_slice = m_str.to_str().expect("Invalid UTF-8");
    str_slice.to_owned()
}

fn lsm_c_error_to_rust(lsm_error: *const c_lsm_error) -> LsmError {
    let rc;
    unsafe {
        if !lsm_error.is_null() {
            let code = lsm_error_number_get(lsm_error);
            if code != -1 {
                let message = lsm_error_message_get(lsm_error);

                if !message.is_null() {
                    rc = lsm_error_code_to_lsm_error(code, c_char_ptr_to_string(message))
                } else {
                    rc = lsm_error_code_to_lsm_error(
                        code,
                        String::from("no additional information provided"),
                    )
                }

                // We had a valid pointer, so release the memory and expect success!
                assert!(lsm_error_free(lsm_error) == 0);
            } else {
                panic!("Invalid lsm_error pointer used for error informational retrieval!");
            }
        } else {
            rc = LsmError::LibBug(String::from("The C lsm error ptr was NULL"));
        }
    }
    rc
}

fn c_lsm_string_list_to_vec(c_string_list: *const c_lsm_string_list) -> Vec<String> {
    let mut result_list = Vec::new();

    if !c_string_list.is_null() {
        unsafe {
            let num_items = lsm_string_list_size(c_string_list);

            for i in 0..num_items {
                let disk_path = CStr::from_ptr(lsm_string_list_elem_get(c_string_list, i));
                result_list.push(disk_path.to_string_lossy().into_owned());
            }
            assert!(lsm_string_list_free(c_string_list) == 0);
        }
    }
    result_list
}

/// Searches disk by supplied SCSI VPD 0x83 page NAA type ID.
///
/// Search all the disk paths of specified SCSI VPD 0x83 page NAA type ID.
/// For any ATA and other non-SCSI protocol disks supporting VPD 0x83 pages
/// NAA ID, their disk path will also be included.
///
/// Note: There maybe more than one disk path for any specified disk.
pub fn vpd83_search(vpd83: &str) -> Result<Vec<String>> {
    let c_search_string = CString::new(vpd83).expect("CString::new failed");
    let mut disk_paths = std::ptr::null_mut();
    let mut lsm_error = std::ptr::null_mut();

    unsafe {
        let rc =
            lsm_local_disk_vpd83_search(c_search_string.as_ptr(), &mut disk_paths, &mut lsm_error);
        if rc == 0 {
            Ok(c_lsm_string_list_to_vec(disk_paths))
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

/// Query serial number of specified disk path
///
/// Query the serial number of specified disk path.
/// For SCSI/SAS/SATA/ATA disks, it will be extracted from SCSI VPD 0x80 page.
pub fn serial_num_get(disk_path: &str) -> Result<String> {
    let mut serial_num = std::ptr::null_mut();
    let mut lsm_error = std::ptr::null_mut();

    unsafe {
        let c_sn = CString::new(disk_path).expect("CString::new failed");
        let rc = lsm_local_disk_serial_num_get(c_sn.as_ptr(), &mut serial_num, &mut lsm_error);

        if rc == 0 {
            let sn = CStr::from_ptr(serial_num).to_string_lossy().into_owned();
            free(serial_num as *mut c_void);
            Ok(sn)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

/// Query scsi VPD 0x83 NAA ID.
pub fn vpd83_get(disk_path: &str) -> Result<String> {
    let mut vpd83 = std::ptr::null_mut();
    let mut lsm_error = std::ptr::null_mut();

    unsafe {
        let disk_path = CString::new(disk_path).expect("CString::new failed");
        let rc = lsm_local_disk_vpd83_get(disk_path.as_ptr(), &mut vpd83, &mut lsm_error);

        if rc == 0 {
            let vpd = CStr::from_ptr(vpd83).to_string_lossy().into_owned();
            free(vpd83 as *mut c_void);
            Ok(vpd)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

/// Possible health status of any disk.
pub enum LocalDiskStatus {
    /// Unknown
    Unknown = -1,
    /// Device self reports failure
    Fail = 0,
    /// Device self reports warning
    Warn = 1,
    /// Device self reports good
    Good = 2,
}

impl TryFrom<i32> for LocalDiskStatus {
    type Error = LsmError;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(LocalDiskStatus::Fail),
            1 => Ok(LocalDiskStatus::Warn),
            2 => Ok(LocalDiskStatus::Good),
            _ => Ok(LocalDiskStatus::Unknown),
        }
    }
}

/// Possible values for disk RPM.
#[repr(i32)]
pub enum LocalDiskRpm {
    /// Unknown
    Unknown = -1,
    /// Non-rotational media disk
    NonRotatingMedium = 0,
    /// rotational disk with unknown speed
    UnknownRotationalSpeed = 1,
    /// Rotating disk with specified speed in RPM
    Rpm(i32),
}

impl TryFrom<i32> for LocalDiskRpm {
    type Error = LsmError;

    fn try_from(value: i32) -> Result<Self> {
        let rc = match value {
            -1 => LocalDiskRpm::Unknown,
            0 => LocalDiskRpm::NonRotatingMedium,
            1 => LocalDiskRpm::UnknownRotationalSpeed,
            _ => LocalDiskRpm::Rpm(value),
        };
        Ok(rc)
    }
}

/// Query the health status of the specified disk path.
pub fn health_get(disk_path: &str) -> Result<LocalDiskStatus> {
    let mut status: i32 = 0;
    let mut lsm_error = std::ptr::null_mut();

    unsafe {
        let disk_path = CString::new(disk_path).expect("CString::new failed");
        let rc = lsm_local_disk_health_status_get(disk_path.as_ptr(), &mut status, &mut lsm_error);

        if rc == 0 {
            LocalDiskStatus::try_from(status)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

/// Query disk rotation speed.
pub fn rpm_get(disk_path: &str) -> Result<LocalDiskRpm> {
    let mut rpm: i32 = 0;
    let mut lsm_error = std::ptr::null_mut();

    unsafe {
        let disk_path = CString::new(disk_path).expect("CString::new failed");
        let rc = lsm_local_disk_rpm_get(disk_path.as_ptr(), &mut rpm, &mut lsm_error);

        if rc == 0 {
            LocalDiskRpm::try_from(rpm)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

pub fn list() -> Result<Vec<String>> {
    // Query local disk paths. Currently, only SCSI, SAS, ATA and NVMe disks will be included.
    //
    //! ```rust
    //! extern crate lsm;
    //!
    //! use lsm::local_disk;
    //!
    //!fn main() {
    //!    let disks = local_disk::list().unwrap();
    //!
    //!    for d in disks {
    //!        println!("{}", d);
    //!    }
    //!}
    //! ```
    let mut disk_paths = std::ptr::null_mut();
    let mut lsm_error = std::ptr::null_mut();
    unsafe {
        let rc = lsm_local_disk_list(&mut disk_paths, &mut lsm_error);
        if rc == 0 {
            Ok(c_lsm_string_list_to_vec(disk_paths))
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

/// Query disk link type.
pub fn link_type_get(disk_path: &str) -> Result<LinkType> {
    let mut link_type = LinkType::Unknown;
    let mut lsm_error = std::ptr::null_mut();

    unsafe {
        let disk_path = CString::new(disk_path).expect("CString::new failed");
        let rc = lsm_local_disk_link_type_get(disk_path.as_ptr(), &mut link_type, &mut lsm_error);

        if rc == 0 {
            Ok(link_type)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

fn disk_path_led(disk_path: &str, fault_led: bool, on: bool) -> Result<()> {
    let mut lsm_error = std::ptr::null_mut();
    unsafe {
        let dp = CString::new(disk_path).expect("CString::new failed");

        let rc = match (fault_led, on) {
            (false, true) => lsm_local_disk_ident_led_on(dp.as_ptr(), &mut lsm_error),
            (false, false) => lsm_local_disk_ident_led_off(dp.as_ptr(), &mut lsm_error),
            (true, true) => lsm_local_disk_fault_led_on(dp.as_ptr(), &mut lsm_error),
            (true, false) => lsm_local_disk_fault_led_off(dp.as_ptr(), &mut lsm_error),
        };

        if rc == 0 {
            Ok(())
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

pub fn ident_led_on(disk_path: &str) -> Result<()> {
    // Turn the identification LED on for specified disk path.
    //
    //! ```rust
    //! extern crate lsm;
    //!
    //! use lsm::local_disk;
    //!
    //!fn main() {
    //!     match local_disk::ident_led_on("/dev/sda") {
    //!         Err(e) => println!("Unable to turn on identification LED: reason {:?}", e),
    //!         Ok(_) => println!("Enjoy your blinking LED"),
    //!     }
    //!}
    //! ```
    disk_path_led(disk_path, false, true)
}

/// Turn the identification LED off for specified disk path.
pub fn ident_led_off(disk_path: &str) -> Result<()> {
    disk_path_led(disk_path, false, false)
}

/// Turn the fault LED on for specified disk path.
pub fn fault_led_on(disk_path: &str) -> Result<()> {
    disk_path_led(disk_path, true, true)
}

/// Turn the fault LED off for specified disk path.
pub fn fault_led_off(disk_path: &str) -> Result<()> {
    disk_path_led(disk_path, true, false)
}

/// LED has unknown status
pub const LED_STATUS_UNKNOWN: u32 = 0x0000000000000001;
/// Identification LED is on
pub const LED_STATUS_IDENT_ON: u32 = 0x0000000000000002;
/// Identification LED is off
pub const LED_STATUS_IDENT_OFF: u32 = 0x0000000000000004;
/// Identification LED is unknown
pub const LED_STATUS_IDENT_UNKNOWN: u32 = 0x0000000000000008;
/// Fault LED is on
pub const LED_STATUS_FAULT_ON: u32 = 0x0000000000000010;
/// Fault LED is off
pub const LED_STATUS_FAULT_OFF: u32 = 0x0000000000000020;
/// Fault LED is unknown
pub const LED_STATUS_FAULT_UNKNOWN: u32 = 0x0000000000000040;

/// Retrieve current state of LED for specified disk path.
///
/// Note: Not all enclosures support both identification and fault LEDs.
///
/// Result is a bit sensitive field, see LED_STATUS_* constants
pub fn led_status_get(disk_path: &str) -> Result<u32> {
    let mut lsm_error = std::ptr::null_mut();
    let mut led_status = 0;
    let dp = CString::new(disk_path).expect("CString::new failed");

    unsafe {
        let rc = lsm_local_disk_led_status_get(dp.as_ptr(), &mut led_status, &mut lsm_error);
        if rc == 0 {
            Ok(led_status)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

/// Query the current negotiated disk link speed in Mbps
pub fn link_speed_get(disk_path: &str) -> Result<u32> {
    let mut lsm_error = std::ptr::null_mut();
    let mut link_speed = 0;
    let dp = CString::new(disk_path).expect("CString::new failed");

    unsafe {
        let rc = lsm_local_disk_link_speed_get(dp.as_ptr(), &mut link_speed, &mut lsm_error);
        if rc == 0 {
            Ok(link_speed)
        } else {
            Err(lsm_c_error_to_rust(lsm_error))
        }
    }
}

fn slot_id_get(slot: *const c_lsm_led_slot) -> String {
    let id;
    unsafe {
        id = lsm_led_slot_id(slot);
    }
    assert!(!id.is_null());
    c_char_ptr_to_string(id)
}

fn slot_device_get(slot: *const c_lsm_led_slot) -> Option<String> {
    let device;
    unsafe {
        device = lsm_led_slot_device(slot);
    }

    if device.is_null() {
        None
    } else {
        Some(c_char_ptr_to_string(device))
    }
}

/// Opaque type for interacting with LED slots functionality.
pub struct LedSlots {
    handle: *mut c_lsm_led_handle,
    itr: *mut c_lsm_led_slot_itr,
    curr_slot: *const c_lsm_led_slot,
    curr_id: String,
}

impl Drop for LedSlots {
    fn drop(&mut self) {
        unsafe {
            lsm_led_slot_iterator_free(self.handle, self.itr);
            self.itr = std::ptr::null_mut();
            lsm_led_handle_free(self.handle);
            self.handle = std::ptr::null_mut();
            self.curr_slot = std::ptr::null_mut();
        }
    }
}

impl LedSlots {
    fn find_slot(&mut self, id: &str) -> bool {
        let mut rc = false;
        unsafe {
            // When processing LEDs sequentially, this will help
            if !self.itr.is_null() {
                let slot = lsm_led_slot_next(self.handle, self.itr);
                if !slot.is_null() {
                    let slot_id = slot_id_get(slot);
                    if id == slot_id {
                        self.curr_slot = slot;
                        self.curr_id = slot_id;
                        return true;
                    }
                }
            }

            lsm_led_slot_iterator_reset(self.handle, self.itr);
            loop {
                let slot = lsm_led_slot_next(self.handle, self.itr);
                if slot.is_null() {
                    break;
                } else {
                    let slot_id = slot_id_get(slot);
                    if id == slot_id {
                        rc = true;
                        self.curr_slot = slot;
                        self.curr_id = slot_id;
                        break;
                    }
                }
            }
        }
        rc
    }

    fn slot_set(&mut self, id: &str) {
        if self.curr_id != id {
            assert!(self.find_slot(id));
        }
    }

    /// Create a new instance of LED slots
    pub fn new() -> Result<Self> {
        unsafe {
            let mut handle = std::ptr::null_mut();
            let mut itr = std::ptr::null_mut();
            let mut lsm_error = std::ptr::null_mut();

            let handle_rc = lsm_led_handle_get(&mut handle, 0);

            if handle_rc == 0 {
                let itr_rc = lsm_led_slot_iterator_get(handle, &mut itr, &mut lsm_error, 0);

                if itr_rc == 0 {
                    Ok(Self {
                        handle,
                        itr,
                        curr_slot: std::ptr::null_mut(),
                        curr_id: String::new(),
                    })
                } else {
                    // We failed to get the iterator, but we did get the handle, so free it!
                    lsm_led_handle_free(handle);
                    Err(lsm_c_error_to_rust(lsm_error))
                }
            } else {
                Err(lsm_error_code_to_lsm_error(
                    handle_rc,
                    String::from("LedSlot::new() failed, no additional information!"),
                ))
            }
        }
    }

    /// Retrieve the slots
    pub fn slots_get(&mut self) -> Vec<LedSlot> {
        let mut rc = Vec::new();

        unsafe {
            lsm_led_slot_iterator_reset(self.handle, self.itr);

            loop {
                let slot = lsm_led_slot_next(self.handle, self.itr);
                if !slot.is_null() {
                    let entry = LedSlot::new(slot);
                    rc.push(entry);
                } else {
                    break;
                }
            }
        }

        rc
    }

    /// Retrieve the LED status for the specified slot.
    pub fn slot_status_get(&mut self, slot: &LedSlot) -> u32 {
        self.slot_set(&slot.id());
        unsafe { lsm_led_slot_status_get(self.curr_slot) }
    }

    /// Set the status of the specified slot, bit sensitive input state.
    ///
    /// Please note that not all LED hardware supports both
    /// identification and fault LEDs.
    ///
    /// Using this API, please specify what you
    /// would like regardless of support and the hardware will adhere to your request as best it can.
    /// * LED_STATUS_IDENT_ON => Implies fault off
    /// * LED_STATUS_FAULT_ON => Implies ident and fault on
    /// * LED_STATUS_IDENT_OFF => Implies both ident and fault are off
    /// * LED_STATUS_FAULT_OFF => Implies both ident and fault are off
    /// * LSM_DISK_LED_STATUS_IDENT_OFF|LSM_DISK_LED_STATUS_FAULT_OFF
    /// * LED_STATUS_IDENT_ON|LSM_DISK_LED_STATUS_FAULT_OFF
    /// * LED_STATUS_FAULT_ON|LSM_DISK_LED_STATUS_IDENT_OFF
    /// * LED_STATUS_IDENT_ON|LSM_DISK_LED_STATUS_FAULT_ON
    ///
    pub fn slot_status_set(&mut self, slot: &LedSlot, state: u32) -> Result<()> {
        self.slot_set(&slot.id());

        let mut lsm_error = std::ptr::null_mut();

        unsafe {
            let rc = lsm_led_slot_status_set(self.handle, self.curr_slot, state, &mut lsm_error, 0);
            if rc == 0 {
                Ok(())
            } else {
                Err(lsm_c_error_to_rust(lsm_error))
            }
        }
    }
}

/// Information about specific slot
///
/// Note: you can retrieve/set set of LED with out a block device present
pub struct LedSlot {
    /// Slot ID
    id: String,
    /// Device node for slot which may not be present (used for correlation)
    dev: Option<String>,
}
impl LedSlot {
    fn new(slot: *const c_lsm_led_slot) -> Self {
        Self {
            id: slot_id_get(slot),
            dev: slot_device_get(slot),
        }
    }

    /// Slot ID of slot (this is not persistent across boots or dynamic reconfiguration)
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Device node for block device which may not be present (you don't need a functional disk to change LED state)
    pub fn device(&self) -> Option<String> {
        self.dev.clone()
    }
}
