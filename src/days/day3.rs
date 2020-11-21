extern crate advent;
use self::advent::*;
use std::*;

const START_POINT: Point = Point { x: 0, y: 0 };

pub fn run() {
    let filename = "inputs/day3.txt";
    let inputs = read_inputs(filename);
    let inputs : Vec<&str> = inputs.trim().split("\n").collect();
    let first_line = inputs.get(0).expect("Couldn't get first line");
    let second_line = inputs.get(1).expect("Couldn't get second line");

    let part_one = get_closest_intersection(&first_line, &second_line);
    println!("Part one: {}", part_one);

    let part_two = get_minimum_signal_delay_intersection(&first_line, &second_line);
    println!("Part two: {}", part_two);
}

fn get_closest_intersection(first_line: &str, second_line:&str) -> i32 {
    let first_line : Vec<&str> = first_line.trim().split(",").collect();
    let second_line : Vec<&str> = second_line.trim().split(",").collect();

    let first_moves = get_all_moves(&first_line);
    let second_moves = get_all_moves(&second_line);

    return first_moves.find_intersections(&second_moves)
    .iter()
    .map(|i| i.manhattan_distance(&START_POINT))
    .min()
    .unwrap();
}

fn get_minimum_signal_delay_intersection(first_line: &str, second_line: &str) -> i32 {
    let first_line : Vec<&str> = first_line.trim().split(",").collect();
    let second_line : Vec<&str> = second_line.trim().split(",").collect();

    let first_moves = get_all_moves(&first_line);
    let second_moves = get_all_moves(&second_line);

    return first_moves.find_intersections(&second_moves)
    .iter()
    .map(|i| signal_delay(i, &first_moves) + signal_delay(i, &second_moves))
    .min()
    .unwrap();
}

#[derive(Debug, PartialEq)]
struct Move {
    start: Point,
    end: Point,
    move_type: MoveType,
}

#[derive(Debug, PartialEq)]
enum MoveType {
    Horizontal,
    Vertical,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn get_all_moves(moves_text: &Vec<&str>) -> Vec<Move> {
    let start_point = START_POINT;
    let mut current_point = start_point.clone();
    let mut moves = vec![];

    for move_text in moves_text {
        let move_type = get_move_type(move_text);
        let move_distance = get_moves_distance(move_text);
        let end_point = current_point.move_point(&move_type, &move_distance);

        moves.push(Move {
            start: current_point,
            end: end_point,
            move_type: move_type,
        });

        current_point = end_point;
    }

    return moves;
}

fn get_move_type(move_text: &str) -> MoveType {
    match move_text.chars().next().unwrap() {
        'U' => MoveType::Vertical,
        'D' => MoveType::Vertical,
        'L' => MoveType::Horizontal,
        'R' => MoveType::Horizontal,
        _ => MoveType::Unknown,
    }
}

fn get_moves_distance(move_text: &str) -> i32 {
    let sign = match move_text.chars().next().unwrap() {
        'D' | 'L' => -1,
        _ => 1,
    };

    sign * move_text[1..]
        .parse::<i32>()
        .expect("Expected a positive integer as the move distance")
}

fn signal_delay(intersection: &Point, first_moves: &Vec<Move>) -> i32 {
    let mut delay = 0;

    for m in first_moves {
        let has_intersection = m.intersects(intersection);
        if has_intersection.is_some() {
            delay = delay + has_intersection.unwrap();
            break;
        } else {
            delay = delay + m.distance();
        }

    }
    return delay;
}

impl Point {
    fn move_point(&self, move_type: &MoveType, move_distance: &i32) -> Point {
        match move_type {
            MoveType::Vertical => Point {
                x: self.x,
                y: self.y + move_distance,
            },
            MoveType::Horizontal => Point {
                x: self.x + move_distance,
                y: self.y,
            },
            _ => self.clone(),
        }
    }

