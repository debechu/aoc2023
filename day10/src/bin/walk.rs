use std::io::{self, Read};

const MAP_WIDTH: usize = 140;
const MAP_HEIGHT: usize = 140;

const UP: Vec2 = Vec2{ x: 0, y: -1 };
const DOWN: Vec2 = Vec2{ x: 0, y: 1 };
const LEFT: Vec2 = Vec2{ x: -1, y: 0 };
const RIGHT: Vec2 = Vec2{ x: 1, y: 0 };

fn main()
{
    let data: Vec<u8> = io::stdin().bytes().map(|b| b.unwrap()).collect();

    use std::time::Instant;
    let start = Instant::now();
    let mut starting_point = 0;
    let mut map = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
    data.into_iter()
        .filter(|b| !b.is_ascii_whitespace())
        .enumerate()
        .for_each(|(i, b)| {
            if b == b'S' { starting_point = i; }
            map.push(b);
        });
    let starting_point = Vec2 {
        x: (starting_point % MAP_WIDTH) as i16,
        y: (starting_point / MAP_WIDTH) as i16,
    };
    let parse_time = start.elapsed();

    let start = Instant::now();
    let mut current_pos = starting_point + RIGHT;
    let mut prev_pos = starting_point;
    let mut num_steps = 1;
    while current_pos != starting_point
    {
        let move_direction = current_pos - prev_pos;
        prev_pos = current_pos;

        let index = usize::from(current_pos);
        match map[index]
        {
            b'|' | b'-' => current_pos = current_pos + move_direction,
            b'L' => if move_direction.x == 0 {
                current_pos += RIGHT;
            } else { current_pos += UP; },
            b'J' => if move_direction.x == 0 {
                current_pos += LEFT;
            } else { current_pos += UP; },
            b'7' => if move_direction.x == 0 {
                current_pos += LEFT;
            } else { current_pos += DOWN; },
            b'F' => if move_direction.x == 0 {
                current_pos += RIGHT;
            } else { current_pos += DOWN; },
            _ => (),
        }
        num_steps += 1;
    }
    num_steps >>= 1;
    let part1_time = start.elapsed();

    let start = Instant::now();
    let mut draw = vec![b'.'; map.capacity()];
    draw[usize::from(starting_point)] = b'F';

    let mut current_pos = starting_point + RIGHT;
    let mut prev_pos = starting_point;
    while current_pos != starting_point
    {
        let move_direction = current_pos - prev_pos;
        prev_pos = current_pos;

        let index = usize::from(current_pos);
        let current = map[index];
        match current
        {
            b'|' | b'-' => current_pos = current_pos + move_direction,
            b'L' => if move_direction.x == 0 {
                current_pos += RIGHT;
            } else { current_pos += UP; },
            b'J' => if move_direction.x == 0 {
                current_pos += LEFT;
            } else { current_pos += UP; },
            b'7' => if move_direction.x == 0 {
                current_pos += LEFT;
            } else { current_pos += DOWN; },
            b'F' => if move_direction.x == 0 {
                current_pos += RIGHT;
            } else { current_pos += DOWN; },
            _ => (),
        }
        draw[index] = current;
    }

    let mut num_enclosed_tiles = 0;
    let mut last_pipe = b'.';
    let mut enclosed = false;
    let mut prev_line = 0;
    for i in 0..MAP_WIDTH*MAP_HEIGHT
    {
        let line = i / MAP_WIDTH; 
        if line != prev_line
        {
            prev_line = line;
            last_pipe = b'.';
            enclosed = false;
        }

        let current = draw[i];
        match current
        {
            b'.' => (),
            b'J' => {
                if last_pipe != b'F'
                {
                    enclosed = !enclosed;
                }
                last_pipe = current;
                continue;
            },
            b'7' => {
                if last_pipe != b'L'
                {
                    enclosed = !enclosed;
                }
                last_pipe = current;
                continue;
            },
            b'|' | b'F' | b'L' => {
                enclosed = !enclosed;
                last_pipe = current;
                continue;
            },
            _ => continue,
        }

        if enclosed
        {
            num_enclosed_tiles += 1;
        }
    }
    let part2_time = start.elapsed();

    println!("Numbers of steps until farthest position: {}", num_steps);
    println!("Numbers of enclosed tiles: {}", num_enclosed_tiles);
    println!("==== Elapsed time");
    println!("- Parse time....: {}ns", parse_time.as_nanos());
    println!("- Part 1 time...: {}ns", part1_time.as_nanos());
    println!("- Part 2 time...: {}ns", part2_time.as_nanos());
}

#[inline(always)]
fn coord_to_index(x: i16, y: i16) -> usize
{
    y as usize * MAP_WIDTH + x as usize
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vec2
{
    x: i16,
    y: i16,
}

use std::ops;
impl ops::Add for Vec2
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vec2
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::Sub for Vec2
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Vec2> for usize
{
    #[inline(always)]
    fn from(v: Vec2) -> Self
    {
        coord_to_index(v.x, v.y)
    }
}

use std::fmt;
impl fmt::Display for Vec2
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(f, "({} | {})", self.x, self.y)
    }
}
