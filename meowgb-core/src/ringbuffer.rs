pub struct RingBuffer<T: std::fmt::Debug + Copy + Default, const SIZE: usize> {
	buffer: [T; SIZE],
	size: usize,
	write_ptr: usize,
	read_ptr: usize,
}

impl<T: std::fmt::Debug + Copy + Default, const SIZE: usize> RingBuffer<T, SIZE> {
	pub fn new() -> Self {
		RingBuffer { buffer: [T::default(); SIZE], size: 0, write_ptr: 0, read_ptr: 0 }
	}

	pub fn push(&mut self, value: T) {
		self.buffer[self.write_ptr] = value;
		if self.size < SIZE {
			self.size += 1;
		} else {
			self.read_ptr += 1;
			self.read_ptr %= SIZE;
		}
		self.write_ptr += 1;
		self.write_ptr %= SIZE;
	}

	pub fn to_vec(&self) -> Vec<T> {
		let mut out = Vec::new();
		let mut offset = self.read_ptr;

		for _ in 0..self.size {
			out.push(self.buffer[offset]);

			offset += 1;
			offset %= SIZE;
		}

		out
	}
}

#[test]
fn test_ringbuffer() {
	let mut ringbuffer: RingBuffer<u8, 16> = RingBuffer::new();

	for x in 0..16 {
		ringbuffer.push(x);
	}

	assert_eq!(
		ringbuffer.to_vec().as_slice(),
		&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
	);
	ringbuffer.push(16);
	assert_eq!(
		ringbuffer.to_vec().as_slice(),
		&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
	);
	ringbuffer.push(17);
	assert_eq!(
		ringbuffer.to_vec().as_slice(),
		&[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
	);

	for x in 18..32 {
		ringbuffer.push(x);
	}
	assert_eq!(
		ringbuffer.to_vec().as_slice(),
		&[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
	);
}
