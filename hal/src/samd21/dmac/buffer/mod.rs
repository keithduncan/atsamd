use core::{
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Mul, Sub},
    ptr::{self, NonNull},
};

use crate::dmac;

use generic_array::{
    self,
    typenum::{NonZero, Unsigned, U1},
    ArrayLength, GenericArray,
};

pub mod serial;
pub use serial::Serial;

pub struct Buffer<N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    // Descriptors, one fewer than given due to dmac::BASE_DESCRIPTORS
    descriptors: MaybeUninit<GenericArray<dmac::Descriptor, <N as Sub<U1>>::Output>>,
    // Backing Store, X bytes per descriptor
    data: MaybeUninit<GenericArray<u8, <N as Mul<X>>::Output>>,

    // Configured marker
    configured: Option<()>,

    // Currently writing block
    write_block: usize,
    // Where to write to
    write: usize,

    // Where to read from
    read: usize,
}

impl<N, X> Buffer<N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    pub fn new() -> Self {
        Buffer {
            descriptors: MaybeUninit::uninit(),
            data: MaybeUninit::uninit(),

            configured: Some(()),

            write_block: 0,
            write: 0,
            read: 0,
        }
    }

    /// Initialise the descriptors for this static buffer
    ///
    /// descriptors: must have enough space to hold (backing.len() / X) - 1 descriptors
    pub fn configure<'a, F>(
        &mut self,
        _dmac: &mut dmac::DMAC,
        channel: dmac::Channel,
        mut f: F,
    ) -> Result<(Producer<'a, N, X>, Consumer<'a, N, X>), ()>
    where
        F: FnMut(*mut u8, u16, *const dmac::Descriptor) -> dmac::Descriptor,
    {
        self.configured.take().expect("only configured once");

        let descriptors_ptr: *mut dmac::Descriptor =
            self.descriptors.as_mut_ptr() as *mut dmac::Descriptor;
        let descriptors_len = <N as Sub<U1>>::Output::to_usize();

        let data_ptr: *mut u8 = self.data.as_mut_ptr() as *mut u8;
        let _data_len = <N as Mul<X>>::Output::to_usize();

        unsafe {
            // First descriptor, perhaps only descriptor
            dmac::BASE_DESCRIPTORS[channel as usize] = f(
                data_ptr,
                X::to_u16(),
                if descriptors_len == 0 {
                    &dmac::BASE_DESCRIPTORS[channel as usize]
                } else {
                    descriptors_ptr
                },
            );

            for desc_idx in 0usize..descriptors_len {
                let dst = data_ptr.offset(X::to_isize() * (desc_idx as isize + 1));
                let count = X::to_u16();

                let next = if desc_idx < (descriptors_len - 1) {
                    // Next descriptor
                    descriptors_ptr.offset((desc_idx + 1) as isize)
                } else {
                    // Loop to the first descriptor
                    &dmac::BASE_DESCRIPTORS[channel as usize] as *const _
                };

                *descriptors_ptr.offset(desc_idx as isize) = f(dst, count, next);
            }

            Ok((
                Producer {
                    buf: NonNull::new_unchecked(self as *mut _),
                    pd: PhantomData,
                },
                Consumer {
                    buf: NonNull::new_unchecked(self as *mut _),
                    pd: PhantomData,
                },
            ))
        }
    }
}

pub struct Producer<'a, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    buf: NonNull<Buffer<N, X>>,
    pd: PhantomData<&'a ()>,
}

impl<'a, N, X> Producer<'a, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    /// Inform the buffer a descriptor is partially complete.
    ///
    /// used: The total transfer of the current descriptor. A descriptor
    ///       may notify partial more than once.
    pub fn notify_descriptor_partial(&self, remaining: u16) {
        unsafe {
            let buf = self.buf.as_ptr();

            let written = ptr::read_volatile(&(*buf).write);
            let already_written =
                written as usize - ptr::read_volatile(&(*buf).write_block) as usize;

            let newly_written = (X::to_u16() - remaining) - already_written as u16;

            ptr::write_volatile(
                &mut (*buf).write as *mut _,
                written + newly_written as usize,
            );
        }
    }

    /// Advance the buffer the entire inprogress block is complete
    pub fn notify_descriptor(&self) {
        unsafe {
            let buf = self.buf.as_ptr();

            // These will always be aligned to the X byte blocks
            let write_block_start = ptr::read_volatile(&(*buf).write_block);
            let write_block_end = write_block_start + X::to_usize();

            let buf_len = <N as Mul<X>>::Output::to_usize();

            // Set write_block to the start of the next block to write
            let next_block = if write_block_end >= buf_len {
                // Back to the beginning
                0
            } else {
                // Next block
                write_block_end
            };

            ptr::write_volatile(&mut (*buf).write_block as *mut _, next_block);
            ptr::write_volatile(&mut (*buf).write as *mut _, next_block);
        }
    }
}

pub struct Consumer<'a, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    buf: NonNull<Buffer<N, X>>,
    pd: PhantomData<&'a ()>,
}

impl<'a, N, X> Consumer<'a, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    pub fn read(&self) -> GrantR<N, X> {
        let slice = unsafe {
            let buf = self.buf.as_ptr();

            let data_ptr: *mut u8 = (*buf).data.as_mut_ptr() as *mut u8;
            let data_len = <N as Mul<X>>::Output::to_usize();

            let write = ptr::read_volatile(&(*buf).write);
            let read = ptr::read_volatile(&(*buf).read);

            if write == read {
                core::slice::from_raw_parts(NonNull::dangling().as_ptr(), 0)
            } else if write > read {
                core::slice::from_raw_parts(data_ptr.offset(read as isize), write - read)
            } else {
                core::slice::from_raw_parts(data_ptr.offset(read as isize), data_len - read)
            }
        };

        GrantR {
            buf: self.buf,
            slice,
        }
    }
}

pub struct GrantR<'a, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    buf: NonNull<Buffer<N, X>>,
    slice: &'a [u8],
}

impl<'a, N, X> GrantR<'a, N, X>
where
    N: NonZero + Unsigned,
    X: NonZero + Unsigned,
    N: Sub<U1> + Mul<X>,
    <N as Sub<U1>>::Output: ArrayLength<dmac::Descriptor>,
    <N as Mul<X>>::Output: ArrayLength<u8>,
{
    pub fn as_slice(&self) -> &'a [u8] {
        self.slice
    }

    /// Inform the buffer the given number of bytes are now free to be reused
    pub fn release(self, used: usize) {
        unsafe {
            let buf = self.buf.as_ptr();

            let most = core::cmp::min(used, self.slice.len());
            let data_len = <N as Mul<X>>::Output::to_usize();

            let mut read = ptr::read_volatile(&(*buf).read) + most;

            if read >= data_len {
                read = 0;
            }

            ptr::write_volatile(&mut (*buf).read as *mut _, read);
        }
    }
}
