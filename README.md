# vr.p8
brainstorming ways to connect the [PICO-8 fantasy console](https://www.lexaloffle.com/pico-8.php) to OpenXR.

## helper program:
- find the pico-8 process, search its memory for a magic number like [pinput](https://github.com/VyrCossont/Pinput) does.
    - use this offset to read the rest of pico-8 memory since the layout should be the same from there.
    - e.g. the sprite memory would be below the magic number, while the display would be above.
- read the palette info, construct spritesheet as a texture. (detect updates and redraw?)
    - pico-8 has a fixed palette of 16 colours from a selection of 32.
- vr.p8 renders to the hmd using standard openxr stuff.
    - pico-8 builds a vertex buffer "scene" in memory that we process to construct the scene.
        - pico-8 handles the main game loop, mesh and texture mapping, object transforms, constructing the scene, etc.
        - all camera transformations, culling, rasterisation, depth sorting, (lighting?) is done on the vr.p8 side at hmd resolution.
        - should we utilise display memory for 2d hud elements in screen space?
- vr.p8 writes hmd/controller pose and input states to pico-8 memory.
    - pico-8's number precision doesn't matter if we're handling camera transforms in the vr.p8 application, unless we want to render something attached to the player (like hands or a ui). it should be precise enough for those purposes anyway.
    - performance of the pico-8 application also holds no bearing to the performance of vr rendering, so hmd view transforms will be smooth even if pico-8 is struggling.
- use display palette to render triangles.
    - this program is only for interfacing with the hmd, it shouldn't do anything more than pico-8 can do such as extra colours. (though maybe vertex colour blending could be allowed)

### openxr input (hmd/controller pose, buttons) (vr to pico, for input)
starts at gpio address (`0x5f80`)
```
-- targeting quest/pico controller layout for now
-- 16 bytes for controls
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
u8 (unused)
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
perhaps it would be better to map openxr actions instead of buttons.

### transform buffer (pico to vr, for rendering)
starts at upper memory address (`0x8000`)
first address contains a bool stating if the buffer is locked by pico-8
coordinates are integers; the rendered world should be scaled accordingly, say for example 1 unit is 1cm

```
1 x
2 x
3 y
4 y
5 z
6 z
7 u + switches
    0-1 mode switch (one of vertex, point, light?, or none)
    2-3 variable
    4-7 colour (typically)
8 varies by mode: see below
```
Using vertex transforms, tris are drawn in fans: after drawing one tri, its last two verts are reused for the next tri.
- Colour of the triangle and its blend state are determined by the last transform.
- A tri fan stops once a non-vertex transform is reached.
- UVs have a precision of 0-63, in half-tile steps
- this allows for wrapping uvs, since 0-32 cover the full map
```
7 u + switches
    2-3 u / (unused)
    4-7 u / colour
8 v + switches
    0 vertex colour/UV switch
    1 vertex blend switch
    2-7 v / (unused)
```

Point transforms let shapes be drawn at a location.
- for a circle, 8 is used as the radius.
- for a billboard, 8 is used as the sprite index, and colour is used as size. Large billboards are 1-16 units and take 2, 4, 6, or 8 sprites based on size, while small billboards are 0-1.5 units and use one sprite.
```
7 shape
    2-3 shape (circle, small billboard, large billboard, 3)
8 shape value (0-255)
```

Light transform is used as a light source?
```
8 light settings
    0-3 range (0-15 units)
    4-7 intensity (0-15)
```

A none transform can simply be used to separate triangle fans without rendering anything.

### screenspace canvas
potentially, we could utilise pico-8's normal display memory and reconstruct it as a texture to display for screenspace HUD elements. This should follow the display palette and transparency settings.
- different update rates may cause memory to be read partway through a frame. this would potentially cause transparency to be incorrect, leading to flickering as it changes across rendered frames. perhaps we allocate a portion of gpio to be read for the transparent colour indexes, or specify colour 0 or 15 as the transparent colour since the display palette is observed.
- with this it should also be possible to just play normal pico-8 games in the headset by setting the magic number and redirecting `btn()` and `btnp()` to vr.p8 inputs.

### currently implemented
- connecting runtime to pico-8
- writing to gpio
- reading from upper memory

### todo
- implement openxr
- get vr device poses and input states
- rendering triangles
- get textures from pico-8 (spritesheet and display, map?)
- a demo game in pico-8 (probably a bad saber remake)

### thoughts
- updates will be juddery, as pico-8 runs at up to 60fps, while most hmds run at 90hz (2/3).
    - this could be mitigated by running pico-8 at 30fps and interpolating transforms for the other two frames. (1/3) (for hmds at 90hz)
    - or, set the hmd to a 60hz or 120hz refresh rate. (1 or 1/2)
    - or, run pico-8 on a display at the hmd refresh rate and use `_set_fps()` to match it.
        - this comes with other issues, however. like very low cpu limits, or whether that bug has been or will be fixed.
- maybe a transform type could be a "space switch", so we can have `world`, `hmd`, `left hand`, `right hand` spaces that transforms can be attached to, so that the pico-8 update rate doesn't cause controller tracking to feel juddery in-world.
    - i.e. you would change render space to left controller, place the verts for the hand, then it'll render on the hand regardless of if the transform buffer is updated or not.

## ...why?
idk i feel like it would be funny to play pico-8 with my pico 4.
also i want to build a vr game in pico-8 because i think it's possible, given an interface to the hardware. see [bad saber](https://cubee.games/?rel=the_random_box&sub=bad_saber), my attempt at making it interface with webxr that stopped cause i couldn't get the screen to display in the headset. you can also play it with a cardboard emulator like trinus.

## references:
- borrowed pico-8 interface from [Pinput](https://github.com/VyrCossont/Pinput)
