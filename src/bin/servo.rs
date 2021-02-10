use anyhow::Result;
use gilrs::{Axis, Button, EventType, Gilrs};
use rppal::pwm::{Channel, Polarity, Pwm};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

const DELAY: Duration = Duration::from_millis(100);

// the servo SG90 uses 50 Hz frequency, so it's 1 / 50 = 0.02 s = 20 ms
const SERVO_PERIOD: Duration = Duration::from_millis(20);
const SERVO_PULSE_WIDTH_MIN: Duration = Duration::from_micros(500);
const SERVO_PULSE_WIDTH_MAX: Duration = Duration::from_micros(2500);
const SERVO_ANGLE_MIN: u32 = 0;
const SERVO_ANGLE_MAX: u32 = 180;
const SERVO_ANGLE_OFFSET: i32 = -10;

// our car only can turn from -25 to 25 degrees ( using 0 as servo's 90 degrees)
const STEERING_MIN_ANGLE: i32 = -30;
const STEERING_MAX_ANGLE: i32 = 30;

fn angle_to_pulse_width(angle: i32) -> Duration {
    let angle = match angle {
        a if a > STEERING_MAX_ANGLE => STEERING_MAX_ANGLE,
        a if a < STEERING_MIN_ANGLE => STEERING_MIN_ANGLE,
        _ => angle,
    };

    // convert to servo angle
    let servo_angle: u32 = (90 + SERVO_ANGLE_OFFSET - angle) as u32;
    let ratio =
        (SERVO_PULSE_WIDTH_MAX - SERVO_PULSE_WIDTH_MIN) / (SERVO_ANGLE_MAX - SERVO_ANGLE_MIN);

    SERVO_PULSE_WIDTH_MIN + ratio * servo_angle
}

fn axis_to_pulse_width(v: f32) -> Duration {
    let ratio = (STEERING_MAX_ANGLE - STEERING_MIN_ANGLE) as f32 / 2.0;
    let angle = -1.0 + ratio * v;

    angle_to_pulse_width(angle as i32)
}

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let pw = angle_to_pulse_width(0);
    let servo = Pwm::with_period(Channel::Pwm0, SERVO_PERIOD, pw, Polarity::Normal, true)?;

    while running.load(Ordering::SeqCst) {
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::AxisChanged(Axis::LeftStickX, v, ..) => {
                    let pw = axis_to_pulse_width(v);
                    println!("steer!, v={}, pw={:?}", v, pw);
                    servo.set_pulse_width(pw)?;
                }
                EventType::ButtonChanged(Button::LeftTrigger2, v, ..) => {
                    println!("reverse!, v={}", v);
                }

                EventType::ButtonChanged(Button::RightTrigger2, v, ..) => {
                    println!("forward!, v={}", v);
                }
                _ => {}
            }
        }
        sleep(Duration::from_millis(10));
    }

    Ok(())
}
