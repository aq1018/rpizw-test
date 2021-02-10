use core::marker::PhantomData;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

pub mod ic;

pub struct Motor<IN1, IN2, PWM, E, IC>
where
    IN1: OutputPin<Error = E>,
    IN2: OutputPin<Error = E>,
    PWM: PwmPin,
{
    in1: IN1,
    in2: IN2,
    pwm: PWM,
    _ic: PhantomData<IC>,
}

pub enum Command {
    ClockWise,
    CounterClockWise,
    Coast,
    Break,
}

impl<IN1, IN2, PWM, E, IC> Motor<IN1, IN2, PWM, E, IC>
where
    IN1: OutputPin<Error = E>,
    IN2: OutputPin<Error = E>,
    PWM: PwmPin,
{
    pub fn run(&mut self, dir: Command, duty: PWM::Duty) -> Result<(), E> {
        match dir {
            Command::ClockWise => {
                self.in1.set_high()?;
                self.in2.set_low()?;
            }
            Command::CounterClockWise => {
                self.in1.set_low()?;
                self.in2.set_high()?;
            }
            Command::Coast => {
                self.in1.set_low()?;
                self.in2.set_low()?;
            }
            Command::Break => {
                self.in1.set_high()?;
                self.in2.set_high()?;
            }
        }

        self.pwm.set_duty(duty);
        Ok(())
    }
}

impl<IN1, IN2, PWM, E> Motor<IN1, IN2, PWM, E, ic::L298>
where
    IN1: OutputPin<Error = E>,
    IN2: OutputPin<Error = E>,
    PWM: PwmPin,
{
    /// Creates a new `Motor`
    pub fn l298(mut in1: IN1, mut in2: IN2, mut pwm: PWM) -> Result<Self, E> {
        // initial state: brake
        in1.set_high()?;
        in2.set_high()?;

        pwm.enable();

        Ok(Self {
            in1,
            in2,
            pwm,
            _ic: PhantomData,
        })
    }
}

impl<IN1, IN2, PWM, E> Motor<IN1, IN2, PWM, E, ic::TB6612FNG>
where
    IN1: OutputPin<Error = E>,
    IN2: OutputPin<Error = E>,
    PWM: PwmPin,
{
    /// Creates a new `Motor`
    pub fn tb6612fng(mut in1: IN1, mut in2: IN2, mut pwm: PWM) -> Result<Self, E> {
        // initial state: brake
        in1.set_high()?;
        in2.set_high()?;

        pwm.enable();

        Ok(Self {
            in1,
            in2,
            pwm,
            _ic: PhantomData,
        })
    }
}
