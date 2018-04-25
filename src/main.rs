extern crate libusb;
extern crate rand;

use std::process;
use std::time::Duration;


const UDMX_VENDORID: u16 = 0x16c0;
const UDMX_PRODUCTID: u16 = 0x5dc;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut values: Vec<u8> = Vec::new();

    if args.len() < 3 {
        println!("Usage: {binary} [channel] [values...]", binary = args[0]);
        println!("Example: {binary} 1 255 0 0 255", binary = args[0]);
        process::exit(1);
    }

    let mut channel: u16 = args[1].parse().expect("Values for channel must be between 1 and 512");
    if channel == 0 || channel > 512 {
        println!("Values for channel must be between 1 and 512");
        process::exit(1);
    }
    channel = channel - 1;

    for i in 2..args.len() {
        let value: u8 = args[i].parse().expect("Values must be between 0 and 255");
        values.push(value);
    }

    let context = libusb::Context::new().unwrap();

    let device = context.open_device_with_vid_pid(UDMX_VENDORID, UDMX_PRODUCTID).unwrap();

    let bm_request_type = libusb::request_type(libusb::Direction::Out,
                                               libusb::RequestType::Vendor,
                                               libusb::Recipient::Device);


    device.write_control(bm_request_type,
                         2,
                         values.len() as u16,
                         channel,
                         values.as_slice(),
                         Duration::from_millis(10))
        .unwrap();
}
