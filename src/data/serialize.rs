pub trait ByteSerializer {
	fn serialize_to_bytes(&self) -> Vec<u8>;
}

impl ByteSerializer for Vec<i16> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(2 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i16_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(i16, i16)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(4 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i16_to_buffer(&element.0, &mut result);
			ByteSerializer::push_i16_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(i16, i16, i16)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(6 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i16_to_buffer(&element.0, &mut result);
			ByteSerializer::push_i16_to_buffer(&element.1, &mut result);
			ByteSerializer::push_i16_to_buffer(&element.2, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<i32> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(4 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i32_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(i32, i32)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(8 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i32_to_buffer(&element.0, &mut result);
			ByteSerializer::push_i32_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(i32, i32, i32)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(12 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i32_to_buffer(&element.0, &mut result);
			ByteSerializer::push_i32_to_buffer(&element.1, &mut result);
			ByteSerializer::push_i32_to_buffer(&element.2, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<i64> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(8 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i64_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(i64, i64)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(16 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i64_to_buffer(&element.0, &mut result);
			ByteSerializer::push_i64_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(i64, i64, i64)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(24 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_i64_to_buffer(&element.0, &mut result);
			ByteSerializer::push_i64_to_buffer(&element.1, &mut result);
			ByteSerializer::push_i64_to_buffer(&element.2, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<u16> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(2 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u16_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(u16, u16)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(4 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u16_to_buffer(&element.0, &mut result);
			ByteSerializer::push_u16_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(u16, u16, u16)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(6 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u16_to_buffer(&element.0, &mut result);
			ByteSerializer::push_u16_to_buffer(&element.1, &mut result);
			ByteSerializer::push_u16_to_buffer(&element.2, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<u32> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(4 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u32_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(u32, u32)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(8 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u32_to_buffer(&element.0, &mut result);
			ByteSerializer::push_u32_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(u32, u32, u32)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(12 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u32_to_buffer(&element.0, &mut result);
			ByteSerializer::push_u32_to_buffer(&element.1, &mut result);
			ByteSerializer::push_u32_to_buffer(&element.2, &mut result);
		});
		result
	}
}


impl ByteSerializer for Vec<u64> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(8 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u64_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(u64, u64)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(16 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u64_to_buffer(&element.0, &mut result);
			ByteSerializer::push_u64_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(u64, u64, u64)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(24 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_u64_to_buffer(&element.0, &mut result);
			ByteSerializer::push_u64_to_buffer(&element.1, &mut result);
			ByteSerializer::push_u64_to_buffer(&element.2, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<f32> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(4 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_f32_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(f32, f32)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(8 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_f32_to_buffer(&element.0, &mut result);
			ByteSerializer::push_f32_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(f32, f32, f32)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(12 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_f32_to_buffer(&element.0, &mut result);
			ByteSerializer::push_f32_to_buffer(&element.1, &mut result);
			ByteSerializer::push_f32_to_buffer(&element.2, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<f64> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(8 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_f64_to_buffer(&element, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(f64, f64)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(16 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_f64_to_buffer(&element.0, &mut result);
			ByteSerializer::push_f64_to_buffer(&element.1, &mut result);
		});
		result
	}
}

impl ByteSerializer for Vec<(f64, f64, f64)> {
	fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut result = Vec::<u8>::with_capacity(24 * self.len());
		self.iter().for_each(|element| {
			ByteSerializer::push_f64_to_buffer(&element.0, &mut result);
			ByteSerializer::push_f64_to_buffer(&element.1, &mut result);
			ByteSerializer::push_f64_to_buffer(&element.2, &mut result);
		});
		result
	}
}


impl dyn ByteSerializer {
	fn push_u8_to_buffer(value: &u8, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_1byte_to_buffer(&bytes, buffer);
	}

	fn push_u16_to_buffer(value: &u16, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_2bytes_to_buffer(&bytes, buffer);
	}

	fn push_i16_to_buffer(value: &i16, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_2bytes_to_buffer(&bytes, buffer);
	}

	fn push_u32_to_buffer(value: &u32, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_4bytes_to_buffer(&bytes, buffer);
	}

	fn push_i32_to_buffer(value: &i32, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_4bytes_to_buffer(&bytes, buffer);
	}

	fn push_u64_to_buffer(value: &u64, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_8bytes_to_buffer(&bytes, buffer);
	}

	fn push_i64_to_buffer(value: &i64, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_8bytes_to_buffer(&bytes, buffer);
	}

	fn push_f32_to_buffer(value: &f32, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_4bytes_to_buffer(&bytes, buffer);
	}

	fn push_f64_to_buffer(value: &f64, buffer: &mut Vec<u8>) {
		let bytes = value.to_le_bytes();
		Self::push_8bytes_to_buffer(&bytes, buffer);
	}

	fn push_1byte_to_buffer(bytes: &[u8; 1], buffer: &mut Vec<u8>) {
		buffer.push(bytes[0]);
	}

	fn push_2bytes_to_buffer(bytes: &[u8; 2], buffer: &mut Vec<u8>) {
		buffer.push(bytes[0]);
		buffer.push(bytes[1]);
	}

	fn push_4bytes_to_buffer(bytes: &[u8; 4], buffer: &mut Vec<u8>) {
		buffer.push(bytes[0]);
		buffer.push(bytes[1]);
		buffer.push(bytes[2]);
		buffer.push(bytes[3]);
	}

	fn push_8bytes_to_buffer(bytes: &[u8; 8], buffer: &mut Vec<u8>) {
		buffer.push(bytes[0]);
		buffer.push(bytes[1]);
		buffer.push(bytes[2]);
		buffer.push(bytes[3]);
		buffer.push(bytes[4]);
		buffer.push(bytes[5]);
		buffer.push(bytes[6]);
		buffer.push(bytes[7]);
	}
}

