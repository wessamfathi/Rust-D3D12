mod dx12;

use windows::{core::Result, Win32::System::Threading::*};

static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

fn main() -> Result<()> {
    println!("Hello, Win-rs!");

    unsafe {
        let work = CreateThreadpoolWork(Some(callback), None, None)?;

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, false);

        CloseThreadpoolWork(work);
    }

    let counter = COUNTER.read().unwrap();
    print!("counter: {}", *counter);

    println!("-----------------------------");

    dx12::create();

    Ok(())
}

extern "system" fn callback(_: PTP_CALLBACK_INSTANCE, _: *mut std::ffi::c_void, _: PTP_WORK) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}