use std::io::{
    self,
    Read,
};
use std::iter::Peekable;
use std::str::Chars;

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    use std::time::Instant;
    let start = Instant::now();

    let mut total_points = 0u64;
    let mut total_cards = 0u32;
    let mut copies = [1u32; 199];

    let mut iter = data.chars().peekable();
    while let Some(card) = parse_card(&mut iter)
    {
        let mut winning_map = [0u32; 100];
        for n in card.winnings
        {
            winning_map[n as usize] = 1;
        }
        let winning_map = winning_map;

        let mut wins = 0u32;
        for n in card.numbers
        {
            wins += winning_map[n as usize];
        }
        let wins = wins;

        let current_copy = copies[(card.id-1) as usize];
        total_cards += current_copy;

        for i in 0..wins
        {
            copies[(card.id+i) as usize] += current_copy;
        }

        total_points += {
            let mut points = 5u64;
            for _ in 0..wins
            {
                points *= 2;
            }
            points / 10
        };
    }

    let elapsed_time = start.elapsed();

    println!("Total points...: {}", total_points);
    println!("Total cards....: {}", total_cards);
    println!("Elapsed time...: {}ns", elapsed_time.as_nanos());
}
#[derive(Debug)]
struct Card
{
    id: u32,
    winnings: [u32; 10],
    numbers: [u32; 25],
}

fn parse_card(iter: &mut Peekable<Chars>) -> Option<Card>
{
    skip_whitespace(iter);
    if iter.peek().is_some()
    {
        if !match_card(iter)
        {
            panic!("Expected token 'Card'!");
        }
        
        skip_whitespace(iter);
        let mut card = Card{
            id: parse_number(iter).expect("Expected number!"),
            winnings: [0u32; 10],
            numbers: [0u32; 25],
        };

        skip_whitespace(iter);
        if iter.next().unwrap_or('\0') != ':'
        {
            panic!("Expected token ':'!");
        }

        let mut i = 0;
        skip_whitespace(iter);
        while let Some(n) = parse_number(iter)
        {
            card.winnings[i] = n;
            skip_whitespace(iter);
            i += 1;
        }

        if iter.next().unwrap_or('\0') != '|'
        {
            panic!("Expected token '|'!");
        }

        let mut i = 0;
        skip_whitespace(iter);
        while let Some(n) = parse_number(iter)
        {
            card.numbers[i] = n;
            skip_whitespace(iter);
            i += 1;
        }

        if iter.next().unwrap_or('\n') != '\n'
        {
            panic!("Expected token '\\n'!");
        }

        return Some(card);
    }

    None
}

fn match_card(iter: &mut Peekable<Chars>) -> bool
{
    let mut it = iter.clone();
    let mut c = match it.next() {
        Some(c) => c,
        None => return false,
    };
    if c != 'C' { return false; }

    c = match it.next() {
        Some(c) => c,
        None => return false,
    };
    if c != 'a' { return false; }

    c = match it.next() {
        Some(c) => c,
        None => return false,
    };
    if c != 'r' { return false; }

    c = match it.next() {
        Some(c) => c,
        None => return false,
    };
    if c != 'd' { return false; }

    iter.nth(4);
    true
}

fn parse_number(iter: &mut Peekable<Chars>) -> Option<u32>
{
    if let Some(c) = iter.peek()
    {
        if !c.is_ascii_digit() { return None; }
    }

    let mut n = 0;
    while let Some(c) = iter.peek()
    {
        if !c.is_ascii_digit() { break; }
        n = n * 10 + c.to_digit(10).unwrap();
        iter.next();
    }
    Some(n)
}

fn skip_whitespace(iter: &mut Peekable<Chars>)
{
    while iter.next_if(
        |c| matches!(c, ' ' | '\t' | '\r' | '\x0B' | '\x0C')
    ).is_some() {}
}
