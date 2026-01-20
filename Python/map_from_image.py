from math import cos, sin, pi, floor, ceil, inf

import itertools

from maps import *
from draw_hex_map import *
from run_game import *

# pre = DrawHexMap(2000, 2000)

# pre.scale = 40
# pre.cx = pre.width //2/pre.scale
# pre.cy = pre.height//2/pre.scale

# pre.draw_grid( -pre.width // pre.scale , pre.width // pre.scale, -pre.height // pre.scale , pre.height // pre.scale)
# pre.m_init = np.array(pre.m)
# pre.save_map("grid.png",[],(0,[]),{})

drawing = DrawHexMap(2000, 2000)
drawing.load_map(input("filename:"))

for x,y in drawing.game_map:
    print ((x,y),drawing.game_map[(x,y)])

drawing.save_map(input("output name:"),[],(0,[]),{})
