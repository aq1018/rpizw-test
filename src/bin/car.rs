use anyhow::Result;
use gilrs::{Axis, Button, EventType, Gilrs};
use rpizw_test::devices::motor::{Command, Motor};
use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread::sleep, time::Duration};

// the servo SG90 uses 50 Hz frequency, so it's 1 / 50 = 0.02 s = 20 ms
const SERVO_PERIOD: Duration = Duration::from_millis(20);
const SERVO_PULSE_WIDTH_MIN: Duration = Duration::from_micros(500);
const SERVO_PULSE_WIDTH_MAX: Duration = Duration::from_micros(2500);
const SERVO_ANGLE_MIN: u32 = 0;
const SERVO_ANGLE_MAX: u32 = 180;
const SERVO_ANGLE_OFFSET: i32 = -10;

// motor
const MOTOR_IN_1: u8 = 13;
const MOTOR_IN_2: u8 = 26;
const MOTOR_FREQUENCY: f64 = 120.0;
const MOTOR_DUTY_CYCLE: f64 = 0.0;

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

    // servo
    let servo = Pwm::with_period(
        Channel::Pwm0,
        SERVO_PERIOD,
        angle_to_pulse_width(0),
        Polarity::Normal,
        true,
    )?;

    // motor
    let in1 = Gpio::new()?.get(MOTOR_IN_1)?.into_output();
    let in2 = Gpio::new()?.get(MOTOR_IN_2)?.into_output();
    let pwm = Pwm::with_frequency(
        Channel::Pwm1,
        MOTOR_FREQUENCY,
        MOTOR_DUTY_CYCLE,
        Polarity::Normal,
        true,
    )?;
    let mut motor = Motor::l298(in1, in2, pwm)?;

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
                    motor.run(Command::ClockWise, v as f64)?;
                }
                EventType::ButtonChanged(Button::RightTrigger2, v, ..) => {
                    println!("forward!, v={}", v);
                    motor.run(Command::CounterClockWise, v as f64)?;
                }
                EventType::ButtonPressed(Button::South, ..) => {
                    println!("break!");
                    motor.run(Command::Break, 1.0)?;
                }
                EventType::ButtonReleased(Button::South, ..) => {
                    println!("coast!");
                    motor.run(Command::Coast, 0.0)?;
                }
                _ => {}
            }
        }
        sleep(Duration::from_millis(10));
    }

    servo.set_pulse_width(axis_to_pulse_width(0.0))?;
    motor.run(Command::Coast, 0.0)?;
    sleep(Duration::from_millis(10));

    Ok(())
}
