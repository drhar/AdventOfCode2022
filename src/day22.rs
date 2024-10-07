// Intentionally did this using Rc/Wk/RefCell to get a better understanding of how they work, meaning a horrible design of self-referential
// structures. I might return to this and remove those bits and store the edge relations in an Outline struct along with the ordering etc.
// instead.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn day22(input_lines: &str) -> (String, String) {
    day22_main(input_lines, 50)
}

pub fn day22_main(input_lines: &str, face_size: usize) -> (String, String) {
    let (map, instruction_str) = input_lines.split_once("\n\n").unwrap();
    let mut instruction_str = instruction_str.chars().peekable();
    let mut instructions = vec![];
    while let Some(c) = instruction_str.peek() {
        if c.is_ascii_uppercase() {
            match instruction_str.next() {
                Some('L') => {
                    instructions.push(Instruction::TurnLeft);
                }
                Some('R') => {
                    instructions.push(Instruction::TurnRight);
                }
                _ => panic!("Invalid instruction"),
            }
        } else {
            let mut number = String::new();
            while let Some(c) = instruction_str.peek() {
                if c.is_ascii_digit() {
                    number.push(instruction_str.next().unwrap());
                } else {
                    break;
                }
            }
            instructions.push(Instruction::Move(number.parse().unwrap()));
        }
    }

    let board = BoardMap::from_str(map);
    let mut device = InputDevice::new(board);
    let answer1 = device.get_password(&instructions);
    let cube = CubeMap::from_str(map, face_size);
    let mut device = InputDevice::new(cube);
    device.print();
    let answer2 = device.get_password(&instructions);
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Copy)]
pub enum Tile {
    Void,
    Open,
    Solid,
}

#[derive(Debug)]
pub enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

pub trait Map {
    fn traverse_void(
        &mut self,
        start: (usize, usize),
        direction: (i32, i32),
    ) -> ((usize, usize), (i32, i32));
    fn get_tile(&self, x: usize, y: usize) -> Tile;
    fn get_map_width(&self) -> usize;
    fn get_map_length(&self) -> usize;
}

pub struct InputDevice<T: Map> {
    map: T,
    marker: (usize, usize),
    direction: (i32, i32),
}
impl<T> InputDevice<T>
where
    T: Map,
{
    pub fn new(map: T) -> Self {
        let mut starter_x = 0;
        while !matches!(map.get_tile(starter_x, 0), Tile::Open) {
            starter_x += 1;
        }

        Self {
            map,
            marker: (starter_x, 0),
            direction: (1, 0),
        }
    }

    pub fn get_password(&mut self, instructions: &Vec<Instruction>) -> usize {
        for instruction in instructions {
            match instruction {
                Instruction::Move(moves) => self.move_marker(*moves),
                Instruction::TurnLeft => self.turn_marker(Instruction::TurnLeft),
                Instruction::TurnRight => self.turn_marker(Instruction::TurnRight),
            }
            // self.print();
        }
        let facing = match self.direction {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => panic!("Invalid direction"),
        };
        self.print();
        1000 * (self.marker.1 + 1) + 4 * (self.marker.0 + 1) + facing
    }

    fn move_marker(&mut self, moves: usize) {
        println!("Moving {:?}", moves);
        let (mut dx, mut dy) = self.direction;
        let (mut x, mut y) = self.marker;
        for _ in 0..moves {
            let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);
            let ((new_x, new_y), (new_dx, new_dy)) = if new_x < 0
                || new_x == self.map.get_map_width() as i32
                || new_y < 0
                || new_y == self.map.get_map_length() as i32
                || matches!(
                    self.map.get_tile(new_x as usize, new_y as usize),
                    Tile::Void
                ) {
                self.map.traverse_void((x, y), self.direction)
            } else {
                ((new_x as usize, new_y as usize), (dx, dy))
            };
            match self.map.get_tile(new_x, new_y) {
                Tile::Open => {
                    (x, y) = (new_x, new_y);
                    (dx, dy) = (new_dx, new_dy);
                }
                Tile::Solid => break,
                Tile::Void => panic!("Invalid move"),
            }
        }
        self.marker = (x, y);
        self.direction = (dx, dy);
    }

    fn turn_marker(&mut self, turn: Instruction) {
        println!("Turning {:?}", turn);
        let (dx, dy) = self.direction;
        let (dx, dy) = match turn {
            Instruction::TurnLeft => (dy, -dx),
            Instruction::TurnRight => (-dy, dx),
            Instruction::Move(_) => panic!("Invalid turn"),
        };
        self.direction = (dx, dy);
    }

    pub fn print(&self) {
        for y in 0..self.map.get_map_length() {
            for x in 0..self.map.get_map_width() {
                if (x, y) == self.marker {
                    match self.direction {
                        (1, 0) => print!(">"),
                        (0, -1) => print!("^"),
                        (-1, 0) => print!("<"),
                        (0, 1) => print!("v"),
                        _ => panic!("Invalid direction"),
                    }
                } else {
                    match self.map.get_tile(x, y) {
                        Tile::Void => print!(" "),
                        Tile::Open => print!("."),
                        Tile::Solid => print!("#"),
                    }
                }
            }
            println!();
        }
    }
}

