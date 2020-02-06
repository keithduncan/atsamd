use crate::{
    target_device,
    gpio,
    calibration,
};

use nb::{self, block};

pub struct ADC {
    adc: target_device::ADC,
}

impl ADC {
    pub fn init(pm: &mut target_device::PM, _clock: crate::clock::AdcClock, adc: target_device::ADC) -> Self {
        adc.calib.modify(|_, w| {
            unsafe {
                w.linearity_cal().bits(calibration::adc_linearity_cal());
                w.bias_cal().bits(calibration::adc_bias_cal())
            }
        });

        pm.apbcmask.modify(|_, w| w.adc_().set_bit());

        while adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        adc.ctrlb.modify(|_, w| {
            w.prescaler().div512();
            w.ressel()._10bit()
        });

        adc.sampctrl.write(|w| unsafe {
            w.samplen().bits(0b11_1111)
        });

        while adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        adc.inputctrl.modify(|_, w| w.muxneg().gnd());

        adc.avgctrl.modify(|_, w| {
            unsafe {
                w.adjres().bits(0);
            }
            w.samplenum()._1()
        });

        // Reference
        adc.inputctrl.modify(|_, w| w.gain().div2());
        adc.refctrl.modify(|_, w| w.refsel().intvcc1());

        while adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        adc.ctrla.modify(|_, w| {
            w.enable().set_bit();
            w.runstdby().set_bit()
        });

        /*
            First read
        */

        let mut new = ADC {
            adc,
        };

        new.start(&mut InternalTemp);
        let _result = block!(new.wait());
        new.clear_ready_interrupt();

        new
    }

    pub fn set_input<PIN: Channel<Self, ID = Input>>(&mut self, _input: &mut PIN) {
        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.inputctrl.modify(|_, w| {
            w.muxpos().variant(<PIN as Channel<Self>>::CHANNEL)
        });
    }

    pub fn start<PIN: Channel<Self, ID = Input>>(&mut self, input: &mut PIN) {
        self.set_input(input);

        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.swtrig.write(|w| w.start().set_bit());
    }

    pub fn wait(&mut self) -> Result<u16, nb::Error<core::convert::Infallible>> {
        if self.adc.intflag.read().resrdy().bit_is_set() {
            Ok(self.result())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn result(&mut self) -> u16 {
        self.adc.result.read().bits()
    }

    pub fn enable_ready_interrupt(&mut self) {
        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.intenset.modify(|_, w| w.resrdy().set_bit());
    }

    pub fn clear_ready_interrupt(&mut self) {
        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.intflag.modify(|_, w| w.resrdy().set_bit());
    }

    pub fn enable_start_conversion_event(&mut self) {
        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.evctrl.modify(|_, w| w.startei().set_bit());
    }
}

type Input = target_device::adc::inputctrl::MUXPOS_A;

pub trait Channel<ADC> {
    /// Channel ID type
    ///
    /// A type used to identify this ADC channel. For example, if the ADC has eight channels, this
    /// might be a `u8`. If the ADC has multiple banks of channels, it could be a tuple, like
    /// `(u8: bank_id, u8: channel_id)`.
    type ID;

    // `channel` is a function due to [this reported
    // issue](https://github.com/rust-lang/rust/issues/54973). Something about blanket impls
    // combined with `type ID; const CHANNEL: Self::ID;` causes problems.
    const CHANNEL: Self::ID;
}

pub struct InternalTemp;

impl Channel<ADC> for InternalTemp {
    type ID = Input;
    const CHANNEL: Self::ID = Input::TEMP;
}

macro_rules! adc_pins {
    ($($pin:ident: $chan:expr,)+) => {
        $(
            impl Channel<ADC> for gpio::$pin<gpio::PfB> {
                type ID = Input;
                const CHANNEL: Self::ID = $chan;
            }
        )+
    }
}

adc_pins! {
    Pa2: Input::PIN0,
    Pa3: Input::PIN1,
    Pa4: Input::PIN4,
    Pa5: Input::PIN5,
    Pa6: Input::PIN6,
    Pa7: Input::PIN7,
    Pa8: Input::PIN16,
    Pa9: Input::PIN17,
    Pa10: Input::PIN18,
    Pa11: Input::PIN19,
}

#[cfg(any(feature = "samd21g18a", feature = "samd21j18a"))]
adc_pins! {
    Pb0: Input::PIN8,
    Pb1: Input::PIN9,
    Pb2: Input::PIN10,
    Pb3: Input::PIN11,
    Pb4: Input::PIN12,
    Pb5: Input::PIN13,
    Pb6: Input::PIN14,
    Pb7: Input::PIN15,
    Pb8: Input::PIN2,
    Pb9: Input::PIN3,
}
