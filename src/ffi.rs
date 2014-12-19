// FFI bindings to hackrf.h
// Copyright Adam Greig <adam@adamgreig.com> 2014
// Licensed under MIT license

use libc::{c_void, c_uint, c_int, c_char, c_double,
           uint8_t, uint32_t, uint64_t};

pub const HACKRF_SUCCESS: c_int = 0;
pub const HACKRF_TRUE: c_int = 1;
pub const HACKRF_ERROR_INVALID_PARAM: c_int = -2;
pub const HACKRF_ERROR_NOT_FOUND: c_int = -5;
pub const HACKRF_ERROR_BUSY: c_int = -6;
pub const HACKRF_ERROR_NO_MEM: c_int = -11;
pub const HACKRF_ERROR_LIBUSB: c_int = -1000;
pub const HACKRF_ERROR_THREAD: c_int = -1001;
pub const HACKRF_ERROR_STREAMING_THREAD_ERR: c_int = -1002;
pub const HACKRF_ERROR_STREAMING_STOPPED: c_int = -1003;
pub const HACKRF_ERROR_STREAMING_EXIT_CALLED: c_int = -1004;
pub const HACKRF_ERROR_OTHER: c_int = -9999;

pub const BOARD_ID_JELLYBEAN: uint8_t = 0;
pub const BOARD_ID_JAWBREAKER: uint8_t = 1;
pub const BOARD_ID_HACKRF_ONE: uint8_t = 2;
pub const BOARD_ID_INVALID: uint8_t = 0xFF;

pub const RF_PATH_FILTER_BYPASS: c_uint = 0;
pub const RF_PATH_FILTER_LOW_PASS: c_uint = 1;
pub const RF_PATH_FILTER_HIGH_PASS: c_uint = 2;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct hackrf_device;

#[repr(C)]
pub struct hackrf_transfer {
    pub device: *mut hackrf_device,
    pub buffer: *mut uint8_t,
    pub buffer_length: c_int,
    pub valid_length: c_int,
    pub rx_ctx: *mut c_void,
    pub tx_ctx: *mut c_void
}

#[repr(C)]
pub struct read_partid_serialno_t {
    pub part_id: [uint32_t, ..2],
    pub serial_no: [uint32_t, ..4]
}

#[link(name="hackrf")]
extern "C" {
    pub fn hackrf_init() -> c_int;
    pub fn hackrf_exit() -> c_int;

    pub fn hackrf_open(device: *mut *mut hackrf_device) -> c_int;
    pub fn hackrf_close(device: *mut hackrf_device) -> c_int;

    pub fn hackrf_start_rx(device: *mut hackrf_device,
                           callback: extern fn(*mut hackrf_transfer) -> c_int,
                           rx_ctx: *mut c_void)
                           -> c_int;
    pub fn hackrf_stop_rx(device: *mut hackrf_device) -> c_int;
    pub fn hackrf_start_tx(device: *mut hackrf_device,
                           callback: extern fn(*mut hackrf_transfer) -> c_int,
                           tx_ctx: *mut c_void)
                           -> c_int;
    pub fn hackrf_stop_tx(device: *mut hackrf_device) -> c_int;

    // Returns HACKRF_TRUE=1 if success
    pub fn hackrf_is_streaming(device: *mut hackrf_device) -> c_int;

    pub fn hackrf_set_baseband_filter_bandwidth(device: *mut hackrf_device,
                                            bandwidth_hz: uint32_t) -> c_int;

    pub fn hackrf_board_id_read(device: *mut hackrf_device,
                            value: *mut uint8_t) -> c_int;
    pub fn hackrf_version_string_read(device: *mut hackrf_device,
                                      version: *mut c_char,
                                      length: uint8_t) -> c_int;
    pub fn hackrf_board_partid_serialno_read(
        device: *mut hackrf_device,
        read_partid_serialno: *mut read_partid_serialno_t) -> c_int;

    pub fn hackrf_set_freq(device: *mut hackrf_device,
                       freq_hz: uint64_t) -> c_int;
    pub fn hackrf_set_freq_explicit(device: *mut hackrf_device,
                                if_freq_hz: uint64_t,
                                lo_freq_hz: uint64_t,
                                path: c_uint) -> c_int;
    
    // Currently 8-20MHz, either as a fraction:
    // freq=20_000_000 divider=2 giving 10MHz
    // or as a double: freq=10_000_000
    // Preferred rates are 8, 10, 12.5, 16 and 20MHz due to less jitter.
    pub fn hackrf_set_sample_rate_manual(device: *mut hackrf_device,
                                     freq_hz: uint32_t,
                                     divider: uint32_t) -> c_int;
    pub fn hackrf_set_sample_rate(device: *mut hackrf_device,
                              freq_hz: c_double) -> c_int;

    // External amp, bool on/off
    pub fn hackrf_set_amp_enable(device: *mut hackrf_device,
                             value: uint8_t) -> c_int;

    // Range 0-40 step 8dB
    pub fn hackrf_set_lna_gain(device: *mut hackrf_device,
                           value: uint32_t) -> c_int;
    // Range 0-62 step 2dB
    pub fn hackrf_set_vga_gain(device: *mut hackrf_device,
                           value: uint32_t) -> c_int;
    // Range 0-47 step 1dB
    pub fn hackrf_set_txvga_gain(device: *mut hackrf_device,
                             value: uint32_t) -> c_int;

    // Antenna port power control
    pub fn hackrf_set_antenna_enable(device: *mut hackrf_device,
                                 value: uint8_t) -> c_int;

    pub fn hackrf_error_name(errcode: c_int) -> *const c_char;
    pub fn hackrf_board_id_name(hackrf_board_id: uint8_t) -> *const c_char;
    pub fn hackrf_filter_path_name(path: c_uint) -> *const c_char;

    // Compute nearest freq for bw filter (manual filter)
    pub fn hackrf_compute_baseband_filter_bw_round_down_lt(
        bandwidth_hz: uint32_t) -> uint32_t;
    // Compute best default value depending on sample rate (auto filter)
    pub fn hackrf_compute_baseband_filter_bw(
        bandwidth_hz: uint32_t) -> uint32_t;

}
