use crate::{
    target_device,
    calibration,
};

use nb::{self, block};

pub type Input = target_device::adc::inputctrl::MUXPOS_A;

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

        new.set_input(Input::TEMP);
        new.start();
        let _result = block!(new.wait());
        new.clear_ready_interrupt();

        new
    }

    pub fn set_input(&mut self, input: Input) {
        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.inputctrl.modify(|_, w| {
            w.muxpos().variant(input)
        });
    }

    // TODO take AIN[x] as an argument?
    pub fn start(&mut self) {
        while self.adc.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.adc.swtrig.write(|w| w.start().set_bit());
    }

    pub fn wait(&mut self) -> Result<u16, nb::Error<()>> {
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
