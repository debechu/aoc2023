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

    use std::time::Instant;
    let start = Instant::now();
    let maps = parse_map(&data);
    let parse_time = start.elapsed();

    let start = Instant::now();
    let mut num_instructions = 0;
    let mut iter = maps.instructions.chars();
    let mut current_node = maps.node_index.get(0).unwrap();
    loop
    {
        if let Some(instruction) = iter.next()
        {
            let node = maps.nodes[current_node];
            if node.id == 26425
            {
                break;
            }

            match instruction
            {
                'L' => current_node= node.left,
                'R' => current_node= node.right,
                _ => (),
            }
            num_instructions += 1;
        }
        else { iter = maps.instructions.chars(); }
    }
    let part1_time = start.elapsed();

    let start = Instant::now();
    let mut current_indices = maps.starting_points.clone();
    let mut current_instructions = Vec::with_capacity(current_indices.capacity());
    let mut counter = 0u64;
    let mut iter = maps.instructions.chars().peekable();
    loop
    {
        if current_indices.is_empty() { break; }

        if let Some(instruction) = iter.next()
        {
            let mut i = 0;
            while i < current_indices.len()
            {
                let node = maps.nodes[current_indices[i]];
                if node.id & 0x1F == 25
                {
                    current_indices.swap_remove(i);
                    current_instructions.push(counter);
                    continue;
                }

                match instruction
                {
                    'L' => current_indices[i] = node.left,
                    'R' => current_indices[i] = node.right,
                    _ => (),
                }

                i += 1;
            }
            counter += 1;
        }

        if iter.peek().is_none()
        {
            iter = maps.instructions.chars().peekable();
        }
    }

    let mut num_instructions2 = current_instructions[0];
    for &num_instructions in &current_instructions[1..]
    {
        let gcd = gcd(num_instructions2, num_instructions);
        num_instructions2 *= num_instructions / gcd;
    }
    let part2_time = start.elapsed();

    println!("Amount of instructions to navigate: {}", num_instructions);
    println!("Amount of instructions to navigate as ghost: {}", num_instructions2);
    println!("==== Elapsed time");
    println!("- Parse time....: {}ns", parse_time.as_nanos());
    println!("- Part 1 time...: {}ns", part1_time.as_nanos());
    println!("- Part 2 time...: {}ns", part2_time.as_nanos());
}

fn gcd(a: u64, b: u64) -> u64
{
    let mut div = a.max(b);
    let mut rem = a.min(b);

    loop
    {
        let rest = div % rem;
        if rest == 0 { break; }

        div = rem;
        rem = rest;
    }

    rem
}

#[allow(dead_code)]
struct NodeId(u32);

use std::fmt;
impl fmt::Display for NodeId
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(
            f,
            "{}{}{}",
            ((self.0 >> 10) + 0x41) as u8 as char,
            (((self.0 >> 5) & 0x1F) + 0x41) as u8 as char,
            ((self.0 & 0x1F) + 0x41) as u8 as char
        )
    }
}

impl From<u32> for NodeId
{
    fn from(n: u32) -> Self
    {
        NodeId(n)
    }
}

#[derive(Clone, Copy, Debug)]
struct Node
{
    id: u32,
    left: usize,
    right: usize,
}

struct NodeIndex
{
    keys: Vec<Option<u32>>,
    indices: Vec<usize>,
}

impl NodeIndex
{
    fn with_capacity(capacity: usize) -> Self
    {
        Self {
            keys: vec![None; capacity],
            indices: vec![0; capacity],
        }
    }

    fn set(&mut self, key: u32, index: usize)
    {
        let start = key as usize % self.keys.capacity();
        let mut i = start;
        loop
        {
            match self.keys[i]
            {
                Some(k) => if key == k { break; },
                None => {
                    self.keys[i] = Some(key);
                    break;
                },
            }

            i = (i + 1) % self.keys.capacity();
            if i == start
            {
                panic!("Full capacity reached!");
            }
        }
        self.indices[i] = index;
    }

    fn get(&self, key: u32) -> Option<usize>
    {
        let start = key as usize % self.keys.capacity();
        let mut index = start;
        loop
        {
            match self.keys[index]
            {
                Some(k) => if key == k { return Some(self.indices[index]); },
                None => return None,
            }

            index = (index + 1) % self.keys.capacity();
            if index == start
            {
                return None;
            }
        }
    }
}

struct Maps<'a>
{
    instructions: &'a str,
    nodes: Vec<Node>,
    node_index: NodeIndex,
    starting_points: Vec<usize>,
}

fn parse_map(data: &str) -> Maps
{
    let mut iter = data.char_indices().peekable();

    skip_all_whitespace(&mut iter);
    let instructions = {
        let start = iter.peek().map_or(0, |v| v.0);
        while iter.next_if(|(_, c)| c.is_ascii_alphabetic()).is_some() {}
        let end = iter.peek().map_or(data.len(), |v| v.0);
        &data[start..end]
    };

    skip_whitespace(&mut iter);
    expect_newline(&mut iter);

    let mut nodes = Vec::with_capacity(800);
    let mut node_index = NodeIndex::with_capacity(1500);
    let mut starting_points = Vec::with_capacity(25);

    while let Some(node) = parse_node(&mut iter)
    {
        let index = nodes.len();
        nodes.push(node);
        node_index.set(node.id, index);
        if node.id & 0x1F == 0
        {
            starting_points.push(index);
        }
    }

    // Resolve nodes
    for node in nodes.iter_mut()
    {
        node.left = node_index.get(node.left as u32).unwrap();
        node.right = node_index.get(node.right as u32).unwrap();
    }

    Maps {
        instructions,
        nodes,
        node_index,
        starting_points,
    }
}

fn parse_node(iter: &mut Peekable<CharIndices>) -> Option<Node>
{
    skip_all_whitespace(iter);
    if iter.peek().is_none() { return None; }

    let id = scan_node_id(iter);

    skip_whitespace(iter);
    expect_char(iter, '=');
    skip_whitespace(iter);
    expect_char(iter, '(');

    skip_whitespace(iter);
    let left = scan_node_id(iter) as usize;

    skip_whitespace(iter);
    expect_char(iter, ',');

    skip_whitespace(iter);
    let right = scan_node_id(iter) as usize;

    skip_whitespace(iter);
    expect_char(iter, ')');
    skip_whitespace(iter);
    expect_newline(iter);

    Some(Node { id, left, right })
}

fn scan_node_id(iter: &mut Peekable<CharIndices>) -> u32
{
    let mut id = 0;
    while let Some((_, c)) = iter.peek()
    {
        if !matches!(c, 'A'..='Z') { break; }
        id = (id << 5) | (*c as u32 - 'A' as u32);
        iter.next();
    }
    id
}

fn expect_char(iter: &mut Peekable<CharIndices>, c: char)
{
    if !iter.next().is_some_and(|v| v.1 == c)
    {
        panic!("Expected char {}!", c);
    }
}

fn expect_newline(iter: &mut Peekable<CharIndices>)
{
    if iter.next().map_or('\n', |v| v.1) != '\n'
    {
        panic!("Expected new line!");
    }
}

fn skip_whitespace(iter: &mut Peekable<CharIndices>)
{
    while iter.next_if(
        |(_, c)| matches!(c, ' ' | '\t' | '\r' | '\x0B' | '\x0C' )
    ).is_some() {}
}

fn skip_all_whitespace(iter: &mut Peekable<CharIndices>)
{
    while iter.next_if(
        |(_, c)| matches!(c, ' ' | '\t' | '\r' | '\n' | '\x0B' | '\x0C' )
    ).is_some() {}
}
