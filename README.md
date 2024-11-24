# vr.p8
brainstorming ways to connect the [PICO-8 fantasy console](https://www.lexaloffle.com/pico-8.php) to OpenXR.

helper program:
- find the pico-8 process, search its memory for a magic number like [pinput](https://github.com/VyrCossont/Pinput) does.
    - use this offset to read the rest of pico-8 memory since the layout should be the same from there.
    - e.g. the sprite memory would be below the magic number, while the display would be above.
- read the palette info, construct spritesheet as a texture. (detect updates and redraw?)
    - pico-8 has a fixed palette of 16 colours from a selection of 32.
- vr.p8 renders to the hmd using standard openxr/vulkan/whatever stuff. how to we tell the runtime what to draw?
    - pico-8 builds a vertex buffer "scene" in memory that we process to construct the scene.
        - pico-8 handles the main game loop, mesh and texture mapping, object transforms, constructing the scene, etc.
        - all camera transformations, culling, rasterisation, depth sorting, (lighting?) is done on the vr.p8 side at hmd resolution.
    - pico-8 passes raw triangle/uv coords via the display memory.
        - is this screenspace or worldspace? does pico-8 handle everything but actual tri rendering? or do we just do verts+culling in pico-8 and vr.p8 does world>camera>screenspace transforms?
        - triangles are rendered in full resolution.
    - alternatively, pico-8 does all of the rendering, and we just reconstruct the display memory using the display palette.
      - performance is severely limited though, so the more we offload the better. but, we don't want to go so far that we're not "running vr on pico-8" anymore.
      - authentic 128x128 (64x128 per-eye, unless using multi-display mode) vr gameplay.
- vr.p8 writes hmd/controller pose and input states to pico-8 memory.
    - precision: how good is pico-8's fixed point precision for vr?
- use display palette to render triangles.
    - this program is only for interfacing with the hmd, it shouldn't do anything more than pico-8 can do such as extra colours. (though maybe vertex colour blending could be allowed)

openxr input (hmd/controller pose, buttons)
starts at gpio address (`0x5f80`)
```
-- targeting quest/pico controller layout for now
u8 buttons:
- a
- b
- x
- y
- left_stick
- right_stick
- left_menu
- 
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
```
perhaps it would be better to map openxr actions instead of buttons.

vertex buffer
starts at memory address 0x8000 (upper memory)
```
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
```
Tris are drawn in fans: after drawing one tri, its last two verts are reused for the next tri
If a vert has everything zeroed, stop here and start a new fan

UVs have a precision of 0-31 (half-tile steps)
    - they cannot reach the right/lower-most edge
    - may add another bit to solve that and so UVs can wrap


(old) javascript+webxr
- experiment here: [bad saber](https://cubee.games/?rel=the_random_box&sub=bad_saber)
- needs a way to capture the canvas and display it in the hmd.
    - it should just be a texture on the page somewhere, right? then we can put a plane covering the screen and call it a day or something.
    
references:
- borrowed pico-8 interface from [Pinput](https://github.com/VyrCossont/Pinput)
