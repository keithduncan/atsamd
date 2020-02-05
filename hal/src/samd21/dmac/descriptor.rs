use core::{
	default::Default,
	ptr,
};

mod btctrl;
pub use btctrl::{
	Btctrl,
	Stepsize,
	Stepsel,
	Beatsize,
	BlockAction,
	EventOutputAction,
};

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Descriptor {
	pub btctrl: Btctrl,
	pub btcnt: u16,
	pub srcaddr: *const u8,
	pub dstaddr: *mut u8,
	pub descaddr: *const Descriptor,
}

pub const fn default() -> Descriptor {
	Descriptor {
		btctrl: Btctrl(0),
		btcnt: 0,
		srcaddr: ptr::null(),
		dstaddr: ptr::null::<u8>() as *mut u8,
		descaddr: ptr::null(),
	}
}

impl Default for Descriptor {
	fn default() -> Self {
		default()
	}
}

impl Descriptor {
	/// Copy 1 u32 from src to len u32 values in dst
	pub fn memset_u32_pattern1(src: *const u32, dst: *mut u32, len: u16) -> Self {
		Descriptor {
			btctrl: {
				let mut btctrl = Btctrl(0);
				btctrl.set_stepsize(Stepsize::X1);
				btctrl.set_stepsel(Stepsel::StepDst);
				btctrl.set_dstinc(true);
				btctrl.set_srcinc(false);
				btctrl.set_beatsize(Beatsize::Word);
				btctrl.set_blockact(BlockAction::None);
				btctrl.set_evosel(EventOutputAction::None);
				btctrl.set_valid(true);
				btctrl
			},
			btcnt: len,
			srcaddr: unsafe { src.offset(0) as *const u8 },
			dstaddr: unsafe { dst.offset(len as isize) as *mut u8 },
			.. Default::default()
		}
	}

	/// Copy 1 u16 from src to len u16 values in dst
	pub fn memset_u16_pattern1(src: *const u16, dst: *mut u16, len: u16) -> Self {
		Descriptor {
			btctrl: {
				let mut btctrl = Btctrl(0);
				btctrl.set_stepsize(Stepsize::X1);
				btctrl.set_stepsel(Stepsel::StepDst);
				btctrl.set_dstinc(true);
				btctrl.set_srcinc(false);
				btctrl.set_beatsize(Beatsize::HalfWord);
				btctrl.set_blockact(BlockAction::None);
				btctrl.set_evosel(EventOutputAction::None);
				btctrl.set_valid(true);
				btctrl
			},
			btcnt: len,
			srcaddr: unsafe { src.offset(0) as *const u8 },
			dstaddr: unsafe { dst.offset(len as isize) as *mut u8 },
			.. Default::default()
		}
	}

	/// Copy 1 u8 from src to len u8 values in dst
	pub fn memset_u8_pattern1(src: *const u8, dst: *mut u8, len: u16) -> Self {
		Descriptor {
			btctrl: {
				let mut btctrl = Btctrl(0);
				btctrl.set_stepsize(Stepsize::X1);
				btctrl.set_stepsel(Stepsel::StepDst);
				btctrl.set_dstinc(true);
				btctrl.set_srcinc(false);
				btctrl.set_beatsize(Beatsize::Byte);
				btctrl.set_blockact(BlockAction::None);
				btctrl.set_evosel(EventOutputAction::None);
				btctrl.set_valid(true);
				btctrl
			},
			btcnt: len,
			srcaddr: unsafe { src.offset(0) as *const u8 },
			dstaddr: unsafe { dst.offset(len as isize) as *mut u8 },
			.. Default::default()
		}
	}

	/// Copy len bytes from src to a single byte dst
	pub fn memcpy_u8_peripheral(src: *const u8, dst: *mut u8, len: u16) -> Self {
		Descriptor {
			btctrl: {
				let mut btctrl = Btctrl(0);
				btctrl.set_stepsize(Stepsize::X1);
				btctrl.set_stepsel(Stepsel::StepSrc);
				btctrl.set_dstinc(false);
				btctrl.set_srcinc(true);
				btctrl.set_beatsize(Beatsize::Byte);
				btctrl.set_blockact(BlockAction::None);
				btctrl.set_evosel(EventOutputAction::None);
				btctrl.set_valid(true);
				btctrl
			},
			btcnt: len,
			srcaddr: unsafe { src.offset(len as isize) as *const u8 },
			dstaddr: unsafe { dst.offset(0) as *mut u8 },
			.. Default::default()
		}
	}

	/// Copy len bytes from src to len bytes in dst
	pub fn memcpy_u8(src: *const u8, dst: *mut u8, len: u16) -> Self {
		Descriptor {
			btctrl: {
				let mut btctrl = Btctrl(0);
				btctrl.set_stepsize(Stepsize::X1);
				btctrl.set_stepsel(Stepsel::StepSrc);
				btctrl.set_dstinc(true);
				btctrl.set_srcinc(true);
				btctrl.set_beatsize(Beatsize::Byte);
				btctrl.set_blockact(BlockAction::None);
				btctrl.set_evosel(EventOutputAction::None);
				btctrl.set_valid(true);
				btctrl
			},
			btcnt: len,
			srcaddr: unsafe { src.offset(len as isize) as *const u8 },
			dstaddr: unsafe { dst.offset(len as isize) as *mut u8 },
			.. Default::default()
		}
	}
}
