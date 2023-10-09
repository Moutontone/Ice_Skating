use rand::Rng;
use rusttype::{point, Point};

// gride size
const N: usize = 7;
// probability precistion
const P_PRECISION: usize = 10000;
// probability to turn
const P: f32 = 0.8;
// number of turn
const T: usize = 20;

#[derive(Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}


impl Directions {
    fn to_ind(&self) -> usize {
        match self {
            Directions::Up => 0,
            Directions::Down => 1,
            Directions::Left => 2,
            Directions::Right => 3,
        }
    }

}

struct Mem {
    // results memory -> mem[t][i][j][Direction]
    mem_reward: [[[[f32; 4]; N]; N]; T + 1], // = [[[[-1., -1., -1., -1.]; N]; N]; T];
    mem_best_action: [[[[Directions; 4]; N]; N]; T + 1], //= [[[[Directions::Up; 4]; N]; N]; T];
}
impl Mem {
    fn action_at(&self, t: usize, s: &State) -> Directions {
        self.mem_best_action[t][s.position.x as usize][s.position.y as usize][s.direction.to_ind()]
    }

    fn is_uncomputed(&self, t: usize, s: &State) -> bool {
        self.mem_reward[t][s.position.x as usize][s.position.y as usize][s.direction.to_ind()]
            == -1.
    }

    fn set_score(&mut self, t: usize, s: &State, arg: f32) -> () {
        self.mem_reward[t][s.position.x as usize][s.position.y as usize][s.direction.to_ind()] = arg
    }

    fn set_action(&mut self, t: usize, s: &State, arg: Directions) -> () {
        self.mem_best_action[t][s.position.x as usize][s.position.y as usize]
            [s.direction.to_ind()] = arg
    }

    fn get_reward(&self, t: usize, s: &State) -> f32 {
        self.mem_reward[t][s.position.x as usize][s.position.y as usize][s.direction.to_ind()]
    }
}

struct State {
    position: rusttype::Point<i32>,
    direction: Directions,
}

impl Copy for State {

}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            position: self.position,
            direction: self.direction,
        }
    }
}

impl State {
    fn avance(&self) -> State {
        let p = self.position;
        let a = self.direction;
        State {
            position: match a {
                Directions::Up => point(p.x - 1, p.y),
                Directions::Down => point(p.x + 1, p.y),
                Directions::Left => point(p.x, p.y - 1),
                Directions::Right => point(p.x, p.y + 1),
            },
            direction: a,
        }
    }

    fn change_dir(self, a: Directions) -> State {
        State {
            position: self.position,
            direction: a,
        }
    }
}

impl Copy for Directions {}
impl Clone for Directions {
    fn clone(&self) -> Self {
        match self {
            Directions::Up => Directions::Up,
            Directions::Down => Directions::Down,
            Directions::Left => Directions::Left,
            Directions::Right => Directions::Right,
        }
    }
}

// impl ops::Add<Directions> for State {
//     type Output = State;
//
//     fn add(self, a: Directions) -> State {
//         let p = self.position;
//         State {
//             position: match a {
//                 Directions::Up => point(p.x - 1, p.y),
//                 Directions::Down => point(p.x + 1, p.y),
//                 Directions::Left => point(p.x, p.y - 1),
//                 Directions::Right => point(p.x, p.y + 1),
//             },
//             direction: a,
//         }
//     }
// }

