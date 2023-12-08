use std::io::{
    self,
    Read,
};
use std::iter::{
    Iterator,
    Peekable,
};
use std::str::CharIndices;

fn main()
{
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    use std::time::Instant;
    let start = Instant::now();
    let almanac = parse_almanac(&data);
    let parse_time = start.elapsed();

    let start = Instant::now();
    let min_location = part1(&almanac);
    let part1_time = start.elapsed();

    let start = Instant::now();
    let min_location2 = part2(&almanac);
    let part2_time = start.elapsed();

    println!("Closest seed location: {}", min_location);
    println!("New closest seed location: {}", min_location2);
    println!("==== Elapsed time");
    println!("- Parse time....: {}ns", parse_time.as_nanos());
    println!("- Part 1 time...: {}ns", part1_time.as_nanos());
    println!("- Part 2 time...: {}ns", part2_time.as_nanos());
}

fn part1(almanac: &Almanac) -> u64
{
    let mut min_location = u64::MAX;
    for seed in &almanac.seeds
    {
        let mut location = *seed;
        for map in &almanac.maps
        {
            let mut index = map.sources.len() as isize - 1;
            for (i, &src) in map.sources.iter().enumerate()
            {
                if location < src 
                {
                    index = i as isize - 1;
                    break;
                }
            }

            if index == -1 { continue; }
            let index = index as usize;

            let diff = location - map.sources[index]; 
            if diff >= map.counts[index] { continue; }

            location = map.destinations[index] + diff
        }
        min_location = min_location.min(location);
    }
    min_location
}

fn part2(almanac: &Almanac) -> u64
{
    let mut map = almanac.maps[0].clone();
    {
        map.name = "seed-to-location";
        map.chain_with(&almanac.maps[1..]);
    }

    let mut min_location = u64::MAX;
    for i in (0..almanac.seeds.len()).step_by(2)
    {
        let mut start = almanac.seeds[i];
        let mut count = almanac.seeds[i+1];
        while count > 0
        {
            let mut index = map.sources.len() as isize - 1;
            for (i, &src) in map.sources.iter().enumerate()
            {
                if start < src
                {
                    index = i as isize - 1;
                    break;
                }
            }

            let mut location = start;
            let mut used_count = count;
            if index != -1
            {
                let index = index as usize;
                let diff = location - map.sources[index]; 
                let map_count = map.counts[index];
                if diff < map.counts[index]
                {
                    location = map.destinations[index] + diff;
                    used_count = count.min(map_count - diff);
                }
            }
            min_location = min_location.min(location);

            let next_index = (index+1) as usize;
            if next_index < map.sources.len()
            {
                let map_src = map.sources[next_index];
                used_count = used_count.min(map_src - start);
            }

            start += used_count;
            count -= used_count;
        }
    }
    min_location
}

#[allow(dead_code)]
struct Map<'a>
{
    name: &'a str,
    sources: Vec<u64>,
    destinations: Vec<u64>,
    counts: Vec<u64>,
}

