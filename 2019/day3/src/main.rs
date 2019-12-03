use std::fs::read_to_string;
use crate::Move::{Left, Right, Up, Down};
use std::collections::HashSet;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[inline]
fn point(x: i64, y: i64) -> Point {
    Point { x, y }
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Move {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

impl Move {
    fn len(&self) -> i64 {
        match *self {
            Left(n) => n,
            Right(n) => n,
            Up(n) => n,
            Down(n) => n,
        }
    }

    fn direction(&self) -> Point {
        match *self {
            Left(_) => point(-1, 0),
            Right(_) => point(1, 0),
            Up(_) => point(0, 1),
            Down(_) => point(0, -1),
        }
    }
}

impl From<&str> for Move {
    fn from(raw: &str) -> Self {
        let str = raw.to_string();

        let direction = str.chars().next().unwrap();

        let num = str.chars()
            .skip(1)
            .collect::<String>()
            .parse::<i64>().unwrap();

        match direction {
            'L' => Left(num),
            'R' => Right(num),
            'U' => Up(num),
            'D' => Down(num),
            _ => panic!("Bad direction input")
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


fn main() {
    let moves = read_input("input.txt");

    assert_eq!(moves.len(), 2);

    let moves1 = moves[0].to_owned();
    let moves2 = moves[1].to_owned();

    let wire1 = wire_from_moves(&moves1);
    let wire2 = wire_from_moves(&moves2);

    let res = wire1.intersection(&wire2)
        .map(Point::manhattan)
        .min()
        .unwrap();

    println!("Result {:?}", res)
}

fn read_input(path: &str) -> Vec<Vec<Move>> {
    read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|line| {
            line
                .split(",")
                .map(Move::from)
                .collect::<Vec<Move>>()
        })
        .collect()
}

fn wire_from_moves(moves: &Vec<Move>) -> HashSet<Point> {
    let mut wire = HashSet::with_capacity(moves.len() * 50);
    let mut cursor = point(0, 0);

    moves.iter().for_each(|movement| {
        let direction = movement.direction();

        for _ in 0..movement.len() {
            cursor += direction;
            wire.insert(cursor);
        }
    });

    wire
}