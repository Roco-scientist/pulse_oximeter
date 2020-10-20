extern crate max3010x;
extern crate rppal;

use max3010x::{Led, Max3010x, SampleAveraging, TimeSlot};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().unwrap();
    let address = 0x57u16;
    i2c.set_slave_address(address).unwrap();
    let max30102_start = Max3010x::new_max30102(i2c);
    let mut max30102 = max30102_start.into_multi_led().unwrap();
    max30102.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    max30102.set_pulse_amplitude(Led::All, 15).unwrap();
    max30102
        .set_led_time_slots([
            TimeSlot::Led1,
            TimeSlot::Led2,
            TimeSlot::Led1,
            TimeSlot::Disabled,
        ])
        .unwrap();
    max30102.enable_fifo_rollover().unwrap();
    let mut data = [4; 100];
    let samples_read = max30102.read_fifo(&mut data).unwrap();

    println!("Sample read: {:?}", samples_read);
    // get the I2C device back
    let i2c = max30102.destroy();
}