pub struct BoardMap {
    tiles: Vec<Vec<Tile>>,
}

impl BoardMap {
    pub fn from_str(input: &str) -> Self {
        let map_width = input.lines().map(|line| line.len()).max().unwrap();
        let tiles = input
            .lines()
            // .rev()
            .map(|line| {
                let mut tiles = line
                    .chars()
                    .map(|c| match c {
                        '.' => Tile::Open,
                        '#' => Tile::Solid,
                        _ => Tile::Void,
                    })
                    .collect::<Vec<Tile>>();
                let padding = map_width - tiles.len();
                tiles.extend(std::iter::repeat(Tile::Void).take(padding));
                tiles
            })
            .collect::<Vec<Vec<Tile>>>();
        Self { tiles }
    }

    fn step_marker(&self, start: (usize, usize), direction: (i32, i32)) -> (usize, usize) {
        let (dx, dy) = direction;
        let (x, y) = start;
        let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);

        let map_len = self.tiles.len();
        let map_width = self.tiles[0].len();

        let new_y = if new_y < 0 {
            map_len - 1
        } else if new_y == map_len as i32 {
            0
        } else {
            new_y as usize
        };

        let new_x = if new_x < 0 {
            map_width - 1
        } else if new_x == map_width as i32 {
            0
        } else {
            new_x as usize
        };
        (new_x, new_y)
    }
}

impl Map for BoardMap {
    fn get_map_length(&self) -> usize {
        self.tiles.len()
    }

    fn get_map_width(&self) -> usize {
        self.tiles[0].len()
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn traverse_void(
        &mut self,
        start: (usize, usize),
        direction: (i32, i32),
    ) -> ((usize, usize), (i32, i32)) {
        let (mut new_x, mut new_y) = self.step_marker(start, direction);
        while let Tile::Void = self.tiles[new_y][new_x] {
            (new_x, new_y) = self.step_marker((new_x, new_y), direction);
        }
        ((new_x, new_y), direction)
    }
}

#[derive(Debug)]
pub struct Edge {
    start: (usize, usize),
    end: (usize, usize),
    direction: (i32, i32),
    connected_edge: Option<Weak<RefCell<Edge>>>,
}

impl Edge {
    pub fn new(start: (usize, usize), end: (usize, usize), direction: (i32, i32)) -> Self {
        Self {
            start,
            end,
            direction,
            connected_edge: None,
        }
    }

