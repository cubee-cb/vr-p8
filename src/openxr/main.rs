
use openxr as xr;
use renderer::Transform;
use subprocess;

mod error;
mod constants;
mod renderer;

use crate::error::Error;
use crate::renderer::{render_buffer, TransBuffer};

fn main() -> Result<(), Error> {
    // init xr
    // https://github.com/heckmarr/rustsphere/blob/master/openxr/examples/

    //
    let entry = unsafe {
        xr::Entry::load()
            .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature")
    };

    let _extensions = entry.enumerate_extensions().unwrap();
    //println!("supported extensions: {:#?}", extensions);

    let _layers = entry.enumerate_layers().unwrap();
    //println!("supported layers: {:?}", layers);

    let instance = entry
        .create_instance(
            &xr::ApplicationInfo {
                application_name: "vr.p8",
                ..Default::default()
            },
            &xr::ExtensionSet::default(),
            &[],
        )
        .unwrap();
    
    let instance_props = instance.properties().unwrap();
    println!(
        "loaded instance: {} v{}",
        instance_props.runtime_name, instance_props.runtime_version
    );
    // */


    let system = instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();
    let system_props = instance.system_properties(system).unwrap();
    println!(
        "selected system {}: {}",
        system_props.system_id.into_raw(),
        if system_props.system_name.is_empty() {
            "<unnamed>"
        } else {
            &system_props.system_name
        }
    );

    let view_config_views = instance
        .enumerate_view_configuration_views(system, xr::ViewConfigurationType::PRIMARY_STEREO)
        .unwrap();
    println!("view configuration views: {:#?}", view_config_views);


    //TODO: create memory process
    //TODO: pass input state
    //TODO: get buffer
    //TODO: render buffer

    let buffer = TransBuffer { //TODO: temp array
        transforms: [Transform {
            x: 0,
            y: 0,
            z: 0,
            u: 1,
            v: 1,
        }; 2048]
    };
    render_buffer(buffer);


    Err(Error::KilledByCtrlC)
}