fn main() {
    let map: [[u32; N]; N] = [
        [0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 1, 1, 0, 0, 1, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];
    let goal = point(1, 5);
    let init: State = State {
        position: point(5, 1),
        direction: Directions::Up,
    };

    let mut m = Mem {
        mem_reward: [[[[-1., -1., -1., -1.]; N]; N]; T + 1],
        mem_best_action: [[[[Directions::Up; 4]; N]; N]; T + 1],
    };
    let mem = &mut m;

    let mut s = init;
    draw_map(map, goal, &s);
    for t in 1..T {
        println!("----- turn {:}",t);
        println!("Current Direction {:?}",s.direction);
        //let _a = policy_rdm(map, goal, &s);
        let a = policy(mem, map, goal, &s, t);

        println!("Chosen action {:?}",a);
        println!();

        s = make_action(s, a);
        draw_map(map, goal, &s);

        if is_game_over(map, &s) {
            break;
        };
    }
    if is_game_won(goal, &s) {
        println!("You Won! ");
    } else {
        println!("Game lost...");
    }
}

fn policy_rdm(_map: [[u32; N]; N], _goal: Point<usize>, _s: &State) -> Directions {
    let p = rand::thread_rng().gen_range(0..3);
    match p {
        0 => Directions::Up,
        1 => Directions::Down,
        2 => Directions::Left,
        3 => Directions::Right,
        _ => Directions::Right,
    }
}

fn reward(goal: Point<usize>, s: &State) -> f32 {
    if s.position.x == goal.x as i32 && s.position.y == goal.y as i32 {
        1.
    } else {
        0.
    }
}
fn policy(
    mem: &mut Mem,
    map: [[u32; N]; N],
    goal: Point<usize>,
    s: &State,
    t: usize,
) -> Directions {
    if mem.is_uncomputed(t, s) {
        w(mem, map, goal, s, t);
    }
    mem.action_at(t, s)
}

fn w(mem: &mut Mem, map: [[u32; N]; N], goal: Point<usize>, s: &State, t: usize) -> f32 {
    if mem.is_uncomputed(t, s) {
        if s.position.x == goal.x as i32 && s.position.y == goal.y as i32 {
            // game won
            mem.set_score(t, s, 1.);
            mem.set_action(t, s, Directions::Up);
        } else if t >= T {
            // out of time
            mem.set_score(t, s, 0.);
            mem.set_action(t, s, Directions::Up);
        } else if map[s.position.x as usize][s.position.y as usize] == 0 {
            // stuck on dirt
            mem.set_score(t, s, 0.);
            mem.set_action(t, s, Directions::Up);
        } else {
            // recusive computation
            let (maxR, action) = [
                Directions::Up,
                Directions::Down,
                Directions::Left,
                Directions::Right,
            ]
            .iter()
            .fold((-1f32, Directions::Up), |(maxR, bestD), &a| {
                let w_turn = w(mem, map, goal, &(s.change_dir(a).avance()), t + 1);
                let w_no_turn = w(mem, map, goal, &s.avance(), t + 1);
                let res = reward(goal, s) + P * w_turn + (1. - P) * w_no_turn;
                if res > maxR {
                    (res, a)
                } else {
                    (maxR, bestD)
                }
            });
            mem.set_score(t, s, maxR);
            mem.set_action(t, s, action);
        }
    }
    mem.get_reward(t, s)
}

fn is_game_won(goal: rusttype::Point<usize>, s: &State) -> bool {
    s.position.x == goal.x as i32 && s.position.y == goal.y as i32
}

fn is_game_over(m: [[u32; N]; N], player: &State) -> bool {
    m[player.position.x as usize][player.position.y as usize] == 0
}

fn make_action(s: State, a: Directions) -> State {
    let p = rand::thread_rng().gen_range(0..P_PRECISION);
    if p as f32 <= P * P_PRECISION as f32 {
        // make the turn
        s.change_dir(a)
    } else {
        // do not turn
        s
    }
    .avance()
}

fn draw_map(m: [[u32; N]; N], goal: rusttype::Point<usize>, player: &State) -> () {
    let position = player.position;
    for i in 0..N {
        for j in 0..N {
            if position.x == i as i32 && position.y == j as i32 {
                print!(" P")
            } else if goal.x == i && goal.y == j {
                print!(" G")
            } else {
                match m[i][j] {
                    0 => {
                        print!(" #")
                    }
                    1 => {
                        print!(" .")
                    }
                    _ => {
                        print!(" ?")
                    }
                }
            }
        }
        println!()
    }
    println!()
}
