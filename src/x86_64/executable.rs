unsafe fn syscall(
	id: usize,
	a0: usize,
	a1: usize,
	a2: usize,
	a3: usize,
	a4: usize,
	a5: usize,
) -> usize {
	let mut ret;

	std::arch::asm! {
		"syscall",
		inlateout("rax") id => ret,
		inout("rdi") a0 => _,
		inout("rsi") a1 => _,
		inout("rdx") a2 => _,
		inout("r10") a3 => _,
		inout("r8") a4 => _,
		inout("r9") a5 => _,
		out("rcx") _,
		out("r11") _,
	}

	ret
}

fn mmap(size: usize) -> *mut u8 {
	unsafe { syscall(9, 0, size, 7, 0x22, 0, 0) as _ }
}

fn munmap(ptr: *mut u8, size: usize) {
	unsafe {
		syscall(11, ptr as _, size, 0, 0, 0, 0);
	}
}

pub struct Executable {
	ptr: *mut u8,
	size: usize,
	entry_point_offset: usize,
}

impl Executable {
	pub fn new(bytes: Vec<u8>, entry_point_offset: usize) -> Self {
		let ptr = mmap(bytes.len());
		unsafe {
			std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
		}

		Self {
			ptr,
			size: bytes.len(),
			entry_point_offset,
		}
	}

	pub fn call(&self) -> usize {
		unsafe {
			let func: extern "C" fn() -> usize =
				std::mem::transmute(self.ptr.add(self.entry_point_offset));
			func()
		}
	}
}
