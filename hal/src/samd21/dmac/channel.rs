use crate::{
	target_device,
	events,
};

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Channel {
	Channel0 = 0,
	Channel1 = 1,
	Channel2 = 2,
	Channel3 = 3,
	Channel4 = 4,
	Channel5 = 5,
	Channel6 = 6,
	Channel7 = 7,
	Channel8 = 8,
	Channel9 = 9,
	Channel10 = 10,
	Channel11 = 11,
}

impl Channel {
	pub fn event_generator(&self) -> events::Generator {
		match self {
			&Channel::Channel0 => events::Generator::DMAC_CH0,
			&Channel::Channel1 => events::Generator::DMAC_CH1,
			&Channel::Channel2 => events::Generator::DMAC_CH2,
			&Channel::Channel3 => events::Generator::DMAC_CH3,
			_ => panic!("channel {:?} does not support events", self),
		}
	}

	pub fn event_user(&self) -> events::User {
		match self {
			&Channel::Channel0 => events::User::DMAC_CH0,
			&Channel::Channel1 => events::User::DMAC_CH1,
			&Channel::Channel2 => events::User::DMAC_CH2,
			&Channel::Channel3 => events::User::DMAC_CH3,
			_ => panic!("channel {:?} does not support events", self),
		}
	}
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum TriggerSource {
	/// Software triggered only
	Software = 0x0,

	Sercom0Rx = 0x1,
	Sercom0Tx = 0x2,

	Sercom1Rx = 0x3,
	Sercom1Tx = 0x4,

	Sercom2Rx = 0x5,
	Sercom2Tx = 0x6,

	Sercom3Rx = 0x7,
	Sercom3Tx = 0x8,

	Sercom4Rx = 0x9,
	Sercom4Tx = 0xA,

	Sercom5Rx = 0xB,
	Sercom5Tx = 0xC,

	/// TimerCounter 3 overflow
	Tc3Ovf = 0x18,

	Tc4Ovf = 0x1B,

	/// ADC Result Ready
	AdcResultReady = 0x27,
}

pub type Action = target_device::dmac::chctrlb::TRIGACT_A;

pub struct Events {
	pub output: bool,
	pub input: Option<EventInputAction>,
}

pub type EventInputAction = target_device::dmac::chctrlb::EVACT_A;

pub struct Interrupts {
	pub suspend: bool,
	pub transfer_complete: bool,
	pub transfer_error: bool,
}
