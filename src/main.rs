// Demonstration of the hackrf crate
// Copyright Adam Greig <adam@adamgreig.com> 2014
// Licensed under MIT license

extern crate hackrf;

fn main() {
    hackrf::init().unwrap();
    
    println!("Opening device.");
    let mut device = hackrf::open().unwrap();

    let board_id = hackrf::board_id_read(&mut device).unwrap();
    println!("Board ID: {}", board_id);
    
    let board_version = hackrf::version_string_read(&mut device).unwrap();
    println!("Board version: {}", board_version);

    let serial = hackrf::board_partid_serialno_read(&mut device);
    let (partid, serialno) = serial.unwrap();
    println!("Board part ID {}, serial number {}", partid, serialno);

    println!("Tuning to IF 2.2GHz, LO 100MHz, filter bypass");
    hackrf::set_freq_explicit(&mut device, 2_200_000_000, 100_000_000,
                              hackrf::RFPathFilter::Bypass).unwrap();

    println!("Tuning to 434MHz");
    hackrf::set_freq(&mut device, 434_000_000).unwrap();

    println!("Setting sample rate to 8Msps");
    hackrf::set_sample_rate(&mut device, 8e6).unwrap();

    println!("Setting sample rate to 16Msps/2");
    hackrf::set_sample_rate_manual(&mut device, 16_000_000, 2).unwrap();

    println!("Disabling power amplifier");
    hackrf::set_amp_enable(&mut device, false).unwrap();

    println!("Setting LNA gain to 2 (=16dB)")
    hackrf::set_lna_gain(&mut device, 0).unwrap();

    println!("Setting VGA gain to 4 (=8dB)")
    hackrf::set_vga_gain(&mut device, 32).unwrap();

    println!("Setting TXVGA gain to 6 (=6dB)")
    hackrf::set_txvga_gain(&mut device, 12).unwrap();

    println!("Disabling antenna power")
    hackrf::set_antenna_enable(&mut device, false).unwrap();

    let bw1 = hackrf::compute_baseband_filter_bw_round_down_lt(2000);
    println!("bw1={}", bw1);

    let bw2 = hackrf::compute_baseband_filter_bw(2000);
    println!("bw2={}", bw2);

    println!("Setting up RX stream");
    let mut rx_cb = |buffer: &[u8]| -> bool {
        println!("rx");
        true
    };
    hackrf::start_rx(&mut device, &mut rx_cb).unwrap();
    std::io::timer::sleep(std::time::duration::Duration::milliseconds(200));
    println!("Stopping RX stream");
    hackrf::stop_rx(&mut device).unwrap();

    println!("Re-opening");
    hackrf::close(device).unwrap();
    let mut device = hackrf::open().unwrap();


    println!("Setting up TX stream");
    let mut tx_cb = |buffer: &mut[u8]| -> bool {
        println!("tx");
        true
    };
    hackrf::start_tx(&mut device, &mut tx_cb).unwrap();
    std::io::timer::sleep(std::time::duration::Duration::milliseconds(200));
    println!("Stopping TX stream");
    hackrf::stop_tx(&mut device).unwrap();

    println!("Closing device.");
    hackrf::close(device).unwrap();

    hackrf::exit().unwrap();
}