impl<'a> Map<'a>
{
    fn chain_with(&mut self, others: &[Map])
    {
        let mut sources = Vec::with_capacity(self.sources.capacity());
        let mut destinations = Vec::with_capacity(self.destinations.capacity());
        let mut counts = Vec::with_capacity(self.counts.capacity());

        sources.extend(&self.sources);
        destinations.extend(&self.destinations);
        counts.extend(&self.counts);

        for other in others
        {
            // Sort by destinations
            for i in (1..self.destinations.len()).rev()
            {
                let mut sorted = true;
                for j in 0..i
                {
                    if self.destinations[j+1] < self.destinations[j]
                    {
                        self.sources.swap(j, j+1);
                        self.destinations.swap(j, j+1);
                        self.counts.swap(j, j+1);

                        sources.swap(j, j+1);
                        destinations.swap(j, j+1);
                        counts.swap(j, j+1);

                        sorted = false;
                    }
                }
                if sorted { break; }
            }

            let mut self_index = 0;
            let mut other_index = 0;
            let mut insert_index = 0;
            let mut other_src = other.sources[other_index];
            let mut other_dest = other.destinations[other_index];
            let mut other_count = other.counts[other_index];
            loop
            {
                if other_count == 0
                {
                    other_index += 1;
                    if other_index >= other.sources.len()
                    {
                        break;
                    }
                    other_src = other.sources[other_index];
                    other_dest = other.destinations[other_index];
                    other_count = other.counts[other_index];
                }


                while self_index < self.destinations.len()
                    && self.destinations[self_index] < other_src
                {
                    self_index += 1;
                }

                while insert_index < destinations.len()
                    && destinations[insert_index] < other_src
                {
                    insert_index += 1;
                }

                if self_index > 0
                {
                    // Check if other source overlaps previous destination
                    let self_dest = self.destinations[self_index-1];
                    let self_count = self.counts[self_index-1];
                    let self_dest_end = self_dest + self_count;
                    if other_src < self_dest_end
                    {
                        // Split previous destination into two parts
                        let no_overlap_count = other_src - self_dest;
                        counts[insert_index-1] = no_overlap_count;

                        let self_src = self.sources[self_index-1];
                        let overlap_count =
                            other_count.min(self_dest_end - other_src);
                        sources.insert(
                            insert_index, self_src + no_overlap_count
                        );
                        destinations.insert(insert_index, other_dest);
                        counts.insert(insert_index, overlap_count);
                        insert_index += 1;

                        // Additional part if other source ends earlier
                        // than previous destination
                        let other_src_end = other_src + other_count;
                        if other_src_end < self_dest_end
                        {
                            let offset = other_src_end - self_dest;
                            sources.insert(
                                insert_index, self_src + offset
                            );
                            destinations.insert(
                                insert_index, self_dest + offset
                            );
                            counts.insert(
                                insert_index, self_dest_end - other_src_end
                            );
                        }

                        other_src += overlap_count;
                        other_dest += overlap_count;
                        other_count -= overlap_count;

                        continue;
                    }
                }

                if insert_index < destinations.len()
                {
                    // Check if other source overlaps current destination
                    let self_dest = destinations[insert_index];
                    let other_src_end = other_src + other_count;
                    if self_dest < other_src_end
                    {
                        if other_src != self_dest
                        {
                            let no_overlap_count = self_dest - other_src;
                            sources.insert(insert_index, other_src);
                            destinations.insert(insert_index, other_dest);
                            counts.insert(insert_index, no_overlap_count);
                            insert_index += 1;

                            other_src += no_overlap_count;
                            other_dest += no_overlap_count;
                            other_count -= no_overlap_count;
                        }

                        let self_src = sources[insert_index];
                        let self_count = counts[insert_index];
                        let overlap_count =
                            other_count.min(other_src_end - self_dest);
                        destinations[insert_index] = other_dest;
                        counts[insert_index] = overlap_count;
                        insert_index += 1;

                        // Additional part when other source ends earlier
                        // than current destination
                        let self_dest_end = self_dest + self_count;
                        if other_src_end < self_dest_end
                        {
                            let offset = other_src_end - self_dest;
                            sources.insert(insert_index, self_src + offset);
                            destinations.insert(
                                insert_index, self_dest + offset
                            );
                            counts.insert(
                                insert_index, self_dest_end - other_src_end
                            );
                        }

                        other_src += overlap_count;
                        other_dest += overlap_count;
                        other_count -= overlap_count;

                        continue;
                    }
                }

                sources.insert(insert_index, other_src);
                destinations.insert(insert_index, other_dest);
                counts.insert(insert_index, other_count);
                insert_index += 1;

                other_count -= other_count;
            }

            self.sources.clear(); self.sources.extend(&sources);
            self.destinations.clear(); self.destinations.extend(&destinations);
            self.counts.clear(); self.counts.extend(&counts);
        }

        // Sort by sources
        for i in (1..self.sources.len()).rev()
        {
            let mut sorted = true;
            for j in 0..i
            {
                if self.sources[j+1] < self.sources[j]
                {
                    self.sources.swap(j, j+1);
                    self.destinations.swap(j, j+1);
                    self.counts.swap(j, j+1);

                    sorted = false;
                }
            }
            if sorted { break; }
        }

        // Merge any continous adjacent sources that
        // has continous adjacent destinations
        let mut i = 0;
        loop
        {
            let end = self.sources.len()-1;
            if i >= end { break; }
            let src = self.sources[i];
            let dest = self.destinations[i];
            loop
            {
                let count = self.counts[i];
                let next_src = self.sources[i+1];
                let next_dest = self.destinations[i+1];
                if (src + count) != next_src || (dest + count) != next_dest
                {
                    break;
                }

                self.counts[i] += self.counts[i+1];
                self.sources.remove(i+1);
                self.destinations.remove(i+1);
                self.counts.remove(i+1);
            }
            i += 1;
        }
    }
}


impl<'a> Clone for Map<'a>
{
    fn clone(&self) -> Self
    {
        let mut sources = Vec::with_capacity(self.sources.capacity());
        sources.extend(&self.sources);
        let mut destinations = Vec::with_capacity(self.destinations.capacity());
        destinations.extend(&self.destinations);
        let mut counts = Vec::with_capacity(self.counts.capacity());
        counts.extend(&self.counts);

        Map {
            name: self.name,
            sources,
            destinations,
            counts,
        }
    }
}

