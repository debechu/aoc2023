use std::io::{self, Read};

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    use std::time::Instant;
    let start = Instant::now();
    let galaxies = parse_galaxies(&data);
    let parse_time = start.elapsed();
    
    let start = Instant::now();
    let mut sum_pair_lengths = 0;
    for i in 0..galaxies.len()-1
    {
        for j in i+1..galaxies.len()
        {
            let diff = galaxies[j] - galaxies[i];
            sum_pair_lengths += (diff.x + diff.y.abs()) as u64;
        }
    }
    let process_time = start.elapsed();

    println!(
        "Sum of the length of the shortest galaxy pairs: {}", sum_pair_lengths
    );
    println!("==== Part 2 elapsed time");
    println!("- Parse time.....: {}ns", parse_time.as_nanos());
    println!("- Process time...: {}ns", process_time.as_nanos());
}

fn parse_galaxies(data: &str) -> Vec<Vec2>
{
    let mut galaxies: Vec<Vec2> = Vec::with_capacity(1000);

    let mut rows = 0;
    let mut columns = 0;
    let mut num_galaxies_in_row = 0;
    let mut iter = data.chars().peekable();
    loop
    {
        while iter.next_if(|&c| c == '.').is_some() { columns += 1; }
        if let Some(c) = iter.peek()
        {
            match c
            {
                '#' => {
                    let mut index = galaxies.len();
                    for i in 0..galaxies.len()
                    {
                        if galaxies[i].x > columns
                        {
                            index = i;
                            break;
                        }
                    }
                    galaxies.insert(index, Vec2 { x: columns, y: rows });

                    iter.next();
                    columns += 1;
                    num_galaxies_in_row += 1;
                },
                '\r' => {
                    iter.next();
                    if iter.peek().unwrap_or(&'\0').to_owned() == '\n'
                    {
                        iter.next();
                    }

                    rows += 1 + if num_galaxies_in_row == 0 { 999999 } else { 0 };
                    num_galaxies_in_row = 0;
                    columns = 0;
                },
                '\n' => {
                    iter.next();
                    rows += 1 + if num_galaxies_in_row == 0 { 999999 } else { 0 };
                    num_galaxies_in_row = 0;
                    columns = 0;
                },
                _ => panic!("Unrecognized token '{}'!", c),
            }
        }

        if iter.peek().is_none() { break; }
    }

    // Expand columns
    {
        let mut offset = 0;
        let mut current_column = 0;
        for (_, galaxy) in galaxies.iter_mut().enumerate()
        {
            if galaxy.x > current_column
            {
                offset += (galaxy.x - current_column - 1) * 999999;
                current_column = galaxy.x;
            }
            galaxy.x += offset;
        }
    }

    galaxies
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vec2
{
    x: i32,
    y: i32,
}

use std::ops;
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

use std::fmt;
impl fmt::Display for Vec2
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Vec2
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(f, "({}, {})", self.x, self.y)
    }
}
