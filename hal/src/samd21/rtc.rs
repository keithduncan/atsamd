use crate::{
	target_device,
	clock,
	time::*,
};

pub struct RTC {
	rtc: target_device::RTC,
}

impl RTC {
	pub fn five_minutes(pm: &mut target_device::PM, clock: clock::RtcClock, rtc: target_device::RTC) -> Self {
		Self::init(pm, clock, rtc, 9_600)
	}

	pub fn ten_seconds(pm: &mut target_device::PM, clock: clock::RtcClock, rtc: target_device::RTC) -> Self {
		Self::init(pm, clock, rtc, 312)
	}

	fn init(pm: &mut target_device::PM, clock: clock::RtcClock, rtc: target_device::RTC, count: u32) -> Self {
		let hz: Hertz = clock.into();
		assert!(hz == 32_000.hz());

		pm.apbamask.modify(|_, w| w.rtc_().set_bit());

		let mode = rtc.mode0_mut();

		// Disable and reset rtc
		mode.ctrl.modify(|_, w| {
			w.enable().clear_bit()
		});
		while mode.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
		}

		mode.ctrl.modify(|_, w| {
			w.swrst().set_bit()
		});
		while mode.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
		}

		// Configure prescaler
		mode.ctrl.write(|w| {
			w.matchclr().set_bit();
			w.mode().count32();
			w.prescaler().div1024()
		});
		while mode.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
		}

		mode.comp[0].write(|w| unsafe {
			w.bits(count)
		});
		while mode.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
		}

		mode.intenset.modify(|_, w| {
			w.cmp0().set_bit()
		});
		while mode.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
		}

		// Enable
		mode.ctrl.modify(|_, w| {
			w.enable().set_bit()
		});
		while mode.status.read().syncbusy().bit_is_set() {
			cortex_m::asm::nop();
		}

		RTC {
			rtc: rtc,
		}
	}

	pub fn clear_interrupt(&mut self) {
		self.rtc.mode0_mut().intflag.modify(|_, w| {
			w.cmp0().set_bit()
		})
	}
}
