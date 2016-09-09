/// Errno
#[no_mangle]
pub static mut __errno: isize = 0;

/// Shim for ralloc
/// Cooperatively gives up a timeslice to the OS scheduler.
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn sched_yield() -> isize {
    match ::syscall::sched_yield() {
        Ok(_) => 0,
        Err(_) => -1
    }
}

/// Shim for ralloc
/// Increment data segment of this process by some, _n_, return a pointer to the new data segment
/// start.
///
/// This uses the system call BRK as backend.
///
/// This is unsafe for multiple reasons. Most importantly, it can create an inconsistent state,
/// because it is not atomic. Thus, it can be used to create Undefined Behavior.
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn sbrk(n: isize) -> *mut u8 {
    let orig_seg_end = match unsafe { ::syscall::brk(0) } {
        Ok(end) => end,
        Err(_) => return !0 as *mut u8
    };

    if n == 0 {
        return orig_seg_end as *mut u8;
    }

    let expected_end = match orig_seg_end.checked_add(n as usize) {
        Some(end) => end,
        None => return !0 as *mut u8
    };

    let new_seg_end = match unsafe { ::syscall::brk(expected_end) } {
        Ok(end) => end,
        Err(_) => return !0 as *mut u8
    };

    if new_seg_end != expected_end {
        // Reset the break.
        let _ = unsafe { ::syscall::brk(orig_seg_end) };

        !0 as *mut u8
    } else {
        orig_seg_end as *mut u8
    }
}

/// Memcpy
///
/// Copy N bytes of memory from one location to another.
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }

    dest
}

/// Memmove
///
/// Copy N bytes of memory from src to dest. The memory areas may overlap.
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }

    dest
}

/// Memset
///
/// Fill a block of memory with a specified value.
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }

    s
}

/// Memcmp
///
/// Compare two blocks of memory.
#[no_mangle]
#[linkage = "weak"]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;

    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }

    0
}
