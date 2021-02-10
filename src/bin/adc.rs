use embedded_hal::adc::OneShot;
use rpizw_test::devices::ads7830::{Reference, Single, ADS7830, CH0};
use rpizw_test::utils::convert_nb_error;
use rppal::i2c::I2c;

use anyhow::{Context, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

const ADC_ADDR: u8 = 0x4b;
const DELAY: u64 = 100;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

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
                let voltage = v as f64 / 255.0 * 3.3;
                println!("ADC Value {}, voltage: {}", v, voltage);
                sleep(Duration::from_millis(DELAY));
            }
        }
    }

    Ok(())
}
