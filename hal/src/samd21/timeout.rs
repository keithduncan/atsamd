use crate::{
    target_device,
    clock,
};

use core::ops::Deref;

pub type Divider = target_device::tc3::count16::ctrla::PRESCALER_A;

pub trait Count16 {
    fn count16(&self) -> &target_device::tc3::COUNT16;
}

pub struct Timeout<TC> {
    tc: TC,
}

impl<TC> Timeout<TC>
where TC: Count16 {
    pub fn enable_overflow_interrupt(&mut self) {
        self.tc.count16().intenset.write(|w| w.ovf().set_bit());
    }

    pub fn diable_overflow_interrupt(&mut self) {
        self.tc.count16().intenclr.write(|w| w.ovf().set_bit());
    }

    pub fn clear_overflow_interrupt(&mut self) {
        self.tc.count16().intflag.modify(|_, w| w.ovf().set_bit());
    }

    pub fn enable_overflow_event(&mut self) {
        self.tc.count16().evctrl.modify(|_, w| {
            w.ovfeo().set_bit()
        });
    }

    pub fn retrigger(&self) {
        self.tc.count16().ctrlbset.write(|w| {
            w.cmd().retrigger()
        });
        while self.tc.count16().status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }
    }

    pub fn count(&self) -> u16 {
        self.tc.count16().count.read().bits()
    }

    pub fn clear(&mut self) {
        self.tc.count16().count.reset()
    }
}

macro_rules! timer {
    ($($TYPE:ident: ($TC:ident, $pm:ident, $clock:ident),)+) => {
        $(
impl Count16 for target_device::$TC {
    fn count16(&self) -> &target_device::tc3::COUNT16 {
        let register: &target_device::tc3::RegisterBlock = self.deref();
        register.count16()
    }
}

impl Timeout<target_device::$TC>
where target_device::$TC: Count16
{
    pub fn $pm(_clock: &clock::$clock, tc: target_device::$TC, divider: Divider, start: u16, pm: &mut target_device::PM) -> Self {
        pm.apbcmask.modify(|_, w| w.$pm().set_bit());

        let counter = tc.count16();
        counter.evctrl.modify(|_, w| {
            w.tcei().set_bit();
            w.evact().retrigger()
        });

        counter.cc[0].write(|w| unsafe {
            w.bits(start)
        });
        while counter.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        // Ensure first retrigger starts from the top value
        counter.count.write(|w| unsafe {
            w.bits(start)
        });
        while counter.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        counter.ctrlbset.write(|w| {
            // Count down from TOP -> 0
            w.dir().set_bit();
            // Once, don't loop back
            w.oneshot().set_bit()
        });
        while counter.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        counter.ctrla.modify(|_, w| {
            w.runstdby().set_bit();
            w.prescaler().variant(divider);
            // Use cc[0] as TOP
            w.wavegen().mfrq();
            w.enable().set_bit()
        });
        while counter.status.read().syncbusy().bit_is_set() {
            cortex_m::asm::nop();
        }

        Self {
            tc,
        }
    }
}
        )+
    }
}

timer! {
    Timeout3: (TC3, tc3_, Tcc2Tc3Clock),
    Timeout4: (TC4, tc4_, Tc4Tc5Clock),
    Timeout5: (TC5, tc5_, Tc4Tc5Clock),
}
