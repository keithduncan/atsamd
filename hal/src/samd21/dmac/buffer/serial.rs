use core::ops::{Mul, Sub};

use embedded_hal::{
    blocking,
    serial::{Read, Write},
};

use super::{dmac, Consumer};

use generic_array::{
    self,
    typenum::{NonZero, Unsigned, U1},
    ArrayLength,
};

pub struct Serial<'a, S, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    serial: &'a mut S,
    consumer: Consumer<'a, N, X>,
}

impl<'a, S, N, X> Serial<'a, S, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    pub fn new(serial: &'a mut S, consumer: Consumer<'a, N, X>) -> Self {
        Self { serial, consumer }
    }
}

impl<'a, S, N, X, W> Write<u8> for Serial<'a, S, N, X>
where
    S: Write<u8, Error = W>,
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    type Error = <S as Write<u8>>::Error;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        self.serial.write(byte)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.serial.flush()
    }
}

impl<'a, S, N, X, W> blocking::serial::Write<u8> for Serial<'a, S, N, X>
where
    S: blocking::serial::Write<u8, Error = W>,
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    type Error = <S as blocking::serial::Write<u8>>::Error;

    fn bwrite_all(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.serial.bwrite_all(buffer)
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        self.serial.bflush()
    }
}

impl<'a, S, N, X> Read<u8> for Serial<'a, S, N, X>
where
    S: Read<u8>,
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    type Error = <S as Read<u8>>::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let grant = self.consumer.read();
            
        let slice = grant.as_slice();
        if slice.len() < 1 {
            grant.release(0);
            return Err(nb::Error::WouldBlock);
        }

        let byte = slice[0];
        grant.release(1);

        Ok(byte)
    }
}
