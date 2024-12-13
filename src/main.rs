
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

use crate::constants::{MAGIC, FRAME_DURATION_MS, VR_TRANS_STRIDE, VR_TRANSBUFFER_SIZE, VR_MAX_TRANSFORMS, SCAN_INTERVAL_MS};
use crate::runtime_connection::RuntimeConnection;
use crate::error::Error;
use crate::input::HMDInterfaceArray;
use crate::renderer::render_buffer;

fn main() -> Result<(), Error> {
    println!("magic: {}\nvert stride: {} (space: {})\nmax verts: {}\ntri range: {}-{} (separate - fan)", MAGIC, VR_TRANS_STRIDE, VR_TRANSBUFFER_SIZE, VR_MAX_TRANSFORMS, VR_MAX_TRANSFORMS / 4, VR_MAX_TRANSFORMS - 2);

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
                // print transforms
                let mut i = 0;
                for t in buffer.transforms {
                    // just print the first few for now
                    println!("coord: {},{},{} (uv: {},{})", t.x, t.y, t.z, t.u, t.v);
                    
                    i += 1;
                    if i > 5 {
                        break;
                    }
                }
                // */

                // render display from transform buffer
                render_buffer(buffer);


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




        // positions are cm
        // angles should be (-3600, 3600) for now
        // i presume openxr reports radians? all these angle formats are annoying.
        // maybe add more precision later, but this is just for if the
        // pico-8 game needs to know the device orientation.
        interfaces[0].hmd_x = 0;
        interfaces[0].hmd_y = 150;
        interfaces[0].hmd_z = 0;
        interfaces[0].hmd_yaw = 0;
        interfaces[0].hmd_pitch = 0;
        interfaces[0].hmd_roll = 0;

        interfaces[0].left_x = -10;
        interfaces[0].left_y = 80;
        interfaces[0].left_z = -5;
        interfaces[0].left_yaw = -360;
        interfaces[0].left_pitch = 360;
        interfaces[0].left_roll = 0;

        interfaces[0].right_x = 10;
        interfaces[0].right_y = 80;
        interfaces[0].right_z = -5;
        interfaces[0].right_yaw = 360;
        interfaces[0].right_pitch = 360;
        interfaces[0].right_roll = 0;


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
