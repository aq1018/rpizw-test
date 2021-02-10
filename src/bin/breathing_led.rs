use anyhow::Result;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

const DELAY: u64 = 30;
const FREQUENCY: f64 = 120.0;
const DUTY_CYCLE: f64 = 0.0;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let led = Pwm::with_frequency(Channel::Pwm0, FREQUENCY, DUTY_CYCLE, Polarity::Normal, true)?;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {
        for i in 0..100 {
            led.set_duty_cycle(i as f64 / 100.0)?;
            sleep(Duration::from_millis(DELAY));
        }
        for i in (1..99).rev() {
            led.set_duty_cycle(i as f64 / 100.0)?;
            sleep(Duration::from_millis(DELAY));
        }
    }

    Ok(())
}
