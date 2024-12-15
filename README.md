# vr.p8
rust newbie tries to build a system to connect the [PICO-8 fantasy console](https://www.lexaloffle.com/pico-8.php) to OpenXR.

## helper program:
- find the pico-8 process, search its memory for a magic number like [pinput](https://github.com/VyrCossont/Pinput) does.
    - use this offset to read the rest of pico-8 memory since the layout should be the same from there.
    - e.g. the sprite memory would be below the magic number, while the display would be above.
- read the palette info, construct spritesheet as a texture. (detect updates and redraw?)
    - pico-8 has a fixed palette of 16 colours from a selection of 32.
- vr.p8 writes hmd/controller pose and input states to pico-8 memory.
    - pico-8's number precision doesn't matter if we're handling camera transforms in the vr.p8 application, unless we want to render something attached to the player (like hands or a ui). it should be precise enough for those purposes anyway.
    - performance of the pico-8 application also holds no bearing to the performance of vr rendering, so hmd view transforms will be smooth even if pico-8 is struggling.
- pico-8 builds a vertex buffer "scene" in memory that we read into vr.p8.
    - pico-8 handles the main game loop, mesh and texture mapping, object transforms, constructing the scene, etc.
    - all camera transformations, culling, rasterisation, depth sorting is done on the vr.p8 side at hmd resolution.
- vr.p8 renders to the hmd using openxr/vulkan and the pico-8 display palette.
    - this program is only for interfacing with the hmd, it shouldn't do anything more than pico-8 can do such as extra colours. (though maybe vertex colour blending could be allowed)

### currently implemented
- connecting vr.p8 to pico-8
- vr.p8: writing device status to gpio
- vr.p8: reading transforms from upper memory
- pico-8: writing transforms to upper memory
- pico-8: reading device status from gpio

### todo
- vr.p8: implement openxr
- vr.p8: get vr device poses and input states into pico-8
- vr.p8: render triangles
- vr.p8: get textures from pico-8 (spritesheet and display, map?)
- pico-8: a demo game, probably port bad saber and finish it

### openxr input (hmd/controller pose, buttons) (vr to pico, for input)
starts at gpio address (`0x5f80`)
```
-- targeting quest/pico controller layout for now
-- 16 bytes for controls
u8 (unused)
u8 buttons:
- a
- b
- x
- y
- left_stick
- right_stick
- left_menu (quest menu / index touchpad)
- right_menu (quest unmapped / index touchpad)
u8 left_trigger
u8 right_trigger
u8 left_grip
u8 right_grip
i16 left_stick_x
i16 left_stick_y
i16 right_stick_x
i16 right_stick_y
u8 left_rumble
u8 right_rumble
-- 36 bytes for device poses (12 per device)
i16 hmd_x
i16 hmd_y
i16 hmd_z
i16 hmd_yaw
i16 hmd_pitch
i16 hmd_roll
i16 left_x
i16 left_y
i16 left_z
i16 left_yaw
i16 left_pitch
i16 left_roll
i16 right_x
i16 right_y
i16 right_z
i16 right_yaw
i16 right_pitch
i16 right_roll
```

### transform buffer (pico to vr, for rendering)
starts at upper memory address (`0x8000`)

coordinates are integers; the rendered world should be scaled accordingly: currently defined as 1 unit = 1cm.
```
1 x
2 x
3 y
4 y
5 z
6 z
7 u + switches
    0-1 mode switch (one of: meta, vertex, point, unused)
    2-3 variable
    4-7 colour (typically)
8 varies by mode: see below
```

Meta transforms are used to control certain things, like render space.
```
7 meta 1
    2 enable flag (render space)
    3-5 enable flag (unused)
    6-7 render space (one of: hmd, world, left, right)
8 meta 2
    0-7 unused
```
If no flags are set, they do nothing and can be used to separate tri fans.

Vertex transforms draw triangles in fans: after drawing one tri, its last two verts are reused for the next tri.
```
7 u + switches
    2-3 u / (unused)
    4-7 u / colour
8 v + switches
    0 vertex colour/UV switch
    1 vertex blend switch
    2-7 v / (unused)
```
- Colour of the triangle and its blend state are determined by the last transform.
- A tri fan stops once a non-vertex transform is reached.
- UVs have a precision of 0-63, in half-tile steps
    - this allows for wrapping uvs, since 0-32 cover the full map

Point transforms let shapes be drawn at a location.
```
7 shape
    2-3 shape (circle, small billboard, large billboard, 3)
8 shape value (0-255)
```
- for a circle, 8 is used as the radius.
- for a billboard, 8 is used as the sprite index, and colour is used as size.
    - Small billboards are 1-16cm (1cm step) and use one sprite. They always face the camera on yaw and pitch.
    - Large billboards are 20-320cm (20cm step) and use sprites from 1x1 to 4x4 based on size. They only face the camera on yaw.

### screenspace canvas
potentially, we could utilise pico-8's normal display memory and reconstruct it as a texture to display for screenspace HUD elements. This should follow the display palette and transparency settings.
- different update rates may cause memory to be read partway through a frame. this would potentially cause transparency to be incorrect, leading to flickering as it changes across rendered frames. perhaps we allocate a portion of gpio to be read for the transparent colour indexes, or specify colour 0 or 15 as the transparent colour since the display palette is observed.
- with this it should also be possible to just play normal pico-8 games in the headset by setting the magic number and redirecting `btn()` and `btnp()` to vr.p8 inputs.

### thoughts
- updates will be juddery, as pico-8 runs at up to 60fps, while most hmds run at 90hz (2/3) -> (updated/rendered).
    - this could be mitigated by running pico-8 at 30fps and interpolating transforms for the other two frames. (1/3) (for hmds at 90hz)
    - or, set the hmd to a 60hz or 120hz refresh rate. (1 or 1/2)
    - or, run pico-8 on a display at the hmd refresh rate and use `_set_fps()` to match it.
        - this comes with other issues, however. like very low cpu limits, or whether that bug has been or will be fixed.
- maybe a transform type could be a "space switch", so we can have `world`, `hmd`, `left hand`, `right hand` spaces that transforms can be attached to, so that the pico-8 update rate doesn't cause controller tracking to feel juddery in-world.
    - should be resolved by `meta transforms`
    - i.e. you would change render space to left controller, place the verts for the hand, then it'll render on the hand regardless of if the transform buffer is updated or not.

## ...why?
idk i feel like it would be funny to play pico-8 with my pico 4.

also i want to build a vr game in pico-8 because i think it's possible, given an interface to the hardware.

see [bad saber](https://cubee.games/?rel=the_random_box&sub=bad_saber), my attempt at making it interface with webxr that stopped cause i couldn't get the screen to display in the headset. you can also play it with a cardboard vr emulator like trinus.

## references:
- borrowed pico-8 interface from [Pinput](https://github.com/VyrCossont/Pinput)
