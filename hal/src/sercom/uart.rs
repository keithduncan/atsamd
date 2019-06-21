use crate::clock;
use crate::time::Hertz;
use crate::hal::blocking::serial::{write::Default, Write};
use crate::hal::serial;
use nb;
use crate::sercom::pads::*;
use crate::target_device::sercom0::USART;
use crate::target_device::Interrupt;
use crate::target_device::{NVIC, PM, SERCOM0, SERCOM1, SERCOM2, SERCOM3};
#[cfg(feature = "samd21g18a")]
use crate::target_device::{SERCOM4, SERCOM5};
use core::fmt;

/// The RxpoTxpo trait defines a way to get the data in and data out pin out
/// values for a given UARTXPadout configuration. You should not implement
/// this trait for yourself; only the implementations in the sercom module make
/// sense.
pub trait RxpoTxpo {
    fn rxpo_txpo(&self) -> (u8, u8);
}

#[derive(Debug)]
pub struct Status {
    pub parity_error: bool,
    pub frame_error: bool,
    pub buffer_overflow: bool,
    pub clear_to_send: bool,
    pub inconsistent_sync: bool,
    pub collision: bool,
}

/// Define a UARTX type for the given Sercom.
///
/// Also defines the valid "pad to uart function" mappings for this instance so
/// that construction is restricted to valid configurations.
macro_rules! uart {
    ($Type:ident: ($Sercom:ident, $SERCOM:ident, $powermask:ident, $clock:ident)) => {
        $crate::paste::item! {
            /// A pad mapping configuration for the SERCOM in UART mode.
            ///
            /// This type can only be constructed using the From implementations 
            /// in this module, which are restricted to valid configurations.
            ///
            /// Defines which sercom pad is mapped to which UART function.
            pub struct [<$Type Padout>]<RX, TX, RTS, CTS> {
                _rx: RX,
                _tx: TX,
                _rts: RTS,
                _cts: CTS,
            }
        }

        /// Define a From instance for either a tuple of two SercomXPadX
        /// instances, or a tuple of four SercomXPadX instances that converts
        /// them into an UARTXPadout instance.
        ///
        /// Also defines a RxpoTxpo instance for the constructed padout instance
        /// that returns the values used to configure the sercom pads for the
        /// appropriate function in the sercom register file.
        macro_rules! padout {
            ($rxpo_txpo:expr => $pad0:ident, $pad1:ident) => {
                $crate::paste::item! {
                    /// Convert from a tuple of (RX, TX) to UARTXPadout
                    impl<PIN0, PIN1> From<([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>)> for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, (), ()> {
                        fn from(pads: ([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>)) -> [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, (), ()> {
                            [<$Type Padout>] { _rx: pads.0, _tx: pads.1, _rts: (), _cts: () }
                        }
                    }

                    impl<PIN0, PIN1> RxpoTxpo for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, (), ()> {
                        fn rxpo_txpo(&self) -> (u8, u8) {
                            $rxpo_txpo
                        }
                    }
                }
            };
            ($rxpo_txpo:expr => $pad0:ident, $pad1:ident, $pad2:ident, $pad3:ident) => {
                $crate::paste::item! {
                    /// Convert from a tuple of (RX, TX, RTS, CTS) to UARTXPadout
                    impl<PIN0, PIN1, PIN2, PIN3> From<([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>)> for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>> {
                        fn from(pads: ([<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>)) -> [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>> {
                            [<$Type Padout>] { _rx: pads.0, _tx: pads.1, _rts: pads.2, _cts: pads.3 }
                        }
                    }

                    impl<PIN0, PIN1, PIN2, PIN3> RxpoTxpo for [<$Type Padout>]<[<$Sercom $pad0>]<PIN0>, [<$Sercom $pad1>]<PIN1>, [<$Sercom $pad2>]<PIN2>, [<$Sercom $pad3>]<PIN3>> {
                        fn rxpo_txpo(&self) -> (u8, u8) {
                            $rxpo_txpo
                        }
                    }
                }
            };
        }

        padout!((0, 1) => Pad0, Pad2);

        padout!((1, 0) => Pad1, Pad0);
        padout!((1, 2) => Pad1, Pad0, Pad2, Pad3);
        padout!((1, 1) => Pad1, Pad2);

        padout!((2, 0) => Pad2, Pad0);

        padout!((3, 0) => Pad3, Pad0);
        padout!((3, 1) => Pad3, Pad2);

        $crate::paste::item! {
            /// UARTX represents the corresponding SERCOMX instance
            /// configured to act in the role of a UART Master.
            /// Objects of this type implement the HAL `serial::Read`,
            /// `serial::Write` traits.
            ///
            /// This type is generic over any valid pad mapping where there is
            /// a defined "receive pin out transmit pin out" implementation.
            pub struct $Type<RX, TX, RTS, CTS> {
                padout: [<$Type Padout>]<RX, TX, RTS, CTS>,
                sercom: $SERCOM,
            }

            impl<RX, TX, RTS, CTS> $Type<RX, TX, RTS, CTS> {
                /// Power on and configure SERCOMX to work as a UART Master operating
                /// with the specified frequency. The padout specifies
                /// which pins are bound to the RX, TX and optionally RTS and CTS
                /// functions.
                ///
                /// You can use any tuple of two or four SercomXPadY instances
                /// for which there exists a From implementation for
                /// UARTXPadout.
                pub fn new<F: Into<Hertz>, T: Into<[<$Type Padout>]<RX, TX, RTS, CTS>>>(
                    clock: &clock::$clock,
                    freq: F,
                    sercom: $SERCOM,
                    nvic: &mut NVIC,
                    pm: &mut PM,
                    padout: T
                ) -> $Type<RX, TX, RTS, CTS> where
                    [<$Type Padout>]<RX, TX, RTS, CTS>: RxpoTxpo {
                    let padout = padout.into();

                    pm.apbcmask.modify(|_, w| w.$powermask().set_bit());

                    // Lots of union fields which require unsafe access
                    unsafe {
                        // Reset
                        sercom.usart().ctrla.modify(|_, w| w.swrst().set_bit());
                        while sercom.usart().syncbusy.read().swrst().bit_is_set()
                            || sercom.usart().ctrla.read().swrst().bit_is_set() {
                            // wait for sync of CTRLA.SWRST
                        }

                        // Unsafe b/c of direct call to bits on rxpo/txpo
                        sercom.usart().ctrla.modify(|_, w| {
                            w.dord().set_bit(); // LSB first

                            w.cmode().clear_bit(); // Comm. mode, asynchronous

                            w.form().bits(0x00); // USART frame, no parity

                            let (rxpo, txpo) = padout.rxpo_txpo();
                            w.rxpo().bits(rxpo);
                            w.txpo().bits(txpo);

                            w.sampr().bits(0x01); // 16x oversample fractional
                            w.runstdby().set_bit(); // Run in standby

                            w.mode().usart_int_clk() // Internal clock mode
                        });

                        // Asynchronous fractional mode (Table 24-2 in datasheet)
                        //   BAUD = fref / (sampleRateValue * fbaud)
                        // (multiply by 8, to calculate fractional piece)
                        let sample_rate: u32 = 16;
                        let fref = clock.freq().0;

                        let baud8x = (fref * 8) / (sample_rate * freq.into().0);
            
                        let fp = baud8x % 8;
                        let baud = baud8x / 8;
            
                        sercom.usart().baud_frac_mode().write(|w| {
                            w.fp().bits(fp as u8);
                            w.baud().bits(baud as u16)
                        });

                        sercom.usart().ctrlb.modify(|_, w| {
                            w.rxen().set_bit();
                            w.txen().set_bit();

                            w.sbmode().clear_bit(); // 0 is one stop bit see sec 25.8.2
                            w.chsize().bits(0x0) // 8 data bits
                        });

                        while sercom.usart().syncbusy.read().ctrlb().bit_is_set() {}

                        nvic.enable(Interrupt::$SERCOM);

                        sercom.usart().intenset.modify(|_, w| {
                            w.rxc().set_bit();
                            w.error().set_bit()
                            //w.txc().set_bit()
                            //w.dre().set_bit()
                        });

                        sercom.usart().ctrla.modify(|_, w| w.enable().set_bit());
                        // wait for sync of ENABLE
                        while sercom.usart().syncbusy.read().enable().bit_is_set() {}
                    }

                    Self {
                        padout,
                        sercom,
                    }
                }

                pub fn free(self) -> ([<$Type Padout>]<RX, TX, RTS, CTS>, $SERCOM) {
                    (self.padout, self.sercom)
                }

                pub fn has_error(&self) -> bool {
                    self.usart().intflag.read().error().bit_is_set()
                }

                pub fn has_data(&self) -> bool {
                    self.usart().intflag.read().rxc().bit_is_set()
                }

                pub fn clear_error(&mut self) {
                    self.usart().intflag.modify(|_, w| {
                        w.error().set_bit()
                    })
                }

                pub fn status(&self) -> Status {
                    let status = self.usart().status.read();

                    Status {
                        parity_error: status.perr().bit_is_set(),
                        frame_error: status.ferr().bit_is_set(),
                        buffer_overflow: status.bufovf().bit_is_set(),
                        clear_to_send: status.cts().bit_is_set(),
                        inconsistent_sync: status.isf().bit_is_set(),
                        collision: status.coll().bit_is_set(),
                    }
                }

                pub fn clear_status(&mut self) {
                    self.usart().status.reset()
                }

                pub fn clear_frame_error(&mut self) {
                    self.usart().status.modify(|_, w| {
                        w.ferr().set_bit()
                    });
                }

                fn usart(&self) -> &USART {
                    return &self.sercom.usart();
                }

                fn data_register_empty(&self) -> bool {
                    self.usart().intflag.read().dre().bit_is_set()
                }
            }

            impl<RX, TX, RTS, CTS> serial::Write<u8> for $Type<RX, TX, RTS, CTS> {
                type Error = ();

                fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
                    unsafe {
                        if !self.data_register_empty() {
                            return Err(nb::Error::WouldBlock);
                        }

                        self.usart().data.write(|w| {
                            w.bits(word as u16)
                        });
                    }

                    Ok(())
                }

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    // simply await DRE empty
                    if !self.data_register_empty() {
                        return Err(nb::Error::WouldBlock);
                    }

                    Ok(())
                }
            }

            impl<RX, TX, RTS, CTS> serial::Read<u8> for $Type<RX, TX, RTS, CTS> {
                type Error = ();

                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    let has_data = self.has_data();
                    if !has_data {
                        return Err(nb::Error::WouldBlock);
                    }

                    let data = self.usart().data.read().bits();

                    Ok(data as u8)
                }
            }

            impl<RX, TX, RTS, CTS> Default<u8> for $Type<RX, TX, RTS, CTS> {}

            impl<RX, TX, RTS, CTS> fmt::Write for $Type<RX, TX, RTS, CTS> {
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    self.bwrite_all(s.as_bytes()).map_err(|_| fmt::Error)
                }
            }
        }
    }
}

uart!(UART0: (Sercom0, SERCOM0, sercom0_, Sercom0CoreClock));
uart!(UART1: (Sercom1, SERCOM1, sercom1_, Sercom1CoreClock));
uart!(UART2: (Sercom2, SERCOM2, sercom2_, Sercom2CoreClock));
uart!(UART3: (Sercom3, SERCOM3, sercom3_, Sercom3CoreClock));
#[cfg(feature = "samd21g18a")]
uart!(UART4: (Sercom4, SERCOM4, sercom4_, Sercom4CoreClock));
#[cfg(feature = "samd21g18a")]
uart!(UART5: (Sercom5, SERCOM5, sercom5_, Sercom5CoreClock));
