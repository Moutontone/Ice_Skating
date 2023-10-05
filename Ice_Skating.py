# /bin/python
import random
import numpy as np

def main():
    print("Move the Player (P) to the goal (G)")

    pos = pos_start
    dir = dir_start
    print_map(pos)
    for t in range(T):

        action = random.choice(Actions)
        #action = policy(pos, dir, t)
        print(f"action : {action}")

        pos, dir = try_movement(pos, dir, action)
        print_map(pos)

        if pos == goal:
            print("Well played you won")
            break;
        elif map[pos[0]][pos[1]] == 0:
            print("You fell on dirt")
            break;
    print("end of Game")

map = [[0, 0, 0, 0, 0, 0, 0],
       [0, 1, 1, 1, 0, 1, 0],
       [0, 1, 1, 1, 1, 1, 0],
       [0, 1, 1, 1, 1, 1, 0],
       [0, 1, 1, 0, 0, 1, 0],
       [0, 1, 1, 1, 1, 1, 0],
       [0, 0, 0, 0, 0, 0, 0]]

up, left, down, right = (-1,0), (0,-1), (1, 0), (0, 1)
Actions = [up, left, down, right]

#pos_start = (5,1)
pos_start = (2,5)
dir_start = left
goal = (1,5)
p_turn = 0.83
T = 5

# Using Markoc Decision process M = {S, A, r, P}

# States are couple (position, direction)

# movements in order up, left, down, right

# dic key : (t, pos, dit) -> val : (reward, action)
dic = {
        (T, goal, up): (1, up),
        (T, goal, right): (1, up),
        (T, goal, down): (1, up),
        (T, goal, left): (1, up),
        }

memR = np.full((T+1,7,7,4),-1)
memA = np.full((T+1,7,7,4),-1)
mem = np.full((T+1,7,7,4),-1)

# base cases
for i in range(7):
    for j in range(7):
        for a in range(4):
            if (i,j) == goal:
                memR[T][i][j][a] = 1
                memA[T][i][j][a] = 0
            else:
                memR[T][i][j][a] = 0
                memA[T][i][j][a] = 0

def has_val(t, pos, dir):
    a = 0
    if dir == left : 
        a = 1
    if dir == down : 
        a = 2
    if dir == right : 
        a = 3
    return memR[t][pos[0]][pos[1]][a] != -1

def get_val(t, pos, dir):
    a = 0
    if dir == left : 
        a = 1
    if dir == down : 
        a = 2
    if dir == right : 
        a = 3
    r = memR[t][pos[0]][pos[1]][a]
    a = memA[t][pos[0]][pos[1]][a]
    if a == 0:
        return (r, up)
    if a == 1:
        return (r, left)
    if a == 2:
        return (r,down)
    if a == 3:
        return (r, right)

def set_val(t, pos, dir, v, ac):
    a = 0
    if dir == left : 
        a = 1
    if dir == down : 
        a = 2
    if dir == right : 
        a = 3
    memR[t][pos[0]][pos[1]][a] = v
    if ac == up:
        memA[t][pos[0]][pos[1]][a] = 0
    if ac == left:
        memA[t][pos[0]][pos[1]][a] = 1
    if ac == down:
        memA[t][pos[0]][pos[1]][a] = 2
    if ac == right:
        memA[t][pos[0]][pos[1]][a] = 3

def policy(pos, dir, t):
    _, a = W(pos, dir, t)
    return a

def W(pos, dir, t):

    # base cases
    if t >= T:
        # should be useless
        return (reward(pos), up)
    # situation already computed
    if has_val(t, pos, dir):
        return get_val(t, pos, dir)
     # compute new situation
    # dead end cases
    if map[pos[0]][pos[1]] == 0:
        return (0, up)
    # recusive case
    action_max = up
    max = -1
    for a in Actions:
        # turning successfuly 
        w1, _ = W((pos[0] + a[0], pos[1] + a[1]), a, t+1)
        # keep the same direction
        w2, _ = W((pos[0] + dir[0], pos[1] + dir[1]), dir, t+1)
        res = reward(pos) + p_turn * w1 + (1-p_turn) * w2
        if res > max:
             max = res
             action_max = a
    set_val(t, pos,dir, max, action_max)
    return (max, action_max)


# reward
def reward(pos):
    return pos == goal

# Probability to change state
def P(pos, dir, action, dir_walked):
    # keeping the direction
    if dir == action:
        if action == dir_walked and dir_walked == action:
            return 1
    # trying to make a turn
    else:
        # turn made
        if action == dir_walked:
            return p_turn
        # turn failed
        else :
            return 1 - p_turn
    assert(False)


def try_movement(pos, dir, action):
    p = random.uniform(0, 1)
    # succed to turn
    if p <= p_turn:
        pos = (pos[0] + action[0], pos[1] + action[1]) 
        dir = action
    # fail to turn
    else:
        pos = (pos[0] + dir[0], pos[1] + dir[1]) 
        #dir
    return (pos, dir)

def print_map(pos):
    
    for i in range(len(map)):
        for j in range(len(map[i])):
            # set char
            c = '0'
            if map[i][j] == 1:
                c = '.'
            # if (i, j) == pos_start:
            #     c = 'S'
            if (i, j) == goal:
                c = 'G'
            if pos == (i, j):
                c = 'P'
            # print tile
            print(f" {c}", end="")
        print(" |")
    print(" ")


if __name__ == "__main__":
    main()
