use anyhow::{Context, Result};
use embedded_hal::adc::OneShot;
use rpizw_test::devices::ads7830::{Reference, Single, ADS7830, CH0};
use rpizw_test::devices::motor::{Command, Motor};
use rpizw_test::utils::convert_nb_error;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::cmp;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

const ADC_ADDR: u8 = 0x4b;
const DELAY: u64 = 100;

const MOTOR_IN_1: u8 = 27;
const MOTOR_IN_2: u8 = 17;
const FREQUENCY: f64 = 120.0;
const DUTY_CYCLE: f64 = 0.0;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // knob
    let i2c = I2c::new().context("Failed to init I2C")?;
    let mut adc = ADS7830::new(i2c, ADC_ADDR, Reference::Internal);
    let mut ch: Single<CH0> = Single::new();

    // motor
    let in1 = Gpio::new()?.get(MOTOR_IN_1)?.into_output();
    let in2 = Gpio::new()?.get(MOTOR_IN_2)?.into_output();
    let pwm = Pwm::with_frequency(Channel::Pwm0, FREQUENCY, DUTY_CYCLE, Polarity::Normal, true)?;

    let mut motor = Motor::l298(in1, in2, pwm)?;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {
        match convert_nb_error(adc.read(&mut ch)).context("Cannot read ADC")? {
            None => println!("Would Block"),
            Some(v) => {
                println!("ADC: {}", v);
                let v = v as i32 - 128;
                let duty = i32::abs(v) as f64 / 128.0;
                let cmd = match v.cmp(&0) {
                    cmp::Ordering::Greater => Command::ClockWise,
                    cmp::Ordering::Less => Command::CounterClockWise,
                    cmp::Ordering::Equal => Command::Coast,
                };

                motor.run(cmd, duty)?;
                sleep(Duration::from_millis(DELAY));
            }
        }
    }

    Ok(())
}
