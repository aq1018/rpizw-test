use embedded_hal::adc::{Channel, OneShot};
use embedded_hal::blocking::i2c::WriteRead;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub enum Reference {
    Internal = 0,
    External = 1,
}

pub struct ADS7830<I2C> {
    i2c: I2C,
    addr: u8,
    reference: Reference,
}

impl<I2C, E> ADS7830<I2C>
where
    I2C: WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, addr: u8, reference: Reference) -> Self {
        Self {
            i2c,
            addr,
            reference,
        }
    }
}

impl<WORD, CH, I2C, E> OneShot<ADS7830<I2C>, WORD, CH> for ADS7830<I2C>
where
    WORD: From<u8>,
    CH: Channel<ADS7830<I2C>, ID = u8>,
    I2C: WriteRead<Error = E>,
{
    type Error = E;

    fn read(&mut self, _ch: &mut CH) -> nb::Result<WORD, Self::Error> {
        // build command
        let ch = CH::channel();
        let pd1 = self.reference as u8;
        let pd0 = 1_u8;
        let cmd = 1 << 7 | ch << 4 | pd1 << 3 | pd0 << 2;
        let mut buf: [u8; 1] = [0];

        self.i2c.write_read(self.addr, &[cmd], &mut buf)?;

        Ok(buf[0].into())
    }
}

//marker trait
pub trait Analog {}
pub struct CH0(());
pub struct CH1(());
pub struct CH2(());
pub struct CH3(());
pub struct CH4(());
pub struct CH5(());
pub struct CH6(());
pub struct CH7(());

impl Analog for CH0 {}
impl Analog for CH1 {}
impl Analog for CH2 {}
impl Analog for CH3 {}
impl Analog for CH4 {}
impl Analog for CH5 {}
impl Analog for CH6 {}
impl Analog for CH7 {}

#[derive(Default)]
pub struct Single<P>(PhantomData<P>);

#[derive(Default)]
pub struct Differential<P, N>(PhantomData<P>, PhantomData<N>);

impl<P> Single<P>
where
    P: Analog,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P, N> Differential<P, N>
where
    P: Analog,
    N: Analog,
{
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH0, CH1>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0000_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH2, CH3>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0001_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH4, CH5>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0010_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH6, CH7>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0011_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH1, CH0>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0100_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH3, CH2>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0101_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH5, CH4>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0110_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Differential<CH7, CH6>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b0111_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH0>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1000_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH1>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1100_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH2>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1001_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH3>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1101_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH4>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1010_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH5>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1110_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH6>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1011_u8
    }
}

impl<I2C, E> Channel<ADS7830<I2C>> for Single<CH7>
where
    I2C: WriteRead<Error = E>,
{
    type ID = u8;

    fn channel() -> u8 {
        0b1111_u8
    }
}
