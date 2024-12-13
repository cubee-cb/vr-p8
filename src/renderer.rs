//use openxr;
use super::VR_MAX_TRANSFORMS;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub u: u8,
    pub v: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct TransBuffer {
    //pub state: State,
    pub transforms: [Transform; VR_MAX_TRANSFORMS as usize]
}

/*/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Free,
    _OccupiedWriting,
    OccupiedRendering
}
// */

enum TransType {
    Vertex,
    Object,
    _Unused,
    None
}

pub fn draw_tri(transforms:Vec<Transform>) {

    for transform in transforms {
        let _u               = transform.u & 0b11111100 >> 2;
        let _v               = transform.v & 0b11111100 >> 2;

        let _col             = transform.u & 0b11110000 >> 4;

        let _switch_colour_u = transform.v & 0b00000001;
        let _switch_blend    = transform.v & 0b00000010 >> 1;

        //TODO: learn openxr and vulkan so i can draw triangles
        let _x = transform.x;
        let _y = transform.y;
        let _z = transform.z;
    }

}

pub fn render_buffer(buffer:TransBuffer) -> bool {
    /*/ skip if the buffer is not free
    if buffer.state != State::Free {
        return false;
    }
    //TODO: this currently is not written to pico-8 memory
    buffer.state = State::OccupiedRendering;
    // */

    let mut tri_buff: Vec<Transform> = vec![];

    for transform in buffer.transforms {
        let mode    = transform.u & 0b00000011;
        //let _var     = transform.u & 0b00001100 >> 2;
        //let _colour  = transform.u & 0b11110000 >> 4;

        let _trans_type = match mode {
            0 => {
                // skip transforms if we don't have enough to draw a triangle
                //TODO: this may cause issues since it won't be cleared when processing the other transform types
                tri_buff.push(transform);
                if tri_buff.len() < 3 {
                    continue;
                }

                // clone the tri buff to make rust-analyzer shush
                draw_tri(tri_buff.clone());

                // remove the oldest transform from the tri buffer
                // idk how to pop_front so here you go
                tri_buff.drain(0..0);

                TransType::Vertex
            },

            1 => {
                TransType::Object
            },

            2 => {TransType::_Unused},

            _ => {
                tri_buff.clear();
                TransType::None
            },

        };


        //println!("got transform: mode {}", mode);

        
    }

    // buffer is done being read
    //buffer.state = State::Free;

    // success
    return true;
}
