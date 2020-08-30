use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::ops::Add;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let wires = parse_wires(text);
    for wire in &wires {
        for point in wire {
            println!("{:?}", point);
        }
    }
    let a = &wires[0][0];
    let b = &wires[0][1];
    println!("a {:?} b {:?} a > b {}", a, b, a > b);
    println!(
        "Closes intersection: {:?}",
        find_closest_intersection(&wires[0], &wires[1])
    )
}

fn parse_wires(text: String) -> Vec<Wire> {
    // Read moves
    let moves_of_wires: Vec<Vec<Move>> = text
        .trim()
        .split("\n") // First split -- on two wires
        .map(|wire_text| {
            wire_text
                .trim()
                .split(",")
                .map(|string| string.into())
                .collect()
        })
        .collect();

    let origin = Point { x: 0, y: 0 };

    // Calculate every point of each wire
    let wires: Vec<Wire> = moves_of_wires
        .iter()
        .map(|wire_moves| {
            let mut current = origin.clone();
            wire_moves
                .iter()
                .map(|&move_| {
                    current = current + move_;
                    current
                })
                .collect()
        })
        .collect();

    wires
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(chr: char) -> Self {
        match chr {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl From<&str> for Move {
    fn from(string: &str) -> Self {
        Move {
            direction: string.chars().nth(0).unwrap().into(),
            distance: string.get(1..).unwrap().parse().unwrap(),
        }
    }
}

type Wire = Vec<Point>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    /// Compare Points by Manhattan distance between a point and an origin (Point {x: 0, y: 0}).
    /// Manhattan distance is sum of absolute values of every coordinate of the Point.
    fn cmp(&self, other: &Self) -> Ordering {
        // (self.x.abs() + self.y.abs()).cmp(&(other.x.abs() + other.y.abs()))
        let a = self.x.abs() + self.y.abs();
        let b = other.x.abs() + other.y.abs();
        println!("CMP: a = {} b = {}", a, b);
        a.cmp(&b)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Add<Move> for Point {
    type Output = Self;

    fn add(self, move_: Move) -> Self::Output {
        match move_.direction {
            Direction::Left => Point {
                x: self.x - move_.distance as i32,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + move_.distance as i32,
                y: self.y,
            },
            Direction::Up => Point {
                x: self.x,
                y: self.y + move_.distance as i32,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - move_.distance as i32,
            },
        }
    }
}

fn find_closest_intersection(left: &[Point], right: &[Point]) -> Option<Point> {
    let left_set: HashSet<Point> = left.into_iter().cloned().collect();
    let right_set: HashSet<Point> = right.into_iter().cloned().collect();
    left_set.intersection(&right_set).min().cloned()
}

#[test]
fn test_find_closest_intersection() {
    let cases = [
        (
            [
                [Point { x: 0, y: 6 }, Point { x: 5, y: 0 }],
                [Point { x: 5, y: 0 }, Point { x: 0, y: 6 }],
            ],
            Some(Point { x: 5, y: 0 }),
        ),
        (
            [
                [Point { x: 0, y: 6 }, Point { x: 5, y: 0 }],
                [Point { x: 0, y: 6 }, Point { x: 5, y: 0 }],
            ],
            Some(Point { x: 5, y: 0 }),
        ),
    ];

    for case in &cases {
        assert_eq!(
            find_closest_intersection(&case.0[0], &case.0[1]),
            Some(Point { x: 5, y: 0 })
        )
    }
}

#[test]
fn test_point_ordering() {
    let point1 = Point { x: 0, y: 6 };
    let point2 = Point { x: 5, y: 0 };
    assert_eq!(point1 > point2, true);
    assert_eq!(point1 < point2, false);

    let point1 = Point { x: -3, y: 6 };
    let point2 = Point { x: 5, y: 0 };
    assert_eq!(point1 > point2, true);
    assert_eq!(point1 < point2, false);

    let point1 = Point { x: 0, y: 6 };
    let point2 = Point { x: -5, y: 0 };
    assert_eq!(point1 > point2, true);
    assert_eq!(point1 < point2, false);

    let point1 = Point { x: 0, y: -6 };
    let point2 = Point { x: 5, y: 0 };
    assert_eq!(point1 > point2, true);
    assert_eq!(point1 < point2, false);

    let point1 = Point { x: 5, y: 5 };
    let point2 = Point { x: 5, y: 5 };
    assert_eq!(point1 > point2, false);
    assert_eq!(point1 < point2, false);

    let point1 = Point { x: -10, y: -10 };
    let point2 = Point { x: 5, y: 0 };
    assert_eq!(point1 > point2, true);
    assert_eq!(point1 < point2, false);

    let point1 = Point { x: 0, y: 3 };
    let point2 = Point { x: 5, y: 2 };
    assert_eq!(point1 > point2, false);
    assert_eq!(point1 < point2, true);
}

#[test]
fn test_wire_parsing() {
    let text = "R992,U221,L822,U805\nL998,U308,R889".to_owned();
    assert_eq!(
        parse_wires(text),
        [
            vec![
                Point { x: 992, y: 0 },
                Point { x: 992, y: 221 },
                Point { x: 170, y: 221 },
                Point { x: 170, y: 1026 }
            ],
            vec![
                Point { x: -998, y: 0 },
                Point { x: -998, y: 308 },
                Point { x: -109, y: 308 }
            ]
        ]
    )
}
