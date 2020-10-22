extern crate max3010x;
extern crate rppal;

use max3010x::{
    FifoAlmostFullLevelInterrupt, Led, LedPulseWidth, Max3010x, SampleAveraging, SamplingRate,
    TimeSlot,
};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().unwrap();
    let address = 0x57u16;
    i2c.set_slave_address(address).unwrap();
    let max30102 = Max3010x::new_max30102(i2c);
    let mut multi_led = max30102.into_multi_led().unwrap();
    // enable interupts
    multi_led.enable_fifo_almost_full_interrupt().unwrap();
    // multi_led.enable_new_fifo_data_ready_interrupt().unwrap();
    // sample average 4, fifo rollover false, fifo almost full 17
    multi_led
        .set_sample_averaging(SampleAveraging::Sa4)
        .unwrap();
    multi_led.disable_fifo_rollover().unwrap();
    multi_led
        .set_fifo_almost_full_level_interrupt(FifoAlmostFullLevelInterrupt::L0)
        .unwrap();
    multi_led.set_sampling_rate(SamplingRate::Sps100).unwrap();
    multi_led.set_pulse_width(LedPulseWidth::Pw411).unwrap();
    multi_led.set_pulse_amplitude(Led::Led1, 15).unwrap();
    multi_led.set_pulse_amplitude(Led::Led2, 15).unwrap();
    multi_led
        .set_led_time_slots([
            TimeSlot::Led1,
            TimeSlot::Led2,
            TimeSlot::Disabled,
            TimeSlot::Disabled,
        ])
        .unwrap();
    let mut data = [0; 500];
    let interupt_status = multi_led.read_interrupt_status().unwrap();
    println!("Multi interupt status: {:?}", interupt_status);
    let samples_read = multi_led.read_fifo(&mut data).unwrap();
    println!("Sample read: {:?}", samples_read);
    println!("Multi data: {:?}", &data[0..20]);
    multi_led.shutdown().unwrap();
    multi_led.destroy();
}
