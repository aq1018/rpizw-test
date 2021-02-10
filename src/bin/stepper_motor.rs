use rpizw_test::devices::stepper_motor::{Dir, StepperMotor};
use rppal::gpio::Gpio;
use std::{
    result::Result,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

const PINS: [u8; 4] = [18, 23, 24, 25];
const DELAY: Duration = Duration::from_millis(3);

fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // initialize
    let mut pins = PINS
        .iter()
        .map(|pin| Gpio::new()?.get(*pin))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|pin| pin.into_output())
        .collect::<Vec<_>>();

    let mut stepper = StepperMotor::new(&mut pins, 0, Dir::CCW, DELAY)?;

    while running.load(Ordering::SeqCst) {
        stepper.step()?;
    }

    Ok(())
}
