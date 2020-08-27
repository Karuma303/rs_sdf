pub fn i32_to_u8_clamped(value: i32) -> u8 {
	u32_to_u8_clamped(value.saturating_abs() as u32 + i8::MAX as u32)
}

pub fn i16_to_u8_clamped(value: i16) -> u8 {
	u16_to_u8_clamped(value.saturating_abs() as u16 + i8::MAX as u16)
}

pub fn i32_to_u16_clamped(value: i32) -> u16 {
	u32_to_u16_clamped(value.saturating_abs() as u32 + i16::MAX as u32)
}

pub fn u16_to_u8_clamped(value: u16) -> u8 {
	if value > 255u16 {
		255u8
	} else {
		value as u8
	}
}

pub fn u32_to_u8_clamped(value: u32) -> u8 {
	if value > 255u32 {
		255u8
	} else {
		value as u8
	}
}
pub fn u32_to_u16_clamped(value: u32) -> u16 {
	if value > u16::MAX as u32 {
		u16::MAX
	} else {
		value as u16
	}
}

pub fn f32_to_u8_clamped(value: f32) -> u8 {
	if value > 255f32 {
		255u8
	} else {
		value as u8
	}
}

pub fn f32_to_u16_clamped(value: f32) -> u16 {
	if value > 65535.0f32 {
		0xffff
	} else {
		// Maybe we should round here? -> value.round() as u16
		value as u16
	}
}