use core::{mem, slice, str};
use env::{args_init, args_destroy};
use syscall::exit;
use vec::Vec;

pub use panicking::{begin_panic, begin_panic_fmt};

#[no_mangle]
#[naked]
#[cfg(target_arch = "x86")]
pub unsafe fn _start() {
    asm!("push esp
        call _start_stack
        pop esp"
        :
        :
        : "memory"
        : "intel", "volatile");
    let _ = exit(0);
}

#[no_mangle]
#[naked]
#[cfg(target_arch = "x86_64")]
pub unsafe fn _start() {
    asm!("mov rdi, rsp
        call _start_stack"
        :
        :
        : "memory"
        : "intel", "volatile");
    let _ = exit(0);
}

#[no_mangle]
pub unsafe extern "C" fn _start_stack(stack: *const usize){
    extern "C" {
        fn main(argc: usize, argv: *const *const u8) -> usize;
    }

    let argc = *stack;
    let argv = stack.offset(1) as *const *const u8;
    let _ = exit(main(argc, argv));
}

#[lang = "start"]
fn lang_start(main: *const u8, argc: usize, argv: *const *const u8) -> usize {
    unsafe {
        let mut args: Vec<&'static str> = Vec::new();
        for i in 0..argc as isize {
            let len = *(argv.offset(i * 2)) as usize;
            let ptr = *(argv.offset(i * 2 + 1));
            args.push(str::from_utf8_unchecked(slice::from_raw_parts(ptr, len)));
        }

        args_init(args);

        mem::transmute::<_, fn()>(main)();

        args_destroy();
    }

    0
}
