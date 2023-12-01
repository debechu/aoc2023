use std::io;
use std::str::Chars;
use std::iter::Peekable;

fn main()
{
    let mut sum = 0u32;
    for line in io::stdin().lines()
    {
        let line = line.unwrap();
        let mut iter = line.chars().peekable();

        let mut start = 0u32;
        while let Some(c) = iter.peek()
        {
            if c.is_ascii_digit()
            {
                start = c.to_digit(10).unwrap() as u32;
                iter.next();
                break;
            }
            else if let Some(digit) = word_to_digit(&mut iter)
            {
                start = digit;
                break;
            }
        }

        let mut end = start;
        while let Some(c) = iter.next()
        {
            if c.is_ascii_digit()
            {
                end = c.to_digit(10).unwrap() as u32;
            }
            else if let Some(digit) = word_to_digit(&mut iter)
            {
                end = digit;
                break;
            }
        }

        sum += start * 10 + end;
    }

    println!("Calibration value: {}", sum);
}

fn word_to_digit(iter: &mut Peekable<Chars>) -> Option<u32>
{
    if let Some(c) = iter.next()
    {
        match c
        {
            'o' => if let Some(c) = iter.peek() {
                match c
                {
                    'n' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'e' => {
                                    iter.next();
                                    return Some(1);
                                },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
            },
            't' => if let Some(c) = iter.peek() {
                match c
                {
                    'w' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'o' => {
                                    iter.next();
                                    return Some(2);
                                },
                                _ => ()
                            }
                        }
                    },
                    'h' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'r' => {
                                    iter.next();
                                    if let Some(c) = iter.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                iter.next();
                                                if let Some(c) = iter.peek()
                                                {
                                                    match c
                                                    {
                                                        'e' => {
                                                            iter.next();
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
            'f' => if let Some(c) = iter.peek() {
                match c
                {
                    'o' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'u' => {
                                    iter.next();
                                    if let Some(c) = iter.peek()
                                    {
                                        match c
                                        {
                                            'r' => {
                                                iter.next();
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
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'v' => {
                                    iter.next();
                                    if let Some(c) = iter.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                iter.next();
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
            's' => if let Some(c) = iter.peek() {
                match c
                {
                    'i' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'x' => {
                                    iter.next();
                                    return Some(6);
                                },
                                _ => ()
                            }
                        }
                    },
                    'e' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'v' => {
                                    iter.next();
                                    if let Some(c) = iter.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                iter.next();
                                                if let Some(c) = iter.peek()
                                                {
                                                    match c
                                                    {
                                                        'n' => {
                                                            iter.next();
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
            'e' => if let Some(c) = iter.peek() {
                match c
                {
                    'i' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'g' => {
                                    iter.next();
                                    if let Some(c) = iter.peek()
                                    {
                                        match c
                                        {
                                            'h' => {
                                                iter.next();
                                                if let Some(c) = iter.peek()
                                                {
                                                    match c
                                                    {
                                                        't' => {
                                                            iter.next();
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
            'n' => if let Some(c) = iter.peek() {
                match c
                {
                    'i' => {
                        iter.next();
                        if let Some(c) = iter.peek()
                        {
                            match c
                            {
                                'n' => {
                                    iter.next();
                                    if let Some(c) = iter.peek()
                                    {
                                        match c
                                        {
                                            'e' => {
                                                iter.next();
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
