use crate::target_device;

use core::ops::{
	Deref,
	DerefMut,
};

pub mod descriptor;
pub use descriptor::Descriptor;

pub mod channel;
pub use channel::Channel;

pub mod buffer;

pub static mut BASE_DESCRIPTORS: [Descriptor; 12] = [descriptor::default(); 12];
pub static mut WRITEBACK_DESCRIPTORS: [Descriptor; 12] = [descriptor::default(); 12];

pub struct DMAC {
	pub dmac: target_device::DMAC,
}

impl Deref for DMAC {
	type Target = target_device::DMAC;

	fn deref(&self) -> &Self::Target {
		&self.dmac
	}
}

impl DerefMut for DMAC {
	fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
		&mut self.dmac
	}
}

impl DMAC {
	pub fn init(pm: &mut target_device::PM, dmac: target_device::DMAC) -> Self {
		pm.ahbmask.modify(|_, w| w.dmac_().set_bit());
		pm.apbbmask.modify(|_, w| w.dmac_().set_bit());

		dmac.ctrl.write(|w| {
			w.swrst().set_bit()
		});

		dmac.baseaddr.write(|w| {
			unsafe { w.bits(&BASE_DESCRIPTORS as *const Descriptor as u32) }
		});
		dmac.wrbaddr.write(|w| {
			unsafe { w.bits(&mut WRITEBACK_DESCRIPTORS[0] as *mut Descriptor as u32) }
		});

		dmac.ctrl.modify(|_, w| {
			w.dmaenable().set_bit();

			w.lvlen3().set_bit();
			w.lvlen2().set_bit();
			w.lvlen1().set_bit();
			w.lvlen0().set_bit()
		});

		dmac.dbgctrl.modify(|_, w| {
			w.dbgrun().set_bit()
		});

		Self {
			dmac,
		}
	}

	pub fn channel(&mut self, channel: Channel, source: channel::TriggerSource, action: channel::Action, events: channel::Events, interrupts: channel::Interrupts) {
		self.dmac.chid.write(|w| unsafe {
			w.bits(channel as u8)
		});
		self.dmac.chctrlb.write(|w| {
			w.lvl().lvl0();

			unsafe { w.trigsrc().bits(source as u8); }
			w.trigact().variant(action);

			if events.output {
				// TODO provide this in the type system, only channels 0-3
				// are connected to the event system
				assert!((channel as u8) < 4);

				w.evoe().set_bit();
			}

			if let Some(input) = events.input {
				// TODO provide this in the type system, only channels 0-3
				// are connected to the event system
				assert!((channel as u8) < 4);

				w.evie().set_bit();
				w.evact().variant(input);
			}

			w
		});
		self.dmac.chctrla.write(|w| {
			w.enable().set_bit()
		});

		self.dmac.chintenset.write(|w| {
			if interrupts.suspend {
				w.susp().set_bit();
			}

			if interrupts.transfer_complete {
				w.tcmpl().set_bit();
			}
			
			if interrupts.transfer_error {
				w.terr().set_bit();
			}

			w
		});
	}

	pub fn trigger(&mut self, channel: Channel) {
		self.dmac.swtrigctrl.write(|w| {
			match channel {
				Channel::Channel0 => w.swtrig0().set_bit(),
				Channel::Channel1 => w.swtrig1().set_bit(),
				Channel::Channel2 => w.swtrig2().set_bit(),
				Channel::Channel3 => w.swtrig3().set_bit(),
				Channel::Channel4 => w.swtrig4().set_bit(),
				Channel::Channel5 => w.swtrig5().set_bit(),
				Channel::Channel6 => w.swtrig6().set_bit(),
				Channel::Channel7 => w.swtrig7().set_bit(),
				Channel::Channel8 => w.swtrig8().set_bit(),
				Channel::Channel9 => w.swtrig9().set_bit(),
				Channel::Channel10 => w.swtrig10().set_bit(),
				Channel::Channel11 => w.swtrig11().set_bit(),
			}
		});
	}

	pub fn interrupt(&mut self) {
		let channels = self.dmac.intstatus.read().bits();
        if channels == 0 {
            return;
        }

        for channel in 0..12u16 {
			let mask = 1 << channel;
			if (channels & mask) == 0 {
				continue;
			}

			self.dmac.chid.write(|w| unsafe {
				w.bits(channel as u8)
			});

			self.dmac.chintflag.modify(|r, w| {
				if r.susp().bit_is_set() {
					w.susp().set_bit();
				}

				if r.tcmpl().bit_is_set() {
					w.tcmpl().set_bit();
				}

				if r.terr().bit_is_set() {
					w.terr().set_bit();
				}

				w
			})
		}
	}
}
