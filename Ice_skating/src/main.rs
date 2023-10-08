use std::ops::{self, Add};

use rand::Rng;
use rusttype::{point, Point};

// gride size
const N: usize = 7;
// probability precistion
const P_PRECISION: usize = 10000;
// probability to turn
const P: f32 = 0.8;
// number of turn
const T: usize = 5;

struct State {
    position: rusttype::Point<i32>,
    direction: Directions,
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

impl ops::Add<Directions> for State {
    type Output = State;

    fn add(self, a: Directions) -> State {
        let p = self.position;
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
}
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

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
        position: point(3, 5),
        direction: Directions::Up,
    };

    let mut s = init;
    draw_map(map, goal, &s);
    for t in (1..T) {
        let a = policy(map, goal, &s);
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

fn policy(map: [[u32; N]; N], goal: Point<usize>, s: &State) -> Directions {
    let p = rand::thread_rng().gen_range(0..3);
    match p  {
        0 => Directions::Up,
        1 => Directions::Down,
        2 => Directions::Left,
        3 => Directions::Right,
        _ =>Directions::Right,
    }
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
