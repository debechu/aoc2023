use std::io;
use std::str::Chars;
use std::iter::{
    Peekable,
    Rev,
};

fn main()
{
    let mut sum = 0u32;
    for line in io::stdin().lines()
    {
        let line = line.unwrap();

        let mut iter = line.chars().peekable();
        let start = loop {
            if let Some(c) = iter.peek()
            {
                if c.is_ascii_digit()
                {
                    break c.to_digit(10).unwrap() as u32;
                }
                else if let Some(digit) = word_to_digit(&mut iter)
                {
                    break digit;
                }
            }
        };

        let mut iter = line.chars().rev().peekable();
        let end = loop {
            if let Some(c) = iter.peek()
            {
                if c.is_ascii_digit()
                {
                    break c.to_digit(10).unwrap() as u32;
                }
                else if let Some(digit) = word_to_digit_rev(&mut iter)
                {
                    break digit;
                }
            }
        };

        // println!("{}{}: {}", start, end, line);

        sum += start * 10 + end;
    }

    println!("Calibration value: {}", sum);
}

fn word_to_digit(iter: &mut Peekable<Chars>) -> Option<u32>
{
    if let Some(c) = iter.next()
    {
        let mut it = iter.clone();

        match c
        {
            'o' => if let Some(c) = it.peek() {
                match c
                {
                    'n' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'e' => {
                                    iter.nth(2);
                                    return Some(1);
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            't' => if let Some(c) = it.peek() {
                match c
                {
                    'w' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'o' => {
                                    iter.nth(2);
                                    return Some(2);
                                },
                                _ => ()
                            }
                        }
                    },
                    'h' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'r' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                it.next();
                                                if let Some(c) = it.peek()
                                                {
                                                    match c
                                                    {
                                                        'e' => {
                                                            iter.nth(4);
                                                            return Some(3);
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'f' => if let Some(c) = it.peek() {
                match c
                {
                    'o' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'u' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'r' => {
                                                iter.nth(3);
                                                return Some(4);
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    'i' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'v' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                iter.nth(3);
                                                return Some(5);
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            's' => if let Some(c) = it.peek() {
                match c
                {
                    'i' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'x' => {
                                    iter.nth(2);
                                    return Some(6);
                                },
                                _ => ()
                            }
                        }
                    },
                    'e' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'v' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                it.next();
                                                if let Some(c) = it.peek()
                                                {
                                                    match c
                                                    {
                                                        'n' => {
                                                            iter.nth(4);
                                                            return Some(7);
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'e' => if let Some(c) = it.peek() {
                match c
                {
                    'i' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'g' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'h' => {
                                                it.next();
                                                if let Some(c) = it.peek()
                                                {
                                                    match c
                                                    {
                                                        't' => {
                                                            iter.nth(4);
                                                            return Some(8);
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'n' => if let Some(c) = it.peek() {
                match c
                {
                    'i' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'n' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                iter.nth(3);
                                                return Some(9);
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    return None
}

fn word_to_digit_rev(iter: &mut Peekable<Rev<Chars>>) -> Option<u32>
{
    if let Some(c) = iter.next()
    {
        let mut it = iter.clone();
        
        match c
        {
            'e' => if let Some(c) = it.peek() {
                match c
                {
                    'n' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'o' => {
                                    iter.nth(2);
                                    return Some(1);
                                },
                                'i' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'n' => {
                                                iter.nth(3);
                                                return Some(9);
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    'e' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'r' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'h' => {
                                                it.next();
                                                if let Some(c) = it.peek()
                                                {
                                                    match c
                                                    {
                                                        't' => {
                                                            iter.nth(4);
                                                            return Some(3);
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    'v' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'i' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'f' => {
                                                iter.nth(3);
                                                return Some(5);
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'o' => if let Some(c) = it.peek() {
                match c
                {
                    'w' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                't' => {
                                    iter.nth(2);
                                    return Some(2);
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'r' => if let Some(c) = it.peek() {
                match c
                {
                    'u' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'o' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'f' => {
                                                iter.nth(3);
                                                return Some(4);
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'x' => if let Some(c) = it.peek() {
                match c
                {
                    'i' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                's' => {
                                    iter.nth(2);
                                    return Some(6);
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            'n' => if let Some(c) = it.peek() {
                match c
                {
                    'e' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'v' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                it.next();
                                                if let Some(c) = it.peek()
                                                {
                                                    match c
                                                    {
                                                        's' => {
                                                            iter.nth(4);
                                                            return Some(7);
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            't' => if let Some(c) = it.peek() {
                match c
                {
                    'h' => {
                        it.next();
                        if let Some(c) = it.peek()
                        {
                            match c
                            {
                                'g' => {
                                    it.next();
                                    if let Some(c) = it.peek()
                                    {
                                        match c
                                        {
                                            'i' => {
                                                it.next();
                                                if let Some(c) = it.peek()
                                                {
                                                    match c
                                                    {
                                                        'e' => {
                                                            iter.nth(4);
                                                            return Some(8);
                                                        },
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            _ => ()
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    return None
}
