use core::ops::Deref;

use crate::{
	target_device,
	clock,
};

pub trait Count16 {
    fn count16(&self) -> &target_device::tc3::COUNT16;
}

pub struct EventCounter<TC> {
	tc: TC,
}

impl<TC> EventCounter<TC>
where TC: Count16 {
	pub fn enable_overflow_interrupt(&mut self) {
		self.tc.count16().intenset.modify(|_, w| w.ovf().set_bit());
	}

	pub fn diable_overflow_interrupt(&mut self) {
		self.tc.count16().intenclr.modify(|_, w| w.ovf().set_bit());
	}

	pub fn clear_overflow_interrupt(&mut self) {
        self.tc.count16().intflag.modify(|_, w| w.ovf().set_bit());
    }

    pub fn enable_overflow_event(&mut self) {
        self.tc.count16().evctrl.modify(|_, w| w.ovfeo().set_bit());
    }

	pub fn count(&self) -> u16 {
		self.tc.count16().count.read().bits()
	}

	pub fn clear(&mut self) {
		self.tc.count16().count.reset()
	}
}

macro_rules! ec {
    ($($TYPE:ident: ($TC:ident, $pm:ident, $clock:ident),)+) => {
        $(
impl Count16 for target_device::$TC {
	fn count16(&self) -> &target_device::tc3::COUNT16 {
        let register: &target_device::tc3::RegisterBlock = self.deref();
        register.count16()
    }
}

impl EventCounter<target_device::$TC>
where target_device::$TC: Count16
{
    pub fn $pm(_clock: &clock::$clock, tc: target_device::$TC, max: u16, pm: &mut target_device::PM) -> Self {
        pm.apbcmask.modify(|_, w| w.$pm().set_bit());

	    let counter = tc.count16();
	    counter.evctrl.modify(|_, w| {
	        w.tcei().set_bit();
	        w.evact().count()
	    });

	    counter.cc[0].write(|w| unsafe {
			w.bits(max)
	    });
	    while counter.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
	    }

	    counter.ctrla.modify(|_, w| {
			w.runstdby().set_bit();
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

ec! {
    EventCounter3: (TC3, tc3_, Tcc2Tc3Clock),
    EventCounter4: (TC4, tc4_, Tc4Tc5Clock),
    EventCounter5: (TC5, tc5_, Tc4Tc5Clock),
}
