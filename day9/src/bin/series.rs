use std::io::{self, Read};
use std::iter::Peekable;
use std::str::Chars;

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    use std::time::Instant;
    let start = Instant::now();
    let mut series = Vec::with_capacity(25);
    let mut sum = 0i32;
    let mut iter = data.chars().peekable();
    while parse_series(&mut iter, &mut series)
    {
        let mut level = 0;
        while level < series.len()
        {
            let end = series.len()-1;
            sum += series[end];

            series[end] = series[end] - series[end-1];
            if series[end] == 0 { break; }

            for i in (level..end-1).rev()
            {
                series[i+1] = series[i+1] - series[i];
            }

            level += 1;
        }
        series.clear();
    }
    let part1_time = start.elapsed();

    let start = Instant::now();
    let mut series = Vec::with_capacity(25);
    let mut sum2 = 0i32;
    let mut iter = data.chars().peekable();
    while parse_series(&mut iter, &mut series)
    {
        let mut level = 0;
        while level < series.len()
        {
            let end = series.len()-1;

            series[end] = series[end] - series[end-1];
            if series[end] == 0 { break; }

            for i in (level..end-1).rev()
            {
                series[i+1] = series[i+1] - series[i];
            }

            level += 1;
        }

        for i in (1..=level).rev()
        {
            series[i-1] -= series[i];
        }
        sum2 += series[0];
        series.clear();
    }
    let part2_time = start.elapsed();

    println!("Sum of all next numbers...: {}", sum);
    println!("Sum of all previous numbers....: {}", sum2);
    println!("==== Elapsed time");
    println!("- Part 1 time...: {}ns", part1_time.as_nanos());
    println!("- Part 2 time...: {}ns", part2_time.as_nanos());
}

fn parse_series(iter: &mut Peekable<Chars>, out: &mut Vec<i32>) -> bool
{
    skip_all_whitespace(iter);
    if iter.peek().is_none() { return false; }

    while let Some(n) = scan_number(iter)
    {
        out.push(n);
    }

    skip_whitespace(iter);
    expect_newline(iter);

    true
}

fn scan_number(iter: &mut Peekable<Chars>) -> Option<i32>
{
    skip_whitespace(iter);

    let mut sign = 1;
    if iter.next_if(|c| matches!(c, '-')).is_some()
    {
        sign = -1;
    }

    if let Some(c) = iter.peek()
    {
        if !c.is_ascii_digit() { return None; }
    }
    else { return None; }

    let mut n = 0i32;
    while let Some(c) = iter.peek()
    {
        if !c.is_ascii_digit() { break; }
        n = n * 10 + c.to_digit(10).unwrap() as i32;
        iter.next();
    }
    Some(sign * n)
}

fn expect_newline(iter: &mut Peekable<Chars>)
{
    if iter.next().unwrap_or('\n') != '\n'
    {
        panic!("Expected new line!");
    }
}

fn skip_whitespace(iter: &mut Peekable<Chars>)
{
    while iter.next_if(
        |c| matches!(c, ' ' | '\t' | '\r' | '\x0B' | '\x0C')
    ).is_some() {}
}

fn skip_all_whitespace(iter: &mut Peekable<Chars>)
{
    while iter.next_if(
        |c| matches!(c, ' ' | '\t' | '\r' | '\n' | '\x0B' | '\x0C')
    ).is_some() {}
}
