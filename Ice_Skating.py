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
pos_start = (1,3)
dir_start = left
goal = (1,5)
p_turn = 0.83
T = 10

# Using Markoc Decision process M = {S, A, r, P}

# States are couple (position, direction)

# movements in order up, left, down, right

# W = [[[-1 for _ in Actions] for _ in range(len(map[0]))] for _ in range(len(map))]
dic = {
        (goal, up): (1, up),
        (goal, right): (up, 1),
        (goal, down): (up, 1),
        (goal, left): (up, 1),
        }

memory = np.full((T,7,7,4), -1)
print(f"{memory[0][0][0][0]}")

def policy(pos, dir, t):
    _, a = W(pos, dir, t)
    return a

def W(pos, dir, t):
    if t == T:
        return (reward(pos), up)
    if map[pos[0]][pos[1]] == 0:
        return (0, up)
    max = -1
    action_max = ()
    for a in Actions:
        w1, _ = W((pos[0] + a[0], pos[1] + a[1]), a, t+1)
        w2, _ = W((pos[0] + dir[0], pos[1] + dir[1]), dir, t+1)
        res = reward(pos) + p_turn * w1 + (1-p_turn) * w2
        if res > max:
             max = res
             action_max = a
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
