use embedded_hal::digital::v2::OutputPin;
use std::{thread::sleep, time::Duration};

#[derive(Copy, Clone, PartialEq)]
pub enum Dir {
    CCW,
    CW,
}

pub struct StepperMotor<'a, PIN, E>
where
    PIN: OutputPin<Error = E>,
{
    pins: &'a mut [PIN],
    pos: usize,
    dir: Dir,
    delay: Duration,
}

impl<'a, PIN, E> StepperMotor<'a, PIN, E>
where
    PIN: OutputPin<Error = E>,
{
    pub fn new(
        pins: &'a mut [PIN],
        pos: usize,
        dir: Dir,
        delay: Duration,
    ) -> Result<StepperMotor<'a, PIN, E>, E> {
        for pin in pins.iter_mut() {
            pin.set_low()?;
        }
        pins[pos].set_high()?;
        sleep(delay);

        Ok(Self {
            pins,
            pos,
            dir,
            delay,
        })
    }

    pub fn set_dir(&mut self, dir: Dir) {
        self.dir = dir;
    }

    pub fn step(&mut self) -> Result<(), E> {
        // update state
        let curr = self.pos;
        let next = self.next_pos();

        self.pins[curr].set_low()?;
        self.pins[next].set_high()?;
        self.pos = next;

        sleep(self.delay);

        Ok(())
    }

    fn next_pos(&mut self) -> usize {
        match self.dir {
            Dir::CW => {
                if self.pos == self.pins.len() - 1 {
                    0
                } else {
                    self.pos + 1
                }
            }
            Dir::CCW => {
                if self.pos == 0 {
                    self.pins.len() - 1
                } else {
                    self.pos - 1
                }
            }
        }
    }
}
