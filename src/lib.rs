// Rust bindings for libhackrf
// Adam Greig <adam@adamgreig.com> Dec 2014

#![allow(dead_code)]

extern crate libc;

mod ffi;

pub struct HackRFDevice {
    ptr: *mut ffi::hackrf_device
}

impl Drop for HackRFDevice {
    #[inline(never)]
    fn drop(&mut self) {
        unsafe {
            ffi::hackrf_close(self.ptr);
        }
    }
}

pub struct HackRFError {
    errno: int,
    errstr: String
}

impl std::fmt::Show for HackRFError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HackRF error: {} ({})", self.errstr, self.errno)
    }
}

fn hackrf_error(err: libc::c_int) -> HackRFError {
    let s = unsafe {
        let ptr = ffi::hackrf_error_name(err);
        std::c_str::CString::new(ptr, false)
    };
    HackRFError {errno: err as int, errstr: s.as_str().unwrap().to_string()}
}

/// Initialise the HackRF library. Call this once at application startup.
pub fn init() -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_init() } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// De-initialise the HackRF library. Call this once at application
/// termination.
pub fn exit() -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_exit() } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Attempt to open a connected HackRF device.
pub fn open() -> Result<HackRFDevice, HackRFError> {
    let mut device: HackRFDevice = unsafe { std::mem::zeroed() };
    match unsafe { ffi::hackrf_open(&mut device.ptr) } {
        ffi::HACKRF_SUCCESS => Ok(device),
        err => Err(hackrf_error(err))
    }
}

/// Close a connected HackRF device.
pub fn close(device: HackRFDevice) -> Result<(), HackRFError> {
        match unsafe { ffi::hackrf_close(device.ptr) } {
            ffi::HACKRF_SUCCESS => Ok(()),
            err => Err(hackrf_error(err))
        }
}

/// The library defines the C callback, which will itself call a closure
/// inside Rust after resolving memory stuff, so that users don't need to
/// write unsafe code.
extern "C" fn rx_cb(transfer: *mut ffi::hackrf_transfer) -> libc::c_int {
    println!("rx_cb");
    let data = unsafe { &*transfer };
    let valid_length = data.valid_length as uint;
    let buffer: &[u8] = unsafe {
        std::slice::from_raw_mut_buf(&data.buffer, valid_length)
    };
    let cb_ptr = data.rx_ctx as *mut |&[u8]| -> bool;
    let cb: &mut |&[u8]| -> bool = unsafe { &mut *cb_ptr };
    match (*cb)(buffer) {
        true => 0 as libc::c_int,
        false => 1 as libc::c_int
    }
}

/// The library defines the C callback, which will itself call a closure
/// inside Rust after resolving memory stuff, so that users don't need to
/// write unsafe code.
extern "C" fn tx_cb(transfer: *mut ffi::hackrf_transfer) -> libc::c_int {
    println!("rx_cb");
    let data = unsafe { &*transfer };
    let buffer_length = data.buffer_length as uint;
    let buffer: &mut[u8] = unsafe {
        std::slice::from_raw_mut_buf(&data.buffer, buffer_length)
    };
    let cb_ptr = data.tx_ctx as *mut |&mut[u8]| -> bool;
    let cb: &mut |&mut[u8]| -> bool = unsafe { &mut *cb_ptr };
    match (*cb)(buffer) {
        true => 0 as libc::c_int,
        false => 1 as libc::c_int
    }
}