use std::fmt;
impl<'a> fmt::Display for Map<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} map:", self.name)?;
        for (i, source) in self.sources.iter().enumerate()
        {
            write!(
                f,
                "\n{} {} {}",
                source,
                self.destinations[i],
                self.counts[i]
            )?;
        }

        Ok(())
    }
}

struct Almanac<'a>
{
    seeds: Vec<u64>,
    maps: Vec<Map<'a>>,
}

fn parse_almanac(data: &String) -> Almanac
{
    let mut iter = data.char_indices().peekable();

    expect_word(data, &mut iter, "seeds");
    skip_whitespace(&mut iter);
    expect_char(&mut iter, ':');

    let mut seeds = Vec::with_capacity(25);
    while let Some(number) = scan_number(&mut iter)
    {
        seeds.push(number);
    }

    skip_whitespace(&mut iter);
    expect_newline(&mut iter);

    let mut maps = Vec::with_capacity(10);
    while let Some(map) = parse_map(data, &mut iter)
    {
        maps.push(map);
    }

    Almanac {
        seeds,
        maps,
    }
}

fn parse_map<'a>(
    data: &'a String,
    iter: &mut Peekable<CharIndices<'a>>
) -> Option<Map<'a>>
{
    while iter.next_if(|(_, c)| c.is_ascii_whitespace()).is_some() {}
    iter.peek()?;

    let name = scan_word(data, iter).expect("Expected map name!");
    expect_word(data, iter, "map");

    expect_char(iter, ':');
    skip_whitespace(iter);
    expect_newline(iter);

    let mut sources = Vec::with_capacity(50);
    let mut destinations = Vec::with_capacity(50);
    let mut counts = Vec::with_capacity(50);

    loop
    {
        while iter.next_if(|(_, c)| c.is_ascii_whitespace()).is_some() {}
        match iter.peek()
        {
            Some((_, c)) => if !c.is_ascii_digit() { break; },
            None => break,
        }

        let destination = scan_number(iter).expect("Expected source starting point!");
        let source = scan_number(iter).expect("Expected source starting point!");
        let count = scan_number(iter).expect("Expected source starting point!");

        let mut index = sources.len();
        for (i, v) in sources.iter().enumerate()
        {
            if *v > source
            {
                index = i;
                break;
            }
        }

        sources.insert(index, source);
        destinations.insert(index, destination);
        counts.insert(index, count);

        skip_whitespace(iter);
        expect_newline(iter);
    }

    Some(Map { name, sources, destinations, counts })
}

fn scan_word<'a>(data: &'a String, iter: &mut Peekable<CharIndices>) -> Option<&'a str>
{
    skip_whitespace(iter);
    iter.peek()?;

    let start = iter.next().unwrap().0;
    let end = {
        while iter.next_if(
            |(_, c)| c.is_ascii_alphanumeric() || *c == '-'
        ).is_some() {}
        iter.peek().map_or(data.len(), |v| v.0)
    };

    Some(&data[start..end])
}

fn scan_number(iter: &mut Peekable<CharIndices>) -> Option<u64>
{
    skip_whitespace(iter);
    if let Some((_, c)) = iter.peek()
    {
        if !c.is_ascii_digit() { return None; }
    }
    else { return None; }

    let mut n = 0;
    while let Some((_, c)) = iter.peek()
    {
        if !c.is_ascii_digit() { break; }
        n = n * 10 + c.to_digit(10).unwrap() as u64;
        iter.next();
    }

    Some(n)
}

fn skip_whitespace(iter: &mut Peekable<CharIndices>)
{
    while iter.next_if(
        |(_, c)| matches!(c, ' ' | '\t' | '\r' | '\x0B' | '\x0C')
    ).is_some() {}
}

fn expect_char(iter: &mut Peekable<CharIndices>, c: char)
{
    if let Some((_, n)) = iter.next()
    {
        if n == c { return; }
    }

    panic!("Expected char '{}'", c);
}

fn expect_newline(iter: &mut Peekable<CharIndices>)
{
    if iter.next().map_or('\n', |v| v.1) != '\n'
    {
        panic!("Expected new line!");
    }
}

fn expect_word<'a>(data: &'a String, iter: &mut Peekable<CharIndices<'a>>, token: &str)
{
    let word = scan_word(data, iter).unwrap_or("");
    if word.chars().ne(token.chars())
    {
        panic!("Expected token '{}', but instead got '{}'!", token, word);
    }
}