    pub fn as_rc_cell(
        start: (usize, usize),
        end: (usize, usize),
        direction: (i32, i32),
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(start, end, direction)))
    }

    pub fn cross_edge(&self, marker: (usize, usize)) -> ((usize, usize), (i32, i32)) {
        // Edges connect start-to-end
        let distance_along_edge = ((marker.0 as i32 - self.start.0 as i32)
            + (marker.1 as i32 - self.start.1 as i32))
            .abs();
        let edge_ref = self.connected_edge.as_ref().unwrap().upgrade().unwrap();
        let connected_edge = edge_ref.borrow();
        (
            (
                (connected_edge.end.0 as i32 + (connected_edge.direction.1 * distance_along_edge))
                    as usize,
                (connected_edge.end.1 as i32 - (connected_edge.direction.0 * distance_along_edge))
                    as usize,
            ),
            (-connected_edge.direction.0, -connected_edge.direction.1),
        )
    }

    pub fn connect(&mut self, other: Weak<RefCell<Edge>>) {
        self.connected_edge = Some(other);
    }

    pub fn is_crossable(&self, position: (usize, usize), direction: (i32, i32)) -> bool {
        if direction != self.direction {
            return false;
        }
        let (x, y) = position;
        let lower_x = self.start.0.min(self.end.0);
        let upper_x = self.start.0.max(self.end.0);
        let lower_y = self.start.1.min(self.end.1);
        let upper_y = self.start.1.max(self.end.1);
        if x < lower_x || x > upper_x || y < lower_y || y > upper_y {
            return false;
        }
        true
    }
}

pub struct CubeMap {
    tiles: Vec<Vec<Tile>>,
    outline: Vec<Rc<RefCell<Edge>>>,
}

impl CubeMap {
    pub fn from_str(input: &str, face_size: usize) -> Self {
        let map_width = input.lines().map(|line| line.len()).max().unwrap();
        let tiles = input
            .lines()
            .map(|line| {
                let mut tiles = line
                    .chars()
                    .map(|c| match c {
                        '.' => Tile::Open,
                        '#' => Tile::Solid,
                        _ => Tile::Void,
                    })
                    .collect::<Vec<Tile>>();
                let padding = map_width - tiles.len();
                tiles.extend(std::iter::repeat(Tile::Void).take(padding));
                tiles
            })
            .collect::<Vec<Vec<Tile>>>();

        // We're going to move clockwise around the edge of our net to find all the edges.
        let mut outline = vec![];
        let mut y = 0;
        let mut x = 0;
        // Find our first edge
        while let Tile::Void = tiles[y][x] {
            y += face_size;
            if y == tiles.len() {
                y = 0;
                x += face_size;
            }
        }

        let start_x = x;
        let start_y = y;

        // First work out the type of our first edge. The way we iterate, it can only be the top or left of a face
        if y == 0 || matches!(tiles[y - 1][x], Tile::Void) {
            x += face_size - 1;
            outline.push(Edge::as_rc_cell((start_x, start_y), (x, y), (0, -1)))
        } else if x == 0 || matches!(tiles[y][x - 1], Tile::Void) {
            y += face_size - 1;
            outline.push(Edge::as_rc_cell((x, y), (start_x, start_y), (-1, 0)))
        } else {
            panic!("Invalid edge")
        }
        println!("{x}, {start_x}, {y}, {start_y}");

        let mut count = 0;
        // Move clockwise around the edges of our net until we're back to the start
        while (x != start_x || y != start_y) && count < 30 {
            count += 1;
            let (dx, dy);
            let (prev_end_x, prev_end_y);
            {
                // Continue in the same direction as previous edge by one tile to see what's there
                let previous_edge = outline.last().unwrap().borrow();
                (dx, dy) = previous_edge.direction;
                (prev_end_x, prev_end_y) = previous_edge.end;
            }
            let new_edge;

            let next_x = prev_end_x as i32 - dy;
            let next_y = prev_end_y as i32 + dx;

            if next_x == map_width as i32
                || next_x < 0
                || next_y == tiles.len() as i32
                || next_y < 0
                || matches!(tiles[next_y as usize][next_x as usize], Tile::Void)
            {
                // We're turning right (we know this as we can't connect only by corners). This will always be part of the same face.
                new_edge = Edge::as_rc_cell(
                    (prev_end_x, prev_end_y),
                    (
                        (prev_end_x as i32 - dx * (face_size as i32 - 1)) as usize,
                        (prev_end_y as i32 - dy * (face_size as i32 - 1)) as usize,
                    ),
                    (-dy, dx),
                );
            } else if next_x + dx == map_width as i32
                || next_x + dx < 0
                || next_y + dy == tiles.len() as i32
                || next_y + dy < 0
                || matches!(
                    tiles[(next_y + dy) as usize][(next_x + dx) as usize],
                    Tile::Void
                )
            {
                // We have another of the same type of edge
                let start = (next_x as usize, next_y as usize);
                new_edge = Edge::as_rc_cell(
                    start,
                    (
                        (next_x - dy * (face_size as i32 - 1)) as usize,
                        (next_y + dx * (face_size as i32 - 1)) as usize,
                    ),
                    (dx, dy),
                );
            } else {
                // We're turning left. Edges with a left turn will always connect when we fold the net
                let next_start = (next_x + dx, next_y + dy);
                new_edge = Edge::as_rc_cell(
                    (next_start.0 as usize, next_start.1 as usize),
                    (
                        (next_start.0 + dx * (face_size as i32 - 1)) as usize,
                        (next_start.1 + dy * (face_size as i32 - 1)) as usize,
                    ),
                    (dy, -dx),
                );
            }
            x = new_edge.borrow().end.0;
            y = new_edge.borrow().end.1;
            outline.push(new_edge);
        }

        outline = Self::fold(outline);

        if outline
            .iter()
            .any(|edge| edge.borrow().connected_edge.is_none())
        {
            panic!("Not all edges connected");
        }

        Self { tiles, outline }
    }

