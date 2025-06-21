use libc::{MAP_ANON, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, mprotect};
use std::mem;

fn main() {
    // This is ARM64 assembly for a function that takes an i32 and returns it.
    // fn(num: i32) -> i32 { return num; }
    //
    // You can get this from a compiler explorer.
    // sub sp, sp, #16
    // str w0, [sp, #12]
    // ldr w0, [sp, #12]
    // add sp, sp, #16
    // ret
    let code = [
        0xD10043FFu32,
        0xB9000FE0u32,
        0xB9400FE0u32,
        0x910043FFu32,
        0xD65F03C0u32,
    ];

    let mem_size = code.len() * mem::size_of::<u32>();

    unsafe {
        // 1. Allocate memory.
        // We ask for memory that is readable and writable.
        // On Apple Silicon, we need MAP_JIT to be able to make it executable later.
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let flags = MAP_ANON | MAP_PRIVATE | libc::MAP_JIT;
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        let flags = MAP_ANON | MAP_PRIVATE;

        let addr = mmap(
            std::ptr::null_mut(),
            mem_size,
            PROT_READ | PROT_WRITE,
            flags,
            -1,
            0,
        );

        if addr == libc::MAP_FAILED {
            panic!("mmap failed");
        }

        // 2. Write assembly to the memory.
        std::ptr::copy_nonoverlapping(code.as_ptr(), addr as *mut u32, code.len());

        // 3. Change memory permissions to read-only and executable.
        // This is important for security (W^X).
        let result = mprotect(addr, mem_size, PROT_READ | PROT_EXEC);
        if result != 0 {
            panic!("mprotect failed");
        }

        // 4. Cast the memory address to a function pointer.
        let func: extern "C" fn(i32) -> i32 = mem::transmute(addr);

        // 5. Call the function!
        let input = 42;
        let output = func(input);

        println!("Called JIT function with {} and got {}", input, output);

        // In a real application, you would also need to unmap the memory with `munmap`.
        // For this simple example, we'll let the OS clean it up on process exit.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
