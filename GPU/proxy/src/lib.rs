use std::os::raw::c_void;

use cudarc::runtime::sys as runtime;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaMalloc(dev_ptr: *mut *mut c_void, size: usize) -> runtime::cudaError {
    println!(
        "[Hooked] Try to malloc device memory at {:p}, len=0x{:04x}",
        dev_ptr, size
    );

    unsafe { runtime::cudaMalloc(dev_ptr, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaFree(dev_ptr: *mut c_void) -> runtime::cudaError {
    println!("[Hooked] Try to free device memory at {:p}", dev_ptr);

    unsafe { runtime::cudaFree(dev_ptr) }
}
