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
    let mut paths = [
        starting_point + RIGHT,
        starting_point + DOWN,
    ];
    let mut previous_positions = [
        starting_point,
        starting_point,
    ];
    let mut num_steps = 1;
    while paths[0] != paths[1]
    {
        for i in 0..paths.len()
        {
            let current_pos = paths[i];
            let move_direction = current_pos - previous_positions[i];
            previous_positions[i] = current_pos;

            let index = usize::from(current_pos);
            match map[index]
            {
                b'|' | b'-' => paths[i] = current_pos + move_direction,
                b'L' => if move_direction.x == 0 {
                    paths[i] += RIGHT;
                } else { paths[i] += UP; },
                b'J' => if move_direction.x == 0 {
                    paths[i] += LEFT;
                } else { paths[i] += UP; },
                b'7' => if move_direction.x == 0 {
                    paths[i] += LEFT;
                } else { paths[i] += DOWN; },
                b'F' => if move_direction.x == 0 {
                    paths[i] += RIGHT;
                } else { paths[i] += DOWN; },
                _ => (),
            }
        }
        num_steps += 1;
    }
    let part1_time = start.elapsed();

    let start = Instant::now();
    let mut draw = vec![b'.'; map.capacity()];
    draw[usize::from(starting_point)] = b'#';

    let mut paths = [
        Vec2 { x: starting_point.x+1, y: starting_point.y},
        Vec2 { x: starting_point.x, y: starting_point.y+1},
    ];
    let mut previous_positions = [
        starting_point,
        starting_point,
    ];
    let mut num_loops = 1;
    loop
    {
        for i in 0..paths.len()
        {
            let current_pos = paths[i];
            let move_direction = current_pos - previous_positions[i];
            previous_positions[i] = current_pos;

            let index = usize::from(current_pos);
            draw[index] = map[index];
            match map[index]
            {
                b'|' | b'-' => paths[i] = current_pos + move_direction,
                b'L' => if move_direction.x == 0 {
                    paths[i].x += 1;
                } else { paths[i].y -= 1; },
                b'J' => if move_direction.x == 0 {
                    paths[i].x -= 1;
                } else { paths[i].y -= 1; },
                b'7' => if move_direction.x == 0 {
                    paths[i].x -= 1;
                } else { paths[i].y += 1; },
                b'F' => if move_direction.x == 0 {
                    paths[i].x += 1;
                } else { paths[i].y += 1; },
                _ => (),
            }
        }

        if previous_positions[0] == previous_positions[1] { break; }
        num_loops += 1;
    }
    num_loops *= 2;

    let mut num_open_tiles = 0;
    for i in 0..MAP_WIDTH
    {
        if draw[i] != b'.' { continue; }
        draw[i] = b'O';
        num_open_tiles += 1;
    }
    for i in (MAP_HEIGHT-1)*MAP_WIDTH..draw.len()
    {
        if draw[i] != b'.' { continue; }
        draw[i] = b'O';
        num_open_tiles += 1;
    }

    let surroundings = [
        LEFT,
        RIGHT,
        UP + LEFT,
        UP,
        UP + RIGHT,
        DOWN + LEFT,
        DOWN,
        DOWN + RIGHT,
    ];
    for y in 1..MAP_HEIGHT-1
    {
        let first = coord_to_index(0, y as i16);
        if draw[first] == b'.'
        {
            draw[first] = b'O';
            num_open_tiles += 1;
        }

        for x in 1..MAP_WIDTH-1
        {
            let index = coord_to_index(x as i16, y as i16);
            if draw[index] != b'.'
            {
                continue;
            }

            let current_pos = Vec2 { x: x as i16, y: y as i16 };
            let mut enclosed = true;
        'check:
            for &surrounding in &surroundings
            {
                if draw[usize::from(current_pos + surrounding)] ==  b'O'
                {
                    for y in (1..=current_pos.y).rev()
                    {
                        let first = coord_to_index(current_pos.x, y);
                        if !matches!(draw[first], b'.' | b'I') { break; }
                        draw[first] = b'O';
                        num_open_tiles += 1;

                        for x in (1..current_pos.x).rev()
                        {
                            let current = coord_to_index(x, y);
                            if !matches!(draw[first], b'.' | b'I') { break; }
                            draw[current] = b'O';
                            num_open_tiles += 1;
                        }
                    }
                    enclosed = false;
                    break 'check;
                }
            }

            if enclosed { draw[index] = b'I'; }
        }

        let last = coord_to_index(MAP_WIDTH as i16 - 1, y as i16);
        if draw[last] == b'.'
        {
            draw[last] = b'O';
            num_open_tiles += 1;
        }
    }
    for y in (1..MAP_HEIGHT-1).rev()
    {
        for x in (1..MAP_WIDTH-1).rev()
        {
            if draw[coord_to_index(x as i16, y as i16)] != b'I'
            {
                continue;
            }

            let current_pos = Vec2 { x: x as i16, y: y as i16 };
        'check:
            for &surrounding in &surroundings
            {
                if draw[usize::from(current_pos + surrounding)] == b'O'
                {
                    for y in current_pos.y..MAP_HEIGHT as i16 - 1
                    {
                        let first = coord_to_index(current_pos.x, y);
                        if draw[first] != b'I' { break; }
                        draw[first] = b'O';
                        num_open_tiles += 1;

                        for x in current_pos.x+1..MAP_WIDTH as i16 - 1
                        {
                            let current = coord_to_index(x, y);
                            if draw[current] != b'I' { break; }
                            draw[current] = b'O';
                            num_open_tiles += 1;
                        }
                    }
                    break 'check;
                }
            }
        }
    }
    let num_enclosed_tiles = MAP_WIDTH*MAP_HEIGHT - num_loops - num_open_tiles;
    let part2_time = start.elapsed();

    // Print out the newly drawn map for satisfaction
    for i in 0..MAP_HEIGHT
    {
        println!(
            "{}",
            unsafe {std::str::from_utf8_unchecked(
                &draw[i*MAP_WIDTH..(i+1)*MAP_WIDTH]
            )}
        );
    }
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
