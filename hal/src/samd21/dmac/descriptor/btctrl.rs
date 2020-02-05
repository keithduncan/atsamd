use bitfield::{
	bitfield_bitrange,
	bitfield_fields,
	BitRange,
};

#[derive(PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub struct Btctrl(pub u16);

bitfield_bitrange! {
    struct Btctrl(u16)
}

impl Btctrl {
    bitfield_fields! {
        u8;
        pub from into Stepsize, stepsize, set_stepsize : 15, 13;
        //pub from into Stepsel, stepsel,  set_stepsel  : 12;
        pub dstinc,   set_dstinc   : 11;
        pub srcinc,   set_srcinc   : 10;
        pub from into Beatsize, beatsize, set_beatsize : 9, 8;
        pub from into BlockAction, blockact, set_blockact : 4, 3;
        pub from into EventOutputAction, evosel,   set_evosel   : 2, 1;
        pub valid,    set_valid    : 0;
    }

    #[allow(unused)]
    pub fn stepsel(&self) -> Stepsel {
		let val: u8 = self.bit_range(12, 12);
		Stepsel::from(val)
    }

    #[allow(dead_code)]
    pub fn set_stepsel(&mut self, stepsel: Stepsel) {
        self.set_bit_range(12, 12, stepsel as u8)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Stepsize {
	X1 = 0,
	X2 = 1,
	X4 = 2,
	X8 = 3,
	X16 = 4,
	X32 = 5,
	X64 = 6,
	X128 = 7,
}

impl Into<u8> for Stepsize {
	fn into(self) -> u8 {
		self as u8
	}
}

impl From<u8> for Stepsize {
	fn from(val: u8) -> Self {
		match val {
			0 => Stepsize::X1,
			1 => Stepsize::X2,
			2 => Stepsize::X4,
			3 => Stepsize::X8,
			4 => Stepsize::X16,
			5 => Stepsize::X32,
			6 => Stepsize::X64,
			7 => Stepsize::X128,
			_ => unimplemented!(),
		}
	}
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Stepsel {
	StepDst = 0,
	StepSrc = 1,
}

impl From<u8> for Stepsel {
	fn from(val: u8) -> Self {
		if val == 0 {
			Stepsel::StepDst
		} else {
			Stepsel::StepSrc
		}
	}
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Beatsize {
	Byte = 0, // 8 bit
	HalfWord = 1, // 16 bit
	Word = 2, // 32 bit
}

impl Into<u8> for Beatsize {
	fn into(self) -> u8 {
		self as u8
	}
}

impl From<u8> for Beatsize {
	fn from(val: u8) -> Self {
		match val {
			0 => Beatsize::Byte,
			1 => Beatsize::HalfWord,
			2 => Beatsize::Word,
			_ => unimplemented!(),
		}
	}
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BlockAction {
	None = 0,
	Interrupt = 1,
	ChannelSuspend = 2,
	ChannelSuspendInterrupt = 3,
}

impl Into<u8> for BlockAction {
	fn into(self) -> u8 {
		self as u8
	}
}

impl From<u8> for BlockAction {
	fn from(val: u8) -> Self {
		match val {
			0 => BlockAction::None,
			1 => BlockAction::Interrupt,
			2 => BlockAction::ChannelSuspend,
			3 => BlockAction::ChannelSuspendInterrupt,
			_ => unimplemented!(),
		}
	}
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum EventOutputAction {
	None = 0,
	Block = 1,
	Beat = 3,
}

impl Into<u8> for EventOutputAction {
	fn into(self) -> u8 {
		self as u8
	}
}

impl From<u8> for EventOutputAction {
	fn from(val: u8) -> Self {
		match val {
			0 => EventOutputAction::None,
			1 => EventOutputAction::Block,
			3 => EventOutputAction::Beat,
			_ => unimplemented!(),
		}
	}
}