    fn manhattan_distance(&self, point: &Point) -> i32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

trait MoveIntersections {
    fn find_intersections(&self, other: &Vec<Move>) -> Vec<Point>;
}
impl MoveIntersections for Vec<Move> {
    fn find_intersections(&self, other: &Vec<Move>) -> Vec<Point> {
        let mut intersections = vec![];

        for m in other {
            let move_intersections : Vec<Point> = self.clone().into_iter()
            .filter_map(|f| f.find_intersection(m))
            .collect();
            intersections.extend(move_intersections);
        }

        intersections = intersections.into_iter().filter(|i| i != &START_POINT).collect();
        return intersections;
    }
}

impl Move {
    fn find_intersection(&self, other: &Move) -> Option<Point> {
        if self.move_type == other.move_type { return None };
        let vertical_move = if self.move_type == MoveType::Vertical {self} else {other};
        let horizontal_move = if self.move_type == MoveType::Horizontal {self} else {other};

        let vertical_top = cmp::max(vertical_move.start.y, vertical_move.end.y);
        let vertical_bottom = cmp::min(vertical_move.start.y, vertical_move.end.y);
        let vertical_x = vertical_move.start.x;
        let horizontal_right = cmp::max(horizontal_move.start.x, horizontal_move.end.x);
        let horizontal_left = cmp::min(horizontal_move.start.x, horizontal_move.end.x);
        let horizontal_y = horizontal_move.start.y;

        let has_intersection = vertical_x >= horizontal_left && vertical_x <= horizontal_right && horizontal_y >= vertical_bottom && horizontal_y <= vertical_top;

        return if has_intersection { Some(Point{x: vertical_x, y: horizontal_y})} else {None}
    }

    fn intersects(&self, point: &Point) -> Option<i32> {
        if (self.move_type == MoveType::Vertical && self.start.x == point.x) ||
           (self.move_type == MoveType::Horizontal && self.start.y == point.y) {
                return Some(self.start.manhattan_distance(&point));
        } 
        return None;
    }

