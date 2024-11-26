
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use timer::Timer;
use chrono::Duration;
use std::sync::mpsc::channel;
use std::env;
use process_memory::Memory;

mod error;
mod constants;
mod runtime_connection;
mod input;
mod renderer;

use crate::constants::{MAGIC, FRAME_DURATION_MS, VR_VERT_STRIDE, VR_VERT_BUFFER_SIZE, VR_MAX_VERTS, SCAN_INTERVAL_MS};
use crate::runtime_connection::RuntimeConnection;
use crate::error::Error;
use crate::input::HMDInterfaceArray;

fn main() -> Result<(), Error> {
    println!("magic: {}\nvert stride: {} (space: {})\nmax verts: {}\ntri range: {}-{} (separate - fan)", MAGIC, VR_VERT_STRIDE, VR_VERT_BUFFER_SIZE, VR_MAX_VERTS, VR_MAX_VERTS / 4, VR_MAX_VERTS - 2);

    check_prerequisites()?;

    let keep_going = Arc::new(AtomicBool::new(true));
    let keep_going_ctrlc = keep_going.clone();
    ctrlc::set_handler(move || keep_going_ctrlc.store(false, Ordering::Relaxed))?;





    //let runtime_connection = scan_for_runtime_connection(keep_going.clone())?;
    //println!(
    //    "connected: {}, PID {}",
    //    runtime_connection.flavor, runtime_connection.pid
    //);

    loop {
        let runtime_connection = scan_for_runtime_connection(keep_going.clone())?;
        println!(
            "connected: {}, PID {}",
            runtime_connection.flavor, runtime_connection.pid
        );
        run_gamepad_loop(
            &keep_going,
            &runtime_connection,
        )?;
        
    }
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


/// Sync SDL interfaces with the runtime until the runtime quits or we are killed.
fn run_gamepad_loop(
    keep_going: &Arc<AtomicBool>,
    runtime_connection: &RuntimeConnection,
) -> Result<(), Error> {

    let (timer_tx, timer_rx) = channel();
    let timer = Timer::new();
    let _timer_guard = Some(timer.schedule_repeating(
        Duration::milliseconds(FRAME_DURATION_MS),
        move || {
            timer_tx
                .send(())
                .expect("we should always be able to send timer ticks")
        },
    ));

    let mut interfaces: HMDInterfaceArray;

    while keep_going.load(Ordering::Relaxed) {
        timer_rx.recv()?;

        // Updates the state of all game controllers.
        //game_controller_subsystem.update();

        let magic = match unsafe { runtime_connection.gpio_as_uuid.read() } {
            Ok(magic) => magic,
            Err(err) => {
                // Failure here probably indicates that the runtime quit.
                println!(
                    "Failed to read from {}: {:#}",
                    runtime_connection.flavor, err
                );
                return Ok(());
            }
        };
        if magic == MAGIC {
            interfaces = HMDInterfaceArray::default();
        } else {
            interfaces = match unsafe { runtime_connection.gpio_as_interface.read() } {
                Ok(interfaces) => interfaces,
                Err(err) => {
                    // Failure here probably indicates that the runtime quit.
                    println!(
                        "Failed to read from {}: {:#}",
                        runtime_connection.flavor, err
                    );
                    return Ok(());
                }
            }
        }

        //for index_usize in 0..0 {
            //let index = index_usize as u32;

            //let mut game_controller = game_controller_subsystem.open(index)?;

            //let mut gamepad = &mut interfaces[index_usize];
            //if let Some(sdl_gamepad) = &mut sdl_interfaces[index_usize] {
            //    sync_gamepad(sdl_gamepad, &mut gamepad)?;
            //}
        //}

        // read vertex buffer from upper memory
        match unsafe {runtime_connection.upper_memory.read() } {
            Ok(buffer) => {
                for vertex in buffer.verts {
                    // just print the first one for now
                    println!("{}", vertex.coords);
                    break;
                }
            },
            Err(err) => {
                // Failure here probably indicates that the runtime quit.
                println!(
                    "Failed to read from {}: {:#}",
                    runtime_connection.flavor, err
                );
                return Ok(());
            }
        }

        // write hmd status to gpio
        match runtime_connection.gpio_as_interface.write(&interfaces) {
            Ok(_) => (),
            Err(err) => {
                // Failure here probably indicates that the runtime quit.
                println!(
                    "Failed to write from {}: {:#}",
                    runtime_connection.flavor, err
                );
                return Ok(());
            }
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

#[cfg(not(target_os = "linux"))]
fn check_prerequisites() -> Result<(), Error> {
    Ok(())
}
