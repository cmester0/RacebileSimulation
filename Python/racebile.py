from math import cos, sin, pi, floor, ceil, inf

import itertools

from maps import *
from draw_hex_map import *
from run_game import *

import time

# game_map, orig_players, start_line, mid_point, player_state_start, player_state_mid = chikane_map()
game_map, orig_players, start_line, mid_point, player_state_start, player_state_mid = rtfm_map()
# game_map, orig_players, start_line, mid_point, player_state_start, player_state_mid = loop_map()

# game_map, orig_players, start_line, mid_point = clover_map()
# game_map, orig_players, start_line, mid_point = tight_clover_map()
# game_map, orig_players, start_line, mid_point = pod_racing_map()

players = list(orig_players[:8])
out_of_map_counter = {}

drawing = DrawHexMap(900, 900)
drawing.set_map(game_map)

screenshot_counter = 0

# for i,j,d in {(-5, 11, 4), (-5, 10, 4), (-5, 9, 4)}: # , (31, -19, 5)
#     xi, yi = drawing.hex_coord(i, j)
#     for xj in range(-3,3+1):
#         for yj in range(-3,3+1):
#             drawing.draw_hex_dir(xi+xj, yi+yj, d, (200,0,0))

drawing.update_init()
drawing.save_map( f'Maps/000_map.png', [], (0, []), out_of_map_counter) # players
print ("saved")

last_sips = {}
last_total = 0

# Strategy

class Strategy:
    def __init__(self, gear_strategy, path_strategy):
        self.gear_strategy = gear_strategy
        self.path_strategy = path_strategy

def default_gear_strategy(g):
    ng = g + 1 if g < 3 else g # TODO: Strategy
    return ng

def default_path_strategy(l):
    print (l)
    return list(filter(lambda x: x[0] == min(l)[0], l))

default_strategy = Strategy(default_gear_strategy, default_path_strategy)

def manual_gear_strategy(g):
    return 3 # int(input(f"Current gear is {g}, what is your next gear: "))

def manual_path_strategy(l):
    global screenshot_counter
    
    oi = 1
    i = 0
    while True:
        time.sleep(0.1)

        # Did the user click the window close button?
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                match event.scancode:
                    case 80:
                        i = (i - 1) % len(l)
                    case 79:
                        i = (i + 1) % len(l)
                    case 44:
                        return [l[i]]
                    case 40: # Enter key
                        pygame.image.save(screen,f"Screenshots/screenshot_{screenshot_counter:03d}.png")
                        screenshot_counter += 1

        if oi != i:
            # Fill the background with white
            screen.fill((255, 255, 255))

            drawing.draw_map(players, (pl, [*l[i][2]]), {})
            img = Image.fromarray(drawing.m, "RGB")
            img = img.rotate(90)

            mode = img.mode
            size = img.size
            data = img.tobytes()
            py_image = pygame.image.fromstring(data, size, mode)
            rect = py_image.get_rect()
            screen.fill((255,255,255))
            screen.blit(py_image, rect)

            screen.blit(game_font.render(f"Path option: {i}", False, (0,0,0)), (drawing.width,drawing.height - 20))

            for sips_i, key in enumerate(last_sips):
                screen.blit(game_font.render(f"{key}: {sips[key]}", False, (0,0,0)), (drawing.width,20 * sips_i))
            screen.blit(game_font.render(f"============", False, (0,0,0)), (drawing.width,20 * len(last_sips)))
            screen.blit(game_font.render(f"Total sips: {last_total}", False, (0,0,0)), (drawing.width,20 * (len(last_sips)+1)))

            pygame.display.update()

        oi = i

    return [l[i]]

manual_strategy = Strategy(manual_gear_strategy, manual_path_strategy)

# Strategies:
# strat = default_strategy
strat = manual_strategy

logic = GameLogic(game_map, start_line, mid_point, player_state_start, player_state_mid, strat)

fell_off_map = [False for p in players]


# Running logic!
blocked = set()
drinking = [0 for p in players]
moves = [0 for p in players]

average_rounds = []

#############
# Game loop #
#############

import pygame
pygame.init()

# Set up the drawing window

screen = pygame.display.set_mode([drawing.width + 200, drawing.height])

# Run until the user asks to quit

running = True

pygame.font.init() # you have to call this at the start,
                   # if you want to use this module.
game_font = pygame.font.SysFont('Comic Sans MS', 30)

