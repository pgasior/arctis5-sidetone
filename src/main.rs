fn main() {
    let api = hidapi::HidApi::new().unwrap();
    let device = api.open(4152, 4778).expect("Unable to open device");

    let buff = vec![0x04, 0x40, 0x02, 0x12, 0xc6, 0x00, 0x00];
    device.write(&buff).unwrap();

    let buff = vec![0x06, 0x81, 0x43, 0x01, 0x23];
    device.write(&buff).unwrap();
}
