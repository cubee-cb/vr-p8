use uuid::Uuid;

/// Magic byte sequence used to identify vr-enabled cartridges.
//pub const MAGIC: Uuid = Uuid::from_u128(0x0220c74677ab446ebedc7fd6d277984d);
pub const MAGIC: Uuid = Uuid::from_u128(0x6F6D6756526F6E5049434F3821576F77);
/*pico:
0x5667.6D6F
0x506E.6F52
0x384F.4349
0x776F.5721
*/

/// 60 Hz.
pub static FRAME_DURATION_MS: i64 = 16;

/// 1 Hz between attempts to connect to the runtime.
pub static SCAN_INTERVAL_MS: i64 = 1000;

///
pub const VR_VERT_STRIDE: i64 = 8;

pub const VR_VERT_BUFFER_SIZE: i64 = 16384;
pub const VR_MAX_VERTS: i64 = VR_VERT_BUFFER_SIZE / VR_VERT_STRIDE;
//pub static VR_MAX_VERTS_BYTES: i64 = VR_VERT_BUFFER_SIZE_ADDR * 8;



//pub const P8_SPRITES: usize = 0x0000;
//pub const P8_SHARED: usize = 0x1000;
//pub const P8_MAP: usize = 0x2000;
//pub const P8_FLAGS: usize = 0x3000;
//pub const P8_MUSIC: usize = 0x3100;
//pub const P8_SFX: usize = 0x3200;
//pub const P8_GENERAL: usize = 0x4300;
//pub const P8_DRAWSTATE: usize = 0x5f00;
//pub const P8_HARDSTATE: usize = 0x5f40;
pub const P8_GPIO: usize = 0x5f80;
//pub const P8_DISPLAY: usize = 0x6000;
pub const P8_UPPER: usize = 0x8000;
