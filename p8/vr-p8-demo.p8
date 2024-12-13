pico-8 cartridge // http://www.pico-8.com
version 41
__lua__
-- vr.p8 test cart
-- by cubee 🐱

function _init()
 vr_init()
end

function _update()

 if vr_connected() then
  --poke(vr_buffer+1,rnd(-1))
 end
end

function _draw()
 -- vr space
 vr_clear_transforms()

 vr_add_transform(0,1,2)
 vr_add_transform(101,1002,10.5)
 vr_add_transform(32000,16000,8000)
 vr_add_transform(1024,1023,8192)
 vr_add_transform(-10,-20,-30)
 vr_add_transform(32767,-32768,32768)

 -- screen space
 cls()
 print(vr_connected() and "connected!" or "waiting...",40,8,vr_connected() and 11 or 13)

 -- show head of transforms
 print("transform buffer head:",8,24,13)
 for i=0,10 do
  ?i..": "..vrd_read_transform(i),8,32+i*6,7
 end
 
 -- show device poses
 for i=0,2 do
  local x,y,z,yaw,pit,rol=vr_device_pose(i)
  print(split("hmd,left,right")[i+1]..": "..x..","..y..","..z.." "..yaw..","..pit..","..rol,8,108+6*i,13)
 end
end

-->8

-->8
-- vr.p8
-- based on pinput by vyrcossont
--https://github.com/vyrcossont/pinput

vr_gpio=0x5f80
vr_buffer=0x8000
vr_trans_idx=0
vr_magic={
 0x5667.6d6f,
 0x506e.6f52,
 0x384f.4349,
 0x776f.5721
}

function vr_init()
 for i=0,#vr_magic-1 do
  poke4(vr_gpio+4*i,vr_magic[i+1])
 end
end

function vr_connected()
 return peek4(vr_gpio)~=vr_magic[1]
end

function vr_update()
 -- write occupied
 --poke(vr_buffer,0b00000001)
 
 
 -- write free
 --poke(vr_buffer,0b00000000)
end

function vr_clear_transforms()
 memset(vr_buffer,0,0x8000)
 vr_trans_idx=0
end

-- coords: int (-32768,32767)
function vr_add_transform(x,y,z)
 local stride=8
 local addr=vr_buffer+vr_trans_idx*stride

 poke2(addr,   x)
 poke2(addr+2, y)
 poke2(addr+4, z)

 --todo: change these based on mode?
 -- or use a function per mode

 -- mode, unused, colour
 --              md..colr
 poke (addr+6, 0b10001011)
 --              variable
 poke (addr+7, 0)


 vr_trans_idx+=1
end

-- hmd:0 left:1 right:2
function vr_device_pose(id)
 local stride=12

 local addr=vr_gpio+16+id*stride

 local x=peek2(addr)/1
 local y=peek2(addr+2)/1
 local z=peek2(addr+4)/1
 
 local yaw=peek2(addr+6)/3600
 local pit=peek2(addr+8)/3600
 local rol=peek2(addr+10)/3600
 
 return x,y,z,yaw,pit,rol
end

function vr_btn(b)
 --todo: deconstruct bitfield
 return peek(vr_addr+1)
end

-- lt:0 rt:1 lg:2 rg:3
function vr_trigger(t)
 return peek(vr_gpio+2+t)
end

-- x,y: left:0,1 right:2,3
function vr_axis(a)
 return peek2(vr_gpio+6+a*2)
end

-- left:0 right:1
function vr_rumble(r,a)
 poke(vr_gpio+14+r,a)
end

-->8
-- vr.p8 debug


function vrd_read_transform(id)
 local stride=8
 local addr=vr_buffer+id*stride

 local s=""

 s..=peek2(addr)..","..peek2(addr+2)..","..peek2(addr+4)
 s..=" ("..peek(addr+6)..", "..peek(addr+7)..")"

 return s
end
__gfx__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
