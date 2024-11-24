use bitflags::bitflags;
//use crate::constants::{FRAME_DURATION_MS};

bitflags! {
    /// Gamepad informational flags.
    #[derive(Default)]
    pub struct ControllerFlags: u8 {
        /// This gamepad is connected.
        const CONNECTED = 1 << 0;

        /// Does this controller support vibration?
        const HAS_RUMBLE = 1 << 1;
    }
}

bitflags! {
    /// Flags indicating which buttons are currently pressed.
    /// Same as <https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad>
    /// with the addition of guide and misc buttons in the two unused bits.
    #[derive(Default)]
    pub struct ControllerButtons: u8 {

        const A = 1 << 1;
        const B = 1 << 2;
        const X = 1 << 3;
        const Y = 1 << 4;

        const LEFT_STICK = 1 << 5;
        const RIGHT_STICK = 1 << 6;

        const LEFT_MENU = 1 << 7;
    }
}

/// Structure representing a gamepad to the runtime.
/// Based on <https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad>
/// and <https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_vibration>
/// but prefixed with controller flags and a battery meter, and with smaller rumble types to fit
/// into a convenient size (16 bytes). All fields are written to the runtime,
/// except for the rumble fields, which are read from the runtime.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default)]
pub struct HMDInterface {
    //pub flags: ControllerFlags,
    pub buttons: ControllerButtons,

    pub left_trigger: u8,
    pub right_trigger: u8,

    pub left_grip: u8,
    pub right_grip: u8,

    pub left_stick_x: i16,
    pub left_stick_y: i16,

    pub right_stick_x: i16,
    pub right_stick_y: i16,

    /// Output from the runtime.
    pub left_rumble: u8,
    pub right_rumble: u8,
}


/// there's only one vr hmd so eh.
pub type HMDInterfaceArray = [HMDInterface; 1];

/*
pub enum Button {
    A,
    B,
    X,
    Y,
    LeftStick,
    RightStick,
    LeftMenu,
}

/// There's no convenient way to iterate over an enum,
/// but fortunately this one doesn't change very often.
const CONTROLLER_BUTTONS: [Button; 7] = [
    Button::A,
    Button::B,
    Button::X,
    Button::Y,
    Button::LeftStick,
    Button::RightStick,
    Button::LeftMenu,
];
 */

//pub fn sync_input() -> Result<(), Error> {
    //let game_controller = &mut sdl_gamepad.game_controller;
    //let joystick = &sdl_gamepad.joystick;
    /*
    if !game_controller.attached() {
        // *gamepad = PinputGamepad::default();
        return Ok(());
    }

    // Set rumble effects, if we can.
    if sdl_gamepad.has_rumble {
        game_controller.set_rumble(
            ((gamepad.lo_freq_rumble as f64) / (u8::MAX as f64) * (u16::MAX as f64)) as u16,
            ((gamepad.hi_freq_rumble as f64) / (u8::MAX as f64) * (u16::MAX as f64)) as u16,
            // Setting one frame of rumble leads to choppiness as the effect may expire early.
            2 * FRAME_DURATION_MS as u32,
        )?;
    }

    // Read gamepad capabilities and power level.
    gamepad.flags = ControllerFlags::default();
    gamepad.flags.insert(ControllerFlags::CONNECTED);
    let mapping = game_controller.mapping();
    if mapping.contains("guide:") {
        gamepad.flags.insert(ControllerFlags::HAS_GUIDE_BUTTON);
    }
    if mapping.contains("misc1:") || mapping.contains("touchpad:") {
        gamepad.flags.insert(ControllerFlags::HAS_MISC_BUTTON);
    }
    if sdl_gamepad.has_rumble {
        gamepad.flags.insert(ControllerFlags::HAS_RUMBLE);
    }
*/

    // Read gamepad buttons.
    // Temporary variable used to avoid an unaligned access.
    //let mut buttons = ControllerButtons::default();

    //for button in CONTROLLER_BUTTONS {
        //if game_controller.button(button) {
        //    if let Ok(button) = ControllerButtons::try_from(button) {
        //        buttons.insert(button);
        //    }
        //}
    //}  
    /*
    gamepad.buttons = buttons;

    // Read gamepad axes (including triggers).
    // Note that SDL Y axes are upside-down compared to XInput:
    // <https://github.com/libsdl-org/SDL/blob/9130f7c/src/joystick/windows/SDL_xinputjoystick.c#L462-L465>
    gamepad.left_stick_x = game_controller.axis(Axis::LeftX);
    gamepad.left_stick_y = !game_controller.axis(Axis::LeftY);
    gamepad.right_stick_x = game_controller.axis(Axis::RightX);
    gamepad.right_stick_y = !game_controller.axis(Axis::RightY);
    gamepad.left_trigger = (game_controller.axis(Axis::TriggerLeft) / 0x81) as u8;
    gamepad.right_trigger = (game_controller.axis(Axis::TriggerRight) / 0x81) as u8;
*/

    //Ok(())
//}
