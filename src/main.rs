extern crate max3010x;
extern crate rppal;

use max3010x::{Led, LedPulseWidth, Max3010x, SampleAveraging, SamplingRate, TimeSlot};
use rppal::i2c::I2c;
use std::{thread::sleep, time::Duration};

fn main() {
    let i2c = I2c::new().unwrap();
    // let address = 0x57u16;
    // i2c.set_slave_address(address).unwrap();
    let mut max30102 = Max3010x::new_max30102(i2c);
    let mut multi_led = max30102.into_multi_led().unwrap();
    let sample_id = multi_led.get_part_id().unwrap();
    println!("Sample ID: {:?}", sample_id);
    multi_led
        .set_sample_averaging(SampleAveraging::Sa4)
        .unwrap();
    multi_led.set_pulse_amplitude(Led::All, 10).unwrap();
    multi_led.set_sampling_rate(SamplingRate::Sps200).unwrap();
    multi_led.set_pulse_width(LedPulseWidth::Pw411).unwrap();
    multi_led
        .set_led_time_slots([
            TimeSlot::Led1,
            TimeSlot::Led2,
            TimeSlot::Led1,
            TimeSlot::Disabled,
        ])
        .unwrap();
    multi_led.enable_fifo_rollover().unwrap();
    let mut data = [0; 500];
    let interupt_status = multi_led.read_interrupt_status().unwrap();
    println!("Multi interupt status: {:?}", interupt_status);
    let mut samples_read = multi_led.read_fifo(&mut data).unwrap();

    println!("Sample read: {:?}", samples_read);
    multi_led.destroy();
    let i2c = I2c::new().unwrap();
    // let address = 0x57u16;
    // i2c.set_slave_address(address).unwrap();
    max30102 = Max3010x::new_max30102(i2c);
    let mut heart_hr = max30102.into_heart_rate().unwrap();
    heart_hr.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    heart_hr.set_pulse_amplitude(Led::All, 15).unwrap();
    heart_hr.set_sampling_rate(SamplingRate::Sps200).unwrap();
    heart_hr.set_pulse_width(LedPulseWidth::Pw411).unwrap();
    heart_hr.enable_fifo_rollover().unwrap();
    heart_hr.shutdown().unwrap();
    sleep(Duration::from_secs(2));
    heart_hr.wake_up().unwrap();
    let interupt_status = heart_hr.read_interrupt_status().unwrap();
    println!("HR interupt status: {:?}", interupt_status);
    samples_read = heart_hr.read_fifo(&mut data).unwrap();
    println!("Herat HR Sample read: {:?}", samples_read);
    heart_hr.destroy();
    let i2c = I2c::new().unwrap();
    // let address = 0x57u16;
    // i2c.set_slave_address(address).unwrap();
    max30102 = Max3010x::new_max30102(i2c);
    let mut spo2 = max30102.into_oximeter().unwrap();
    spo2.set_sample_averaging(SampleAveraging::Sa4).unwrap();
    spo2.set_pulse_amplitude(Led::All, 15).unwrap();
    spo2.set_sampling_rate(SamplingRate::Sps200).unwrap();
    spo2.set_pulse_width(LedPulseWidth::Pw411).unwrap();
    spo2.wake_up().unwrap();
    spo2.enable_fifo_rollover().unwrap();
    let interupt_status = spo2.read_interrupt_status().unwrap();
    println!("SPo2 interupt status: {:?}", interupt_status);
    samples_read = spo2.read_fifo(&mut data).unwrap();
    println!("SPo2 Sample read: {:?}", samples_read);
    spo2.shutdown().unwrap();
    spo2.destroy();
}
