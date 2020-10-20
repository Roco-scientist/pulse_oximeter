extern crate max3010x;
extern crate rppal;

use max3010x::{Led, Max3010x, SampleAveraging};

fn main() {
    let i2c = rppal::i2c::I2c::new().unwrap();
    let address = 0x57u8;
    i2c.set_slave_address(address);

    let mut sensor = Max3010x::new_max30102(i2c);
    let mut sensor = sensor.into_heart_rate().unwrap();
    sensor.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    sensor.set_pulse_amplitude(Led::All, 15).unwrap();
    sensor.enable_fifo_rollover().unwrap();

    let mut data = [0; 3];
    let samples_read = sensor.read_fifo(&mut data).unwrap();
    prinln!("Sample read: {:?}", samples_read);

    // get the I2C device back
    let dev = max30102.destroy();
}
