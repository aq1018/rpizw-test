use anyhow::{Context, Result};
use embedded_hal::adc::OneShot;
use rpizw_test::devices::ads7830::{Reference, Single, ADS7830, CH0};
use rpizw_test::utils::convert_nb_error;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

const ADC_ADDR: u8 = 0x4b;
const DELAY: u64 = 30;
const FREQUENCY: f64 = 100.0;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let led = Pwm::with_frequency(Channel::Pwm0, FREQUENCY, 0.0, Polarity::Normal, true)?;
    let i2c = I2c::new().context("Failed to init I2C")?;
    let mut adc = ADS7830::new(i2c, ADC_ADDR, Reference::Internal);
    let mut ch: Single<CH0> = Single::new();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {
        match convert_nb_error(adc.read(&mut ch)).context("Cannot read ADC")? {
            None => println!("Would Block"),
            Some(v) => {
                println!("ADC Value {}", v);

                led.set_duty_cycle(v as f64 / 255.0)?;
                sleep(Duration::from_millis(DELAY));
            }
        }
    }

    Ok(())
}
