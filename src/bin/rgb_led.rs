use anyhow::Result;
use rppal::gpio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

struct ColorLED {
    r: gpio::OutputPin,
    g: gpio::OutputPin,
    b: gpio::OutputPin,
    frequency: f64,
}

fn init_soft_pwm(pin: u8, frequency: f64) -> Result<gpio::OutputPin> {
    let mut pwm = gpio::Gpio::new()?.get(pin)?.into_output();
    pwm.set_pwm_frequency(frequency, 0.0)?;
    Ok(pwm)
}

impl ColorLED {
    fn new(r: u8, g: u8, b: u8, frequency: f64) -> Result<ColorLED> {
        Ok(ColorLED {
            r: init_soft_pwm(r, frequency)?,
            g: init_soft_pwm(g, frequency)?,
            b: init_soft_pwm(b, frequency)?,
            frequency,
        })
    }

    fn set_rgb(&mut self, r: f64, g: f64, b: f64) -> Result<()> {
        self.r.set_pwm_frequency(self.frequency, r)?;
        self.g.set_pwm_frequency(self.frequency, g)?;
        self.b.set_pwm_frequency(self.frequency, b)?;

        Ok(())
    }
}

fn run(led: &mut ColorLED) -> Result<()> {
    let r: f64 = rand::random();
    let g: f64 = rand::random();
    let b: f64 = rand::random();

    led.set_rgb(r, g, b)?;

    sleep(Duration::from_millis(1000));

    Ok(())
}

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut led = ColorLED::new(16, 20, 21, 100.0)?;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {
        run(&mut led)?;
    }

    Ok(())
}
