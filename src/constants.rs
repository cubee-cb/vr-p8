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
//pub static FRAME_DURATION_MS: i64 = 16;

/// 1 Hz between attempts to connect to the runtime.
pub static SCAN_INTERVAL_MS: i64 = 1000;

///
pub static VR_VERT_STRIDE: i64 = 8;
/*
1 x
2 x
3 y
4 y
5 z
6 z
7 u + col switch
    0 tex or col switch
    1 
    2 
    3 u / blend switch
    4 u / col
    5 u / col
    6 u / col
    7 u / col
8 v
    0 
    1 
    2 
    3 v
    4 v
    5 v
    6 v
    7 v


Tris are drawn in fans: after drawing one tri, its last two verts are reused for the next tri
If a vert has everything zeroed, this triangle fan is finished

UVs have a precision of 0-31 (half-tile steps)
    - they cannot reach the right/lower-most edge
*/

pub static VR_VERT_BUFFER_SIZE: i64 = 16384;
pub static VR_MAX_VERTS: i64 = VR_VERT_BUFFER_SIZE / VR_VERT_STRIDE;
//pub static VR_MAX_VERTS_BYTES: i64 = VR_VERT_BUFFER_SIZE_ADDR * 8;