# Procedural generated racebile with powerups affecting map-gen
verbose = True
iters = 0
total_rounds = 100_000
while (iters < total_rounds):
    iters += 1

    # if verbose:
    #     print(f'\nframe {iters:03d}.png')
    #     filename = f'Maps/{iters:03d}_{pl:02d}_b_map.png'
    #     drawing.save_map(filename, players, (0, []),out_of_map_counter)
    for pl,((x,y),d,g,player_state,rounds) in enumerate(players):
        # Go to next player?
        next_player = False
        while not next_player:
            time.sleep(0.1)

            # Did the user click the window close button?
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    exit()

                if event.type == pygame.KEYUP:
                    match event.scancode:
                        case 44: # Spacebar
                            next_player = True
                        case 40: # Enter key
                            pygame.image.save(screen,f"Screenshots/screenshot_{screenshot_counter:03d}.png")
                            screenshot_counter += 1


        player_steps, sips, steps, total_sips = logic.step_player(pl,players,fell_off_map,blocked)

        if sips["off_map"]:
            ox,oy = players[pl][0]
            if not (ox,oy) in out_of_map_counter:
                out_of_map_counter[(ox,oy)] = 0
            out_of_map_counter[(ox,oy)] += 1

        drinking[pl] += total_sips
        for opl in range(len(players)):
            if opl != pl: drinking[opl] += sips["halfway_cheer"] + sips["goal_cheer"] # Everyone drinks on cheers!

        moves[pl] += 1

        if verbose:
            print (sips)
            print (total_sips)
            print ()

        if iters < 0 and len(average_rounds) == 0:
            print (f"save {iters:03d}, player {pl:02d}", rounds)
            filename = f'Maps/{iters:03d}_{pl:02d}_a_map.png'
            drawing.save_map(filename, players, (pl, player_steps),out_of_map_counter)
            filename = f'Maps/{iters:03d}_{pl:02d}_b_map.png'
            drawing.save_map(filename, players, (pl, []),out_of_map_counter)

        # Fill the background with white
        screen.fill((255, 255, 255))

        drawing.draw_map(players, (pl, player_steps), {}, fading_steps = True)
        img = Image.fromarray(drawing.m, "RGB")
        img = img.rotate(90)

        mode = img.mode
        size = img.size
        data = img.tobytes()
        py_image = pygame.image.fromstring(data, size, mode)
        rect = py_image.get_rect()
        screen.fill((255,255,255))
        screen.blit(py_image, rect)

        screen.blit(game_font.render(f"Next player...", False, (0,0,0)), (drawing.width,drawing.height - 20))

        for sips_i, key in enumerate(sips):
            screen.blit(game_font.render(f"{key}: {sips[key]}", False, (0,0,0)), (drawing.width,20 * sips_i))
        screen.blit(game_font.render(f"============", False, (0,0,0)), (drawing.width,20 * len(sips)))
        screen.blit(game_font.render(f"Total sips: {total_sips}", False, (0,0,0)), (drawing.width,20 * (len(sips)+1)))
        last_total = total_sips
        last_sips = sips

        pygame.display.update()

        if all(rounds > 1 for _,_,_,_,rounds in players): # Any , All
            # if iters < 10:
            #     print (len(average_rounds), iters)
            #     print (players)
            #     filename = f'Maps/bug.png'
            #     drawing.save_map(filename, players, (pl, player_steps),out_of_map_counter)
            #     exit()

            players = list(orig_players[:len(players)])
            # logic = GameLogic(game_map, start_line, mid_point)
            fell_off_map = [False for p in players]
            blocked = set()

            # drinking = [0 for p in players]
            # moves = [0 for p in players]

            average_rounds.append(iters)
            if (len(average_rounds) % 100) == 0:
                print (len(average_rounds), iters, min(average_rounds), max(average_rounds), sum(average_rounds)/len(average_rounds))
            iters = 0
            break

    if len(average_rounds) > 1: # 10000:
        pygame.image.save(screen,f"Screenshots/screenshot_{screenshot_counter:03d}.png")
        screenshot_counter += 1
        break

print (out_of_map_counter)

filename = f'Maps/result_map.png'
drawing.save_map(filename, players, (pl, []), out_of_map_counter)

print ("Average:", sum(drinking) / sum(moves))
print ("Total:", sum(drinking))
print ("Per player", drinking)

drawing.frames[0].save(f'Maps/map.gif', append_images=drawing.frames[1:], save_all=True, duration=120, loop=0)

# Done! Time to quit.
pygame.quit()
