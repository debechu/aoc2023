use std::io::{
    self,
    Read,
};
use std::iter::{
    Enumerate,
    Peekable,
};
use std::ops::Sub;
use std::str::Chars;

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    use std::time::Instant;
    let start = Instant::now();
    let mut schematic = parse_engine_schematic(&data);
    let parse_time = start.elapsed().as_nanos();

    let start = Instant::now();
    let mut sum = 0u32;
    let mut checked = 0usize;
    for symbol in &schematic.symbols
    {
        let start = checked;
        for i in start..schematic.numbers.len()
        {
            let number = &schematic.numbers[i];
            let diff = symbol.pos - number.pos;
            
            if (diff.x >= -1 && diff.x <= number.len) && diff.y.abs() <= 1
            {
                sum += number.value;
                schematic.numbers.swap(checked, i);
                checked += 1;
            }
        }

        if checked == schematic.numbers.len()
        {
            break;
        }
    }
    let part1_time = start.elapsed().as_nanos();

    let start = Instant::now();
    let mut sum_gear_ratio = 0u32;
    let mut checked = 0usize;
    for gear in &schematic.gears
    {
        let start = checked;
        let mut gear_ratio = 1u32;
        for i in start..schematic.numbers.len()
        {
            let number = &schematic.numbers[i];
            let diff = gear.pos - number.pos;
            
            if (diff.x >= -1 && diff.x <= number.len) && diff.y.abs() <= 1
            {
                gear_ratio *= number.value;
                schematic.numbers.swap(checked, i);
                checked += 1;
            }
        }

        if checked - start == 2
        {
            sum_gear_ratio += gear_ratio;
        }
    }
    let part2_time = start.elapsed().as_nanos();

    println!("Sum of all numbers adjacent to a symbol: {}", sum);
    println!("Sum of all gear ratios: {}", sum_gear_ratio);
    println!("==== Elapsed time");
    println!("- Parse : {}ns", parse_time);
    println!("- Part 1: {}ns", part1_time);
    println!("- Part 2: {}ns", part2_time);
}

#[derive(Clone, Copy, Debug)]
struct Vec2
{
    x: i32,
    y: i32,
}

impl Sub for Vec2
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SchematicNumber
{
    pos: Vec2,
    len: i32,
    value: u32,
}

#[derive(Clone, Copy, Debug)]
struct SchematicSymbol
{
    pos: Vec2,
}

const MIN_CAPACITY: usize = 1600;

#[derive(Debug)]
struct EngineSchematic
{
    numbers: Vec<SchematicNumber>,
    symbols: Vec<SchematicSymbol>,
    gears: Vec<SchematicSymbol>,
}

fn parse_engine_schematic(schematic: &str) -> EngineSchematic
{
    let mut numbers: Vec<SchematicNumber> = Vec::with_capacity(MIN_CAPACITY);
    let mut symbols: Vec<SchematicSymbol> = Vec::with_capacity(MIN_CAPACITY);
    let mut gears: Vec<SchematicSymbol> = Vec::with_capacity(MIN_CAPACITY);
    let mut column = 0u32;
    let mut line = 0u32;
    let mut line_width: Option<u32> = None;

    let mut iter = schematic.chars().enumerate().peekable();
    while iter.peek().is_some()
    {
        skip_dots(&mut iter, &mut column);
        if let Some((_, c)) = iter.peek()
        {
            if c.is_ascii_digit()
            {
                let start = column;
                let value = parse_number(&mut iter, &mut column);
                let end = column;
                numbers.push(SchematicNumber {
                    pos: Vec2 { x: start as i32, y: line as i32 },
                    len: (end - start) as i32,
                    value,
                });
            }
            else if *c == '\n'
            {
                if let Some(width) = line_width
                {
                    if width != column
                    {
                        panic!("Line width is not consistent!");
                    }
                }
                else
                {
                    line_width = Some(column);
                }

                line += 1;
                column = 0;
                iter.next();
            }
            else
            {
                let sym = SchematicSymbol {
                    pos: Vec2 { x: column as i32, y: line as i32 }
                };
                symbols.push(sym);

                if *c == '*'
                {
                    gears.push(sym);
                }

                next(&mut iter, &mut column);
            }
        }
    }

    EngineSchematic {
        numbers,
        symbols,
        gears,
    }
}

fn parse_number(iter: &mut Peekable<Enumerate<Chars>>, column: &mut u32) -> u32
{
    let mut n = 0u32;
    while let Some((_, c)) = iter.peek()
    {
        if !c.is_ascii_digit() { break; }
        n = n * 10 + c.to_digit(10).unwrap();
        next(iter, column);
    }
    n
}

fn skip_dots(iter: &mut Peekable<Enumerate<Chars>>, column: &mut u32)
{
    while let Some((_, c)) = iter.peek()
    {
        if !matches!(c, '.') { break; }
        next(iter, column);
    }
}

fn next(iter: &mut Peekable<Enumerate<Chars>>, column: &mut u32) -> Option<(usize, char)>
{
    match iter.next()
    {
        Some(v) => {
            *column += 1;
            Some(v)
        },
        None => None,
    }
}
