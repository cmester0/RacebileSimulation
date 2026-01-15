from math import cos, sin, pi, floor, ceil, inf, fmod
import colorsys

from PIL import Image
from PIL import GifImagePlugin
GifImagePlugin.LOADING_STRATEGY = GifImagePlugin.LoadingStrategy.RGB_AFTER_DIFFERENT_PALETTE_ONLY

from threading import Thread
import numpy as np

class DrawHexMap:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.game_map = None

        self.scale = None
        self.cx = None
        self.cy = None

        self.frames = []
        self.m = np.array([[(0, 0, 0) for j in range(self.width)] for i in range(self.height)],dtype=np.uint8)
        self.m_init = None

        self.star_color = (255, 0, 0)

        # self.hsv_default = ( 51.1/360,1.0,0.75) # 0
        # self.hsv_init    = ( 51.1/360,1.0,0.5 ) # 1
        # self.hsv_gear    = (210.6/360,1.0,0.75) # 2
        # self.hsv_star    = (  1.9/360,1.0,0.75) # 3

    def set_map(self, game_map):
        self.game_map = game_map
        self.compute_scale_and_center()
        self.pre_draw()
        self.update_init()

    def update_init(self):
        self.m_init = np.array(self.m)

    def raw_hax_coord(self, x, y):
        xs, ys = 3 * cos(pi/3), sin(pi/3)
        xi, yi = (xs * x, ys * (x + y * 2))
        return xi, yi

    def hex_coord(self,x,y):
        xi,yi = self.raw_hax_coord(x,y)
        xi = self.width // 2 + (xi-self.cx) * self.scale
        yi = self.height // 2 + (yi-self.cy) * self.scale
        return xi, yi

    def draw_hex(self, xi, yi, color):
        for k in range(round(-self.scale*cos(pi/3)), round(self.scale*cos(pi/3))+1):
            self.m[round(xi+k)][round(yi+self.scale*sin(pi/3))] = color
            self.m[round(xi+k)][round(yi-self.scale*sin(pi/3))] = color

        for k in range(0, self.scale + 1):
            self.m[round(xi-self.scale*cos(pi/3)-k*cos(pi/3))][round(yi-self.scale*sin(pi/3)+k*sin(pi/3))] = color
            self.m[round(xi-self.scale*cos(pi/3)-k*cos(pi/3))][round(yi+self.scale*sin(pi/3)-k*sin(pi/3))] = color

            self.m[round(xi+self.scale*cos(pi/3)+k*cos(pi/3))][round(yi-self.scale*sin(pi/3)+k*sin(pi/3))] = color
            self.m[round(xi+self.scale*cos(pi/3)+k*cos(pi/3))][round(yi+self.scale*sin(pi/3)-k*sin(pi/3))] = color

    def draw_filled_small_hex(self, xi, yi, color):
        for k in range(1, self.scale // 2 + 1):
            for i in range(round(xi-self.scale // 2*cos(pi/3)-k*cos(pi/3))+1, round(xi+self.scale // 2*cos(pi/3)+k*cos(pi/3))-1 +1):
                self.m[i][round(yi-self.scale // 2*sin(pi/3)+k*sin(pi/3))] = color
                self.m[i][round(yi+self.scale // 2*sin(pi/3)-k*sin(pi/3))] = color

    def draw_filled_hex(self, xi, yi, color):
        for k in range(1, self.scale + 1):
            for i in range(round(xi-self.scale*cos(pi/3)-k*cos(pi/3))+1, round(xi+self.scale*cos(pi/3)+k*cos(pi/3))-1 +1):
                self.m[i][round(yi-self.scale*sin(pi/3)+k*sin(pi/3))] = color
                self.m[i][round(yi+self.scale*sin(pi/3)-k*sin(pi/3))] = color

    def draw_hex_dir(self, xi, yi, d, color):
        for k in range(int(self.scale*3/4)):
            rx,ry = cos(d*pi/3+pi/6)*k, sin(d*pi/3+pi/6)*k
            self.m[int(xi+rx)][int(yi+ry)] = color

            for i in range(2):
                self.m[int(xi+rx+i)][int(yi+ry)] = color
                self.m[int(xi+rx-i)][int(yi+ry)] = color

                self.m[int(xi+rx)][int(yi+ry+i)] = color
                self.m[int(xi+rx)][int(yi+ry-i)] = color

    def draw_circle(self, xi, yi, radius, color):
        for i in range(360):
            self.m[round(xi + radius * cos(i / 360 * 2 * pi))][round(yi + radius * sin(i / 360 * 2 * pi))] = color

    def draw_grid(self, x_min, x_max, y_min, y_max):
        # Grid
        for i in range(x_min, x_max+1):
            for j in range(y_min, y_max+1):
                xi, yi = self.hex_coord(i, j)

                if not (self.scale <= xi < self.width-self.scale and self.scale <= yi < self.width-self.scale):
                    continue

                color = (255, 255, 255)
                self.draw_hex(xi, yi, color)

    def pre_draw(self):
        raw_coords = [(xi, yi) for xi, yi in self.game_map]
        xm = list(map(lambda x: x[0], raw_coords))
        x_max, x_min = (max(xm), min(xm))

        ym = list(map(lambda x: x[1], raw_coords))
        y_max, y_min = (max(ym), min(ym))

        # Grid
        self.draw_grid(x_min-1, x_max+1, y_min-1, y_max+1)

        # Map
        for (i,j) in self.game_map:
            dirs,t = self.game_map[(i,j)]

            xi, yi = self.hex_coord(i, j)

            if not (self.scale <= xi <= self.width-self.scale and self.scale <= yi <= self.height-self.scale):
                continue

            h = 210.6/360 if 2 in t else 51.1/360
            s = 1.0
            v = 0.5 if 1 in t else 0.75
            r, g, b = colorsys.hsv_to_rgb(h,s,v)

            color = (int(r * 255), int(g * 255), int(b * 255))

            self.draw_filled_hex(xi, yi, color)

            if 3 in t:
                h = 1.9/360
                s = 1.0
                v = 0.75
                r, g, b = colorsys.hsv_to_rgb(h,s,v)
                color = (int(r * 255), int(g * 255), int(b * 255))
                self.draw_filled_small_hex(xi, yi, color)

            for d in dirs:
                if 4 in t:
                    self.draw_hex_dir(xi, yi, d, (100,100,100))
                elif 5 in t:
                    self.draw_hex_dir(xi, yi, d, (0,0,0))
                else:
                    self.draw_hex_dir(xi, yi, d, (255,255,255))

    def draw_map(self, players, player_steps, fell_off_map, fading_steps=True):
        # pre_draw(m,game_map, cx, cy, scale)
        self.m = np.array(self.m_init,dtype=np.uint8)

        pl, player_steps = player_steps
        for ps,(i,j,d) in enumerate(player_steps):
            xi, yi = self.hex_coord(i, j)

            if not (self.scale <= xi <= self.width-self.scale and self.scale <= yi <= self.width-self.scale):
                continue

            h = 5 * pl / 8 # len(players)
            s = ((ps+1) / (len(player_steps)+1)) if fading_steps else 1
            v = ((ps+1) / (len(player_steps)+1)) if fading_steps else 1
            r, g, b = colorsys.hsv_to_rgb(h,s,v)

            color = (int(r * 255), int(g * 255), int(b * 255))

            self.draw_circle(xi, yi, self.scale // 3 + 2 * pl, color)
            self.draw_circle(xi, yi, self.scale // 3 - 2 * pl, color)

            self.draw_circle(xi, yi, self.scale // 3 + 2 * pl + 1, color)
            self.draw_circle(xi, yi, self.scale // 3 - 2 * pl - 1, color)

            self.draw_hex_dir(xi, yi, d, color)

        # Players
        for pl, ((i,j),d,_,_,_) in enumerate(players):
            xi, yi = self.hex_coord(i, j)

            if not (self.scale <= xi <= self.width-self.scale and self.scale <= yi <= self.width-self.scale):
                continue

            h = 5 * pl / 8 # len(players)
            s = 1.0
            v = 1.0
            r, g, b = colorsys.hsv_to_rgb(h,s,v)

            color = (int(r * 255), int(g * 255), int(b * 255))

            self.draw_circle(xi, yi, self.scale // 3 + 2 * pl, color)
            self.draw_circle(xi, yi, self.scale // 3 - 2 * pl, color)

            self.draw_circle(xi, yi, self.scale // 3 + 2 * pl + 1, color)
            self.draw_circle(xi, yi, self.scale // 3 - 2 * pl - 1, color)

            self.draw_hex_dir(xi, yi, d, color)

        # Players
        fell_off_total = max([fell_off_map[(x,y)] for x, y in fell_off_map] + [1])
        for (x,y) in fell_off_map:
            h = 0
            s = 1
            v = fell_off_map[(x,y)]/(fell_off_total)
            r, g, b = colorsys.hsv_to_rgb(h,s,v)
            color = (int(r * 255), int(g * 255), int(b * 255))
            xi, yi = self.hex_coord(x, y)
            self.draw_filled_small_hex(xi, yi, color)

    def compute_scale_and_center(self):
        extra_x = 1
        extra_y = 0

        raw_coords = [self.raw_hax_coord(xi, yi) for xi, yi in self.game_map]
        xm = list(map(lambda x: x[0], raw_coords))
        x_max, x_min = (max(xm)+extra_x, min(xm)-extra_x)

        ym = list(map(lambda x: x[1], raw_coords))
        y_max, y_min = (max(ym)+extra_y, min(ym))

        self.cx = (x_max + x_min) // 2
        self.cy = (y_max + y_min) // 2

        self.scale = int(min(self.width / 2 / (x_max+2-self.cx), self.height / 2 / (y_max+2-self.cy)))

    def save_map(self, filename, players, player_steps, fell_off_map):
        self.draw_map(players, player_steps, fell_off_map)

        img = Image.fromarray(self.m, "RGB")
        img = img.rotate(90)

        t = Thread(target=lambda i,f: i.save(f), args=[img, filename])
        t.start()

        self.frames.append(img)

        return img

    def inverse_raw_hax_coord(self, xi, yi):
        xs, ys = 3 * cos(pi/3), sin(pi/3)
        x = xi / xs
        y = (yi / ys - x) / 2
        return x, y

    def inverse_hex_coord(self, xi, yi):
        xi = (xi - self.width // 2) / self.scale + self.cx
        yi = (yi - self.height //2) / self.scale + self.cy
        x, y = self.inverse_raw_hax_coord(xi,yi)
        return x, y

    def load_map(self, filename):
        # self.draw_map(players, player_steps, fell_off_map)

        img = Image.open(filename)
        img.load()
        img = img.rotate(-90)
        self.m = np.array(np.asarray(img)) # np.array([[(0, 0, 0) for j in range(self.width)] for i in range(self.height)],dtype=np.uint8)

        extra_x = 1
        extra_y = 0

        for y in range(self.height):
            for x in range(self.width):
                if self.m[x][y][0] != 0:
                    start_of_some_hex = [x, y]
                    break
            else:
                continue

        self.scale = 0
        while (self.m[start_of_some_hex[0] + 1 + self.scale][start_of_some_hex[1]][0] != 0):
            self.scale += 1
        self.scale -= 1

        self.cx = fmod(start_of_some_hex[0], self.scale * 2 * 3 * cos(pi/3))
        self.cy = fmod(start_of_some_hex[1], self.scale * sin(pi/3))

        self.cx -= 18 * (2 * 3 * cos(pi/3))+0.0
        self.cy -= 0.8
        # self.cy += 10 * (self.scale * 2 * 3 * cos(pi/3))

        # print ( self.hex_coord (s_xi-int(s_xi), s_yi-int(s_yi)) , self.hex_coord(0,0) )

        self.game_map = {}

        for i in range(0, self.width): # -int(s_xi), 2*(int(s_xi)+1)+1):
            for j in range(-self.height // self.scale, self.height // self.scale): # (-int(s_yi), 2*(int(s_yi)+1)+1):
                xi, yi = self.hex_coord(i, j)

                if not (self.scale <= xi < self.width-self.scale and self.scale <= yi < self.width-self.scale):
                    continue

                # Outer not zero
                r,g,b = self.m[int(xi - 0.2 * self.scale)][int(yi + 0.7 * self.scale)] # Sample from outer ring of color
                if r != 0 or g != 0 or b != 0:
                    # Outer
                    h,s,v = colorsys.rgb_to_hsv(r,g,b)

                    values = []

                    if (0.45*255 <= v <= 0.55*255):
                        values.append(1)

                    if (205/360 <= h <= 215/360):
                        values.append(2)

                    # Inner Ring
                    r,g,b = self.m[int(xi)][int(yi)] # Sample from outer ring of color
                    # r,g,b = self.m[int(xi - 0.2 * self.scale)][int(yi + 0.25 * self.scale)] # Sample from outer ring of color
                    h,s,v = colorsys.rgb_to_hsv(r,g,b)

                    # Star
                    # values.append(3)
                    if (0.1/360 <= h <= 5/360):
                        values.append(3)

                    # # Directions:
                    directions = []
                    # k = self.scale*0.6 # goes to 3/4
                    # for d in range(6):
                    #     rx,ry = cos(d*pi/3+pi/6)*k, sin(d*pi/3+pi/6)*k
                    #     r,g,b = self.m[int(xi+rx)][int(yi+ry)]
                    #     h,s,v = colorsys.rgb_to_hsv(r,g,b)

                    #     if s == 0:
                    #         if v == 100:
                    #             values.append(4)
                    #         if v == 0:
                    #             values.append(5)
                    #         directions.append(d)

                    self.game_map[(i,j)] = (directions,values)

                    print ((i,j),":",(directions,values),",")

        for i in range(-3,3+1):
            for j in range(-3,3+1):
                self.m[int(self.width//2+i)][int(self.height//2+j)] = [200,100,0]

        for i in range(-self.width // self.scale, self.width // self.scale): # -int(s_xi), 2*(int(s_xi)+1)+1):
            for j in range(-self.height // self.scale, self.height // self.scale): # (-int(s_yi), 2*(int(s_yi)+1)+1):
                xi, yi = self.hex_coord(i, j)

                if not (self.scale <= xi < self.width-self.scale and self.scale <= yi < self.width-self.scale):
                    continue

                color = (255, 255, 255)
                # if not (i == 0 and j == 0):
                #     continue

                for k in range(-1,1+1):
                    for l in range(-1,1+1):
                        self.m[int(xi+k)][int(yi+l)] = [255,0,0]

                k = self.scale*0.6 # goes to 3/4
                for d in range(6):
                    rx,ry = cos(d*pi/3+pi/6)*k, sin(d*pi/3+pi/6)*k
                    self.m[int(xi+rx)][int(yi+ry)] = [100,255,100]

        self.m = np.array([[(0, 0, 0) for j in range(self.width)] for i in range(self.height)],dtype=np.uint8)
        self.compute_scale_and_center()
        self.pre_draw()

        self.m_init = np.array(self.m)

    def load_outline(self): # Inverse of pre_draw
        pass

        # raw_coords = [(xi, yi) for xi, yi in self.game_map]
        # xm = list(map(lambda x: x[0], raw_coords))
        # x_max, x_min = (max(xm), min(xm))

        # ym = list(map(lambda x: x[1], raw_coords))
        # y_max, y_min = (max(ym), min(ym))

        # # Grid
        # for i in range(x_min-1, x_max+1+1):
        #     for j in range(y_min-1, y_max+1+1):
        #         xi, yi = self.hex_coord(i, j)

        #         if not (self.scale <= xi < self.width-self.scale and self.scale <= yi < self.width-self.scale):
        #             continue

        #         color = (255, 255, 255)
        #         self.draw_hex(xi, yi, color)

        # # Map
        # for (i,j) in self.game_map:
        #     dirs,t = self.game_map[(i,j)]

        #     xi, yi = self.hex_coord(i, j)

        #     if not (self.scale <= xi <= self.width-self.scale and self.scale <= yi <= self.height-self.scale):
        #         continue

        #     h = 210.6/360 if 2 in t else 51.1/360
        #     s = 1.0
        #     v = 0.5 if 1 in t else 0.75
        #     r, g, b = colorsys.hsv_to_rgb(h,s,v)

        #     color = (int(r * 255), int(g * 255), int(b * 255))

        #     self.draw_filled_hex(xi, yi, color)

        #     if 3 in t:
        #         h = 1.9/360
        #         s = 1.0
        #         v = 0.75
        #         r, g, b = colorsys.hsv_to_rgb(h,s,v)
        #         color = (int(r * 255), int(g * 255), int(b * 255))
        #         self.draw_filled_small_hex(xi, yi, color)

        #     for d in dirs:
        #         if 4 in t or 5 in t:
        #             self.draw_hex_dir(xi, yi, d, (0,0,0))
        #         else:
        #             self.draw_hex_dir(xi, yi, d, (255,255,255))
