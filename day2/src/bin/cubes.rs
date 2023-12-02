use std::io::{
    self,
    Read,
};
use std::iter::Peekable;
use std::str::CharIndices;

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let mut sum_id = 0u32;
    let mut sum_set_power = 0u32;
    let mut iter = data.char_indices().peekable();

    use std::time::Instant;
    
    let start = Instant::now();
    while let Some(game) = parse_game(&data, &mut iter)
    {
        if game.within_constraint(12, 13, 14)
        {
            sum_id += game.id;
        }

        let set_min = game.minimum_set();
        sum_set_power += set_min.red * set_min.green * set_min.blue;
    }

    println!("Elapsed time: {}ns", start.elapsed().as_nanos());

    println!("Game ID sum: {}", sum_id);
    println!("Game set power sum: {}", sum_set_power);
}

#[derive(Clone, Copy, Debug)]
struct GameSet
{
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_SET_COUNT: usize = 10;

#[derive(Clone, Copy, Debug)]
struct Game
{
    id: u32,
    set_count: usize,
    sets: [GameSet; MAX_SET_COUNT],
}

impl Game
{
    fn within_constraint(&self, red: u32, green: u32, blue: u32) -> bool
    {
        for i in 0..self.set_count
        {
            let set = &self.sets[i];
            if set.red > red || set.green > green || set.blue > blue
            {
                return false;
            }
        }
        return true;
    }

    fn minimum_set(&self) -> GameSet
    {
        let mut set_min = GameSet { red: 0, green: 0, blue: 0 };
        for i in 0..self.set_count
        {
            let set = &self.sets[i];
            set_min.red = set_min.red.max(set.red);
            set_min.green = set_min.green.max(set.green);
            set_min.blue = set_min.blue.max(set.blue);
        }
        return set_min;
    }
}

fn parse_game(
    data: &String,
    iter: &mut Peekable<CharIndices>
) -> Option<Game>
{
    skip_whitespace(iter);
    if let Some(word) = scan_word(data, iter)
    {
        if word.chars().ne("Game".chars())
        {
            panic!("Expected token 'Game'!");
        }
    }
    else { return None; }

    skip_whitespace(iter);
    let mut game = Game {
        id: scan_integer(iter).expect("Expected Game ID!"),
        set_count: 0,
        sets: [ GameSet { red: 0, green: 0, blue: 0 }; MAX_SET_COUNT]
    };

    skip_whitespace(iter);
    if iter.next().expect("Expected token ':'!").1 != ':'
    {
        panic!("Expected token ':'!");
    }
    
    loop
    {
        skip_whitespace(iter);
        if let Some((_, c)) = iter.peek()
        {
            let c = c.to_owned();
            if c == '\n'
            {
                iter.next();
                break;
            }

            let set = parse_set(data, iter);
            game.sets[game.set_count] = set;
            game.set_count += 1;
        }
        else { break; }
    }

    return Some(game);
}

fn parse_set(
    data: &String,
    iter: &mut Peekable<CharIndices>
) -> GameSet
{
    let mut set = GameSet { red: 0, green: 0, blue: 0 };

    while let Some(_) = iter.peek()
    {
        skip_whitespace(iter);
        if let Some(value) = scan_integer(iter)
        {
            skip_whitespace(iter);
            let word = scan_word(data, iter)
                .expect("Expected token 'red', 'green' or 'blue'!");
            if word.chars().eq("red".chars())
            {
                set.red = value;
            }
            else if word.chars().eq("green".chars())
            {
                set.green = value;
            }
            else if word.chars().eq("blue".chars())
            {
                set.blue = value;
            }
            else
            {
                panic!("Expected token 'red', 'green' or 'blue'!");
            }
        }

        skip_whitespace(iter);
        if let Some((_, c)) = iter.peek()
        {
            let c = c.to_owned();
            match c
            {
                ',' => { iter.next(); },
                ';' => { iter.next(); break; },
                '\n' => break,
                _ => panic!("Expected token ',' or ';'!")
            }
        }
        else { break; }
    };

    return set;
}

fn scan_word<'a>(
    data: &'a String,
    iter: &mut Peekable<CharIndices>
) -> Option<&'a str>
{

    if let Some(_) = iter.peek()
    {
        let (start, _) = iter.next().unwrap();
        let end = {
            while let Some((_, c)) = iter.peek()
            {
                let c = c.to_owned();
                if c.is_ascii_whitespace() { break; }
                if c == ',' || c == ';' { break; }
                iter.next();
            }

            if let Some((i, _)) = iter.peek() { i.to_owned() }
            else { data.len() }
        };

        // println!("Word: '{}'", &data[start..end]);

        return Some(&data[start..end]);
    }

    return None;
}

fn scan_integer(iter: &mut Peekable<CharIndices>) -> Option<u32>
{
    if let Some((_, c)) = iter.peek()
    {
        if !c.is_ascii_digit() { return None; }

        let mut n = iter.next().unwrap().1.to_digit(10).unwrap();
        while let Some((_, c)) = iter.peek()
        {
            if !c.is_ascii_digit() { break; }
            n = n * 10 + c.to_digit(10).unwrap();
            iter.next();
        }

        // println!("Integer: {}", n);

        return Some(n);
    }

    return None;
}

fn skip_whitespace(iter: &mut Peekable<CharIndices>)
{
    while let Some(_) = iter.next_if(|(_, c)| is_whitespace(c.to_owned())) {}
}

fn is_whitespace(c: char) -> bool
{
    match c
    {
        ' ' | '\t' | '\r' | '\x0B' | '\x0C' => true,
        _ => false
    }
}