/// Begin RX stream
pub fn start_rx(device: &mut HackRFDevice, cb: &mut |&[u8]| -> bool)
                -> Result<(), HackRFError> {
    let ctx = (cb as *mut |&[u8]| -> bool) as *mut libc::c_void;
    match unsafe { ffi::hackrf_start_rx(device.ptr, rx_cb, ctx) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Stop RX stream
pub fn stop_rx(device: &mut HackRFDevice) -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_stop_rx(device.ptr) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Begin TX stream
pub fn start_tx(device: &mut HackRFDevice, cb: &mut |&mut[u8]| -> bool)
                -> Result<(), HackRFError> {
    let ctx = (cb as *mut |&mut[u8]| -> bool) as *mut libc::c_void;
    match unsafe { ffi::hackrf_start_tx(device.ptr, tx_cb, ctx) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Stop TX stream
pub fn stop_tx(device: &mut HackRFDevice) -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_stop_tx(device.ptr) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Check if a HackRF device is currently streaming data.
/// Returns true if so, false if stopped due to streaming finishing
/// or exit being called, and an error if not streaming due to error.
pub fn is_streaming(device: &mut HackRFDevice) -> Result<bool, HackRFError> {
    match unsafe { ffi::hackrf_is_streaming(device.ptr) } {
        ffi::HACKRF_TRUE => Ok(true),
        ffi::HACKRF_ERROR_STREAMING_STOPPED |
        ffi::HACKRF_ERROR_STREAMING_EXIT_CALLED => Ok(false),
        err => Err(hackrf_error(err))
    }
}

/// Set the HackRF baseband filter bandwidth, in Hz.
/// See also `compute_baseband_filter_bw` and
/// `compute_baseband_filter_bw_round_down_lt`.
pub fn set_baseband_filter_bandwidth(device: &mut HackRFDevice,
                                     bandwidth_hz: uint)
                                     -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_set_baseband_filter_bandwidth(
                            device.ptr, bandwidth_hz as libc::uint32_t) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Read the board ID. Returns a tuple of the numeric ID and a corresponding
/// String. This is the product identifier, not a serial number.
pub fn board_id_read(device: &mut HackRFDevice) 
                     -> Result<(int, String), HackRFError> {
    let mut id: libc::uint8_t = ffi::BOARD_ID_INVALID;
    match unsafe { ffi::hackrf_board_id_read(device.ptr, &mut id) } {
        ffi::HACKRF_SUCCESS => {
            let s = unsafe {
                let ptr = ffi::hackrf_board_id_name(id as libc::uint8_t);
                std::c_str::CString::new(ptr, false)
            };
            Ok((id as int, s.as_str().unwrap().to_string()))
        },
        err => Err(hackrf_error(err))
    }
}


/// Read the board's firmware version string.
pub fn version_string_read(device: &mut HackRFDevice)
                           -> Result<String, HackRFError> {
    let mut buf = [0i8, ..127];
    match unsafe { ffi::hackrf_version_string_read(device.ptr,
                                                         buf.as_mut_ptr(),
                                                         127) } {
        ffi::HACKRF_SUCCESS => {
            let s = unsafe {
                std::str::from_utf8(std::mem::transmute(buf.as_slice()))};
            Ok(String::from_str(s.unwrap()))
        },
        err => Err(hackrf_error(err))
    }
}

/// Read the part ID and serial number
pub fn board_partid_serialno_read(device: &mut HackRFDevice)
                                  -> Result<([u32, ..2], [u32, ..4]),
                                              HackRFError> {
    let mut serial: ffi::read_partid_serialno_t = unsafe {
        std::mem::zeroed() };
    match unsafe { ffi::hackrf_board_partid_serialno_read(device.ptr,
                                                          &mut serial) } {
        ffi::HACKRF_SUCCESS => Ok((serial.part_id, serial.serial_no)),
        err => Err(hackrf_error(err))
    }
}

/// Set HackRF frequency
pub fn set_freq(device: &mut HackRFDevice, freq_hz: u64)
                -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_set_freq(device.ptr, freq_hz) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

#[deriving(Copy, Clone)]
pub enum RFPathFilter {Bypass, LowPass, HighPass}

/// Set HackRF frequency, specifying IF and LO and filters separately.
/// `path` may be `RFPathFilter::Bypass`, `LowPass` or `HighPass`.
pub fn set_freq_explicit(device: &mut HackRFDevice, if_freq_hz: u64,
                         lo_freq_hz: u64, path: RFPathFilter)
                         -> Result<(), HackRFError> {
    let c_path = match path {
        RFPathFilter::Bypass => ffi::RF_PATH_FILTER_BYPASS,
        RFPathFilter::LowPass => ffi::RF_PATH_FILTER_LOW_PASS,
        RFPathFilter::HighPass => ffi::RF_PATH_FILTER_HIGH_PASS
    };
    match unsafe { ffi::hackrf_set_freq_explicit(device.ptr, if_freq_hz,
                                                 lo_freq_hz, c_path) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set HackRF sample rate, specifying integer frequency and divider
/// Preferred rates are 8, 10, 12.5, 16 and 20MHz
pub fn set_sample_rate_manual(device: &mut HackRFDevice, freq_hz: u32,
                              divider: u32) -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_set_sample_rate_manual(device.ptr, freq_hz,
                                                      divider) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set HackRF sample rate, specifying frequency as a double float
/// Preferred rates are 8, 10, 12.5, 16 and 20MHz
pub fn set_sample_rate(device: &mut HackRFDevice, freq_hz: f64)
                       -> Result<(), HackRFError> {
    match unsafe { ffi::hackrf_set_sample_rate(device.ptr, freq_hz) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set HackRF external amplifier on or off
pub fn set_amp_enable(device: &mut HackRFDevice, on: bool)
                      -> Result<(), HackRFError> {
    let value = match on { false => 0u8, true => 1 };
    match unsafe { ffi::hackrf_set_amp_enable(device.ptr, value) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set LNA gain, 0-40 in steps of 8dB
pub fn set_lna_gain(device: &mut HackRFDevice, gain: u32)
                    -> Result<(), HackRFError> {
    assert!(gain <= 40);
    match unsafe { ffi::hackrf_set_lna_gain(device.ptr, gain) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set VGA gain, 0-62 in steps of 2dB
pub fn set_vga_gain(device: &mut HackRFDevice, gain: u32)
                    -> Result<(), HackRFError> {
    assert!(gain <= 62);
    match unsafe { ffi::hackrf_set_vga_gain(device.ptr, gain) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set TXVGA gain, 0-47 in steps of 1dB
pub fn set_txvga_gain(device: &mut HackRFDevice, gain: u32)
                    -> Result<(), HackRFError> {
    assert!(gain <= 47);
    match unsafe { ffi::hackrf_set_txvga_gain(device.ptr, gain) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Set antenna port power on/off
pub fn set_antenna_enable(device: &mut HackRFDevice, on: bool)
                      -> Result<(), HackRFError> {
    let value = match on { false => 0u8, true => 1 };
    match unsafe { ffi::hackrf_set_antenna_enable(device.ptr, value) } {
        ffi::HACKRF_SUCCESS => Ok(()),
        err => Err(hackrf_error(err))
    }
}

/// Compute nearest frequency for bandwidth filter (manual filter)
pub fn compute_baseband_filter_bw_round_down_lt(bandwidth_hz: u32) -> u32 {
    unsafe {
        ffi::hackrf_compute_baseband_filter_bw_round_down_lt(bandwidth_hz)
    }
}

/// Compute best default value for bandwidth filter depending on sample rate
pub fn compute_baseband_filter_bw(bandwidth_hz: u32) -> u32 {
    unsafe {
        ffi::hackrf_compute_baseband_filter_bw(bandwidth_hz)
    }
}
