pub fn u16_to_u8_clamped(value: u16) -> u8 {
	if value > 255u16 {
		255u8
	} else {
		value as u8
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