    fn distance(&self) -> i32 {
        self.start.manhattan_distance(&self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_equal() {
        assert_eq!(Point { x: 99, y: 98 }, Point { x: 99, y: 98 });
        assert_ne!(Point { x: 99, y: 98 }, Point { x: 99, y: 99 });
    }
    #[test]
    fn test_point_clone() {
        let point = Point { x: 99, y: 98 };
        assert_eq!(point, point.clone());
    }
    #[test]
    fn test_get_all_moves_single_move() {
        let moves_representation = vec!["R10"];
        let expected_moves = vec![Move {
            start: START_POINT,
            end: Point { x: 10, y: 0 },
            move_type: MoveType::Horizontal,
        }];

        assert_eq!(expected_moves, get_all_moves(&moves_representation));
    }

    #[test]
    fn test_get_all_moves_multiple_moves() {
        let moves_representation = vec!["R10", "U50", "L20", "D100", "R10", "U50"];
        let expected_moves = vec![
            Move {
                start: START_POINT,
                end: Point { x: 10, y: 0 },
                move_type: MoveType::Horizontal,
            },
            Move {
                start: Point { x: 10, y: 0 },
                end: Point { x: 10, y: 50 },
                move_type: MoveType::Vertical,
            },
            Move {
                start: Point { x: 10, y: 50 },
                end: Point { x: -10, y: 50 },
                move_type: MoveType::Horizontal,
            },
            Move {
                start: Point { x: -10, y: 50 },
                end: Point { x: -10, y: -50 },
                move_type: MoveType::Vertical,
            },
            Move {
                start: Point { x: -10, y: -50 },
                end: Point { x: 0, y: -50 },
                move_type: MoveType::Horizontal,
            },
            Move {
                start: Point { x: 0, y: -50 },
                end: START_POINT,
                move_type: MoveType::Vertical,
            },
        ];

        assert_eq!(expected_moves, get_all_moves(&moves_representation));
    }

    #[test]
    fn test_get_move_type() {
        assert_eq!(MoveType::Horizontal, get_move_type("R10"));
        assert_eq!(MoveType::Horizontal, get_move_type("L10"));
        assert_eq!(MoveType::Vertical, get_move_type("U10"));
        assert_eq!(MoveType::Vertical, get_move_type("D10"));
    }

    #[test]
    fn test_get_move_distance() {
        assert_eq!(10, get_moves_distance("R10"));
        assert_eq!(-999, get_moves_distance("L999"));
        assert_eq!(5, get_moves_distance("U5"));
        assert_eq!(-10000, get_moves_distance("D10000"));
    }

    #[test]
    fn test_move_point() {
        let starting_point = Point { x: 99, y: 98 };
        assert_eq!(
            Point { x: 89, y: 98 },
            starting_point.move_point(&MoveType::Horizontal, &-10)
        );
        assert_eq!(
            Point { x: 109, y: 98 },
            starting_point.move_point(&MoveType::Horizontal, &10)
        );
        assert_eq!(
            Point { x: 99, y: 88 },
            starting_point.move_point(&MoveType::Vertical, &-10)
        );
        assert_eq!(
            Point { x: 99, y: 108 },
            starting_point.move_point(&MoveType::Vertical, &10)
        );
    }

    #[test]
    fn test_find_intersections() {
        let first_moves = vec![
            Move {
                start: START_POINT,
                end: Point { x: 10, y: 0 },
                move_type: MoveType::Horizontal,
            },
            Move {
                start: Point { x: 10, y: 0 },
                end: Point { x: 10, y: 50 },
                move_type: MoveType::Vertical,
            },
            Move {
                start: Point { x: 10, y: 50 },
                end: Point { x: 20, y: 50 },
                move_type: MoveType::Horizontal,
            },
            Move {
                start: Point { x: 20, y: 50 },
                end: Point { x: 20, y: 0 },
                move_type: MoveType::Vertical,
            },
        ];

        let second_moves = vec![
            Move{
                start: Point { x: 25, y: 25},
                end: Point { x:-25, y: 25},
                move_type: MoveType::Horizontal
            }
        ];

        let expected_intersections = vec![Point{x: 10, y: 25}, Point{x: 20, y: 25}];

        assert_eq!(expected_intersections, first_moves.find_intersections(&second_moves));
    }

    #[test]
    fn test_find_move_intersection() {
        let first_move = Move {
                start: Point { x: 10, y: 0 },
                end: Point { x: 10, y: 50 },
                move_type: MoveType::Vertical,
        };
        let second_move = Move {
                start: Point { x: 25, y: 25 },
                end: Point { x: -25, y: 25 },
                move_type: MoveType::Horizontal,
        };

        let expected_intersection = Point{x: 10, y: 25};

        assert_eq!(expected_intersection, first_move.find_intersection(&second_move).unwrap());
    }

    #[test]
    fn test_find_move_intersection_none_found() {
        let first_move = Move {
                start: Point { x: 10, y: 0 },
                end: Point { x: 10, y: 50 },
                move_type: MoveType::Vertical,
        };
        let second_move = Move {
                start: Point { x: 25, y: 25 },
                end: Point { x: 25, y: 26 },
                move_type: MoveType::Vertical,
        };

        assert_eq!(None, first_move.find_intersection(&second_move));
    }

    #[test]
    fn test_get_distance() {
         let first_move = Move {
                start: Point { x: 10, y: 0 },
                end: Point { x: 10, y: 50 },
                move_type: MoveType::Vertical,
        };
        assert_eq!(50, first_move.distance());
    }

    #[test]
    fn test_intersects() {
        let first_move = Move {
                start: Point { x: 10, y: 0 },
                end: Point { x: 10, y: 50 },
                move_type: MoveType::Vertical,
        };
        let valid_point = Point { x: 10, y: 25 };
        let invalid_point = Point { x: 15, y: 25 };

        assert_eq!(Some(25), first_move.intersects(&valid_point));
        assert_eq!(None, first_move.intersects(&invalid_point));
    }

    #[test]
    fn test_manhattan_distance() {
        let first_point = Point{x: 1, y: 2};
        let second_point = Point{x: 3, y: 4};
        assert_eq!(4, first_point.manhattan_distance(&second_point));
    }

    #[test]
    fn test_manhattan_distance_negative() {
        let first_point = START_POINT;
        let second_point = Point{x: 158, y: -12};
        assert_eq!(170, first_point.manhattan_distance(&second_point));
    }


    #[test]
    fn test_manhattan_first_test_case() {
        let first_line = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let second_line = "U62,R66,U55,R34,D71,R55,D58,R83";

        assert_eq!(159, get_closest_intersection(first_line, second_line));
    }

    #[test]
    fn test_manhattan_second_test_case() {
        let first_line = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let second_line = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        assert_eq!(135, get_closest_intersection(first_line, second_line));
    }

    #[test]
    fn test_manhattan_first_example() {
        let first_line = "R8,U5,L5,D3";
        let second_line = "U7,R6,D4,L4";

        assert_eq!(6, get_closest_intersection(first_line, second_line));
    }

    #[test]
    fn test_signal_delay() {
        let moves = vec!["R8", "U5", "L5", "D3"];
        let moves = get_all_moves(&moves);
        let point = Point{x: 3, y: 3};
        assert_eq!(20, signal_delay(&point, &moves));
    }

    #[test]
    fn test_delay_first_example() {
        let first_line = "R8,U5,L5,D3";
        let second_line = "U7,R6,D4,L4";

        assert_eq!(30, get_minimum_signal_delay_intersection(first_line, second_line));
    }

    #[test]
    fn test_delay_first_test_case() {
        let first_line = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let second_line = "U62,R66,U55,R34,D71,R55,D58,R83";

        assert_eq!(610, get_minimum_signal_delay_intersection(first_line, second_line));
    }

    #[test]
    fn test_delay_second_test_case() {
        let first_line = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let second_line = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        assert_eq!(410, get_minimum_signal_delay_intersection(first_line, second_line));
    }
}
