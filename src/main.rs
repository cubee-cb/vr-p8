
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use timer::Timer;
use chrono::Duration;
use std::sync::mpsc::channel;
use std::env;

mod error;
mod constants;
mod runtime_connection;

use crate::constants::{MAGIC, VR_VERT_STRIDE, VR_VERT_BUFFER_SIZE, VR_MAX_VERTS, SCAN_INTERVAL_MS};
use crate::runtime_connection::RuntimeConnection;
use crate::error::Error;

fn main() -> Result<(), Error> {
    println!("magic: {}\nvert stride: {} (space: {})\nmax verts: {}\ntri range: {}-{} (separate - fan)", MAGIC, VR_VERT_STRIDE, VR_VERT_BUFFER_SIZE, VR_MAX_VERTS, VR_MAX_VERTS / 4, VR_MAX_VERTS - 2);

    check_prerequisites()?;

    let keep_going = Arc::new(AtomicBool::new(true));
    let keep_going_ctrlc = keep_going.clone();
    ctrlc::set_handler(move || keep_going_ctrlc.store(false, Ordering::Relaxed))?;





    let runtime_connection = scan_for_runtime_connection(keep_going.clone())?;
    println!(
        "connected: {}, PID {}",
        runtime_connection.flavor, runtime_connection.pid
    );

    Ok(())
}


/// Look for a runtime with the magic number until we either find it or are killed.
fn scan_for_runtime_connection(keep_going: Arc<AtomicBool>) -> Result<RuntimeConnection, Error> {
    let (timer_tx, timer_rx) = channel();
    let timer = Timer::new();
    let _timer_guard = Some(timer.schedule_repeating(
        Duration::milliseconds(SCAN_INTERVAL_MS),
        move || {
            timer_tx
                .send(())
                .expect("we should always be able to send timer ticks")
        },
    ));
    while keep_going.load(Ordering::Relaxed) {
        timer_rx.recv()?;
        match RuntimeConnection::try_new() {
            Ok(runtime_connection) => return Ok(runtime_connection),
            Err(err) => println!("Failed to connect to a runtime: {:#?}", err),
        }
    }
    Err(Error::KilledByCtrlC)
}




#[cfg(target_os = "linux")]
fn check_prerequisites() -> Result<(), Error> {
    if caps::has_cap(
        None,
        caps::CapSet::Effective,
        caps::Capability::CAP_SYS_PTRACE,
    )? {
        Ok(())
    } else {
        eprintln!("Missing the CAP_SYS_PTRACE capability!");
        eprintln!(
            "Please run this command and restart: sudo setcap cap_sys_ptrace+ep {}",
            env::current_exe()?.to_string_lossy()
        );
        Err(Error::MissingPrerequisites)
    }
}

#[cfg(target_os = "macos")]
fn check_prerequisites() -> Result<(), Error> {
    // TODO: need to be root or in an entitled binary to do this on macOS
    // https://dev.to/jasonelwood/setup-gdb-on-macos-in-2020-489k
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn check_prerequisites() -> Result<(), Error> {
    Ok(())
}
