use anyhow::Result;
use rppal::gpio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

const PINS: [u8; 10] = [1, 17, 26, 4, 5, 6, 7, 8, 12, 16];
const DELAY: u64 = 500;

fn init_leds() -> Result<Vec<gpio::OutputPin>> {
    let leds: gpio::Result<Vec<gpio::Pin>> = PINS
        .iter()
        .map(|pin| gpio::Gpio::new()?.get(*pin))
        .collect();

    let leds: Vec<gpio::OutputPin> = leds?.into_iter().map(|led| led.into_output()).collect();
    Ok(leds)
}

fn leds_off(leds: &mut [gpio::OutputPin]) {
    for led in leds.iter_mut() {
        led.set_high();
    }
}

fn run(leds: &mut [gpio::OutputPin]) {
    for led in leds.iter_mut() {
        led.set_low();
        sleep(Duration::from_millis(DELAY));
        led.set_high();
    }
    for led in leds.iter_mut().rev() {
        led.set_low();
        sleep(Duration::from_millis(DELAY));
        led.set_high();
    }
}

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut leds = init_leds()?;
    leds_off(&mut leds);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    while running.load(Ordering::SeqCst) {
        run(&mut leds);
    }

    leds_off(&mut leds);

    Ok(())
}