    fn fold(outline: Vec<Rc<RefCell<Edge>>>) -> Vec<Rc<RefCell<Edge>>> {
        // Edges in a cubic net can only connect if they're:
        //  - adjacent edges where their directions are a left turn (going clockwise)
        //  - edges separated by 2 edges where they have the same direction
        //  - edges separated by 4 edges where their directions are a right turn
        //  - edges separated by 6 edges where they have the opposite direction (e.g. right/left, top/bottom)
        // even separated edges can't connect. 3 separated edges must be the same direction.
        for edges_to_skip in [0, 2, 4, 6].iter() {
            for i in 0..outline.len() {
                let edge = &outline[i];
                if edge.borrow().connected_edge.is_some() {
                    continue;
                }
                let next_edge = &outline[(i + *edges_to_skip + 1) % outline.len()];
                let direction = edge.borrow().direction;
                let required_direction = match edges_to_skip {
                    0 => (direction.1, -direction.0),
                    2 => direction,
                    4 => (-direction.1, direction.0),
                    6 => (-direction.0, -direction.1),
                    _ => panic!("Invalid edges to skip"),
                };
                if next_edge.borrow().direction == required_direction
                    && next_edge.borrow().connected_edge.is_none()
                {
                    println!("Connecting {:?} to {:?}", edge.borrow(), next_edge.borrow());
                    edge.borrow_mut().connect(Rc::downgrade(next_edge));
                    next_edge.borrow_mut().connect(Rc::downgrade(edge));
                }
            }
        }
        outline
    }
}

impl Map for CubeMap {
    fn get_map_length(&self) -> usize {
        self.tiles.len()
    }

    fn get_map_width(&self) -> usize {
        self.tiles[0].len()
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn traverse_void(
        &mut self,
        start: (usize, usize),
        direction: (i32, i32),
    ) -> ((usize, usize), (i32, i32)) {
        let edge = self
            .outline
            .iter()
            .find(|edge| edge.borrow().is_crossable(start, direction))
            .unwrap();
        edge.borrow().cross_edge(start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day22_part1_case1() {
        assert_eq!(
            day22_main(
                "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
                4
            )
            .0,
            "6032".to_string()
        )
    }

    #[test]
    fn check_day22_part2_case1() {
        assert_eq!(
            day22_main(
                "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
                4
            )
            .1,
            "5031".to_string()
        )
    }

    #[test]
    fn check_day22_both_case1() {
        assert_eq!(
            day22_main(
                "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
                4
            ),
            ("6032".to_string(), "5031".to_string())
        )
    }
}